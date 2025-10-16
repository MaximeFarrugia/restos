mod db;
pub mod jwt;
mod mutation;
mod query;

use crate::{mutation::MutationRoot, query::QueryRoot};
use anyhow::Context;
use async_graphql::{EmptySubscription, Schema, http::GraphiQLSource};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    Router,
    extract::State,
    http::HeaderMap,
    response::{Html, IntoResponse},
    routing::get,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use surrealdb::{
    Surreal,
    engine::remote::ws::{Client, Ws},
    opt::auth::Database,
};
use tokio::sync::mpsc;

#[derive(Clone)]
struct AppState {
    pub db: Surreal<Client>,
    pub schema: Schema<QueryRoot, MutationRoot, EmptySubscription>,
}

async fn get_db_client() -> anyhow::Result<Surreal<Client>> {
    let address = format!(
        "{}:{}",
        dotenvy::var("DB_HOST").context("Get DB_HOST from env")?,
        dotenvy::var("DB_PORT").context("Get DB_PORT from env")?
    );
    let db = Surreal::new::<Ws>(address)
        .await
        .context("Create SurrealDB Client")?;

    db.signin(Database {
        namespace: dotenvy::var("DB_NAMESPACE")
            .context("Get DB_NAMESPACE from env")?
            .as_str(),
        database: dotenvy::var("DB_DATABASE")
            .context("Get DB_DATABASE from env")?
            .as_str(),
        username: dotenvy::var("DB_USER")
            .context("Get DB_USER from env")?
            .as_str(),
        password: dotenvy::var("DB_PASSWORD")
            .context("Get DB_PASSWORD from env")?
            .as_str(),
    })
    .await
    .context("Sign in to database")?;

    Ok(db)
}

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

async fn graphql_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    req: GraphQLRequest,
) -> impl IntoResponse {
    let (tx, mut rx) = mpsc::channel::<Cookie>(8);

    let mut req = req.into_inner();
    req = req.data(tx);

    let response = state.schema.execute(req).await;

    let mut updated_jar = jar;
    while let Some(cookie) = rx.recv().await {
        updated_jar = updated_jar.add(cookie);
    }

    (updated_jar, async_graphql_axum::GraphQLResponse::from(response))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let db = get_db_client().await?;
    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(db.clone())
    .finish();

    let app = Router::new()
        .route("/graphql", get(graphiql).post(graphql_handler))
        .with_state(AppState { db, schema });

    let address = format!(
        "{}:{}",
        dotenvy::var("BACKEND_HOST").context("Get BACKEND_HOST from env")?,
        dotenvy::var("BACKEND_PORT").context("Get BACKEND_PORT from env")?
    );
    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .context("Failed to bind TCP listener")?;

    println!("Listening on {address}");
    axum::serve(listener, app)
        .await
        .context("Failed to start server")?;

    Ok(())
}
