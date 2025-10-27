import { redirect } from '@sveltejs/kit'
import { getRequestEvent } from '$app/server'

export function requireLogin() {
  const { locals, url } = getRequestEvent()

  if (!locals.user) {
    const redirectTo = url.pathname + url.search
    const params = new URLSearchParams({ redirectTo })

    redirect(307, `/login?${params}`)
  }

  return locals.user
}

export function requireLogout() {
  let loggedIn = false
  try {
    requireLogin()
    loggedIn = true
  } catch (err) {
    loggedIn = false
  }

  if (loggedIn) {
    redirect(307, '/dashboard')
  }
}
