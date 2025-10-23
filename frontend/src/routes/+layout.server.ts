import { jwtDecode } from 'jwt-decode'
import type { LayoutServerLoad } from './$types'

export const load: LayoutServerLoad = async (event) => {
  const jwt = event.cookies.get('jwt')
  if (jwt) {
    const decoded = jwtDecode<App.JwtPayload>(jwt)
    event.locals.user = {
      userId: decoded.sub,
      email: decoded.email,
    }
  }
}
