import type { LayoutServerLoad } from './$types'
import { requireLogin } from '$lib/server/auth'

export const load: LayoutServerLoad = (event) => {
  const user = requireLogin()

  return {
    user,
  }
}
