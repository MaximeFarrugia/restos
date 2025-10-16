use anyhow::Context;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha512;
use std::collections::BTreeMap;

pub const JWT_COOKIE_NAME: &'static str = "jwt";

pub struct Jwt {
    pub sub: String,
    pub iat: chrono::DateTime<chrono::Utc>,
    pub exp: chrono::DateTime<chrono::Utc>,
}

impl Jwt {
    pub fn sign(&self) -> anyhow::Result<String> {
        let secret = dotenvy::var("JWT_SECRET").context("Get JWT_SECRET from env")?;
        let key: Hmac<Sha512> =
            Hmac::new_from_slice(secret.as_bytes()).context("Failed to create JWT key")?;

        let mut claims = BTreeMap::new();
        claims.insert("sub", self.sub.clone());
        claims.insert("iat", self.iat.to_string());
        claims.insert("exp", self.exp.to_string());

        let token_str = claims.sign_with_key(&key).context("Failed to sign JWT")?;
        Ok(token_str)
    }
}
