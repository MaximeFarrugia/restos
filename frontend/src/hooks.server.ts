import type { Handle } from "@sveltejs/kit";
import { jwtDecode } from "jwt-decode";

export const handle: Handle = async ({ event, resolve }) => {
  const jwt = event.cookies.get('jwt')
  if (jwt) {
    const decoded = jwtDecode<App.JwtPayload>(jwt)
    event.locals.user = {
      userId: decoded.sub,
      email: decoded.email,
    }
  } else {
    event.locals.user = undefined
  }

  const response = await resolve(event)
  return response
}