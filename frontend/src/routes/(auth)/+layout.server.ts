import { redirect } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = (event) => {
  const { user } = event.locals
  if (!user) {
    redirect(301, '/login')
  }

  return {
    user: event.locals.user!
  }
}