// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
  namespace App {
    // interface Error {}
    interface Locals {
      user?: User
    }
    // interface PageData {}
    // interface PageState {}
    // interface Platform {}

    interface User {
      userId: string
      email: string
    }

    interface JwtPayload {
      sub: string
      iat: number
      exp: number
      email: string
    }
  }
}

export {}
