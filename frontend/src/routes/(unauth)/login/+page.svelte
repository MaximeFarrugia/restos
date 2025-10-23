<script lang="ts">
  import { goto } from '$app/navigation'
  import FieldInfo from '$lib/components/field-info.svelte'
  import PasswordInput from '$lib/components/password-input.svelte'
  import { Button } from '$lib/components/ui/button'
  import {
    Card,
    CardContent,
    CardFooter,
    CardHeader,
    CardTitle,
  } from '$lib/components/ui/card'
  import * as Field from '$lib/components/ui/field'
  import { Input } from '$lib/components/ui/input'
  import { graphql } from '$lib/graphql'
  import type { LoginInput, LoginMutationMutation } from '$lib/graphql/graphql'
  import { createForm } from '@tanstack/svelte-form'
  import {
    getContextClient,
    mutationStore,
    type OperationResultState,
  } from '@urql/svelte'
  import { toast } from 'svelte-sonner'
  import z from 'zod'

  const client = getContextClient()
  let result:
    | OperationResultState<LoginMutationMutation, { input: LoginInput }>
    | undefined = $state()

  const form = createForm(() => ({
    defaultValues: {
      email: '',
      password: '',
    },
    validators: {
      onChange: z.object({
        email: z.email(),
        password: z.string().nonempty(),
      }),
    },
    onSubmit: async ({ value }) => {
      login(value).subscribe((r) => {
        result = r
        if (r.error) {
          toast.error(r.error.message)
        } else if (r.data?.login.success) {
          goto('/dashboard')
        }
      })
    },
  }))

  const login = ({ email, password }: LoginInput) =>
    mutationStore({
      client,
      query: graphql(`
        mutation LoginMutation($input: LoginInput!) {
          login(input: $input) {
            success
          }
        }
      `),
      variables: {
        input: {
          email,
          password,
        },
      },
    })
</script>

<div class="flex flex-col justify-center items-center h-screen gap-6">
  <form
    class="w-full max-w-sm"
    onsubmit={(e) => {
      e.preventDefault()
      form.handleSubmit()
    }}
  >
    <Card>
      <CardHeader>
        <CardTitle>Login to your account</CardTitle>
      </CardHeader>
      <CardContent>
        <div class="flex flex-col gap-6">
          <form.Field name="email">
            {#snippet children(field)}
              <Field.Field>
                <Field.Label for={field.name}>Email</Field.Label>
                <Input
                  id={field.name}
                  placeholder="john.doe@email.com"
                  value={field.state.value}
                  onblur={field.handleBlur}
                  oninput={(e) => field.handleChange(e.currentTarget.value)}
                  aria-invalid={field.state.meta.errors.length > 0}
                />
                <FieldInfo {field} />
              </Field.Field>
            {/snippet}
          </form.Field>
          <form.Field name="password">
            {#snippet children(field)}
              <Field.Field>
                <Field.Label for={field.name}>Password</Field.Label>
                <PasswordInput
                  id={field.name}
                  value={field.state.value}
                  onblur={field.handleBlur}
                  oninput={(e) => field.handleChange(e.currentTarget.value)}
                  aria-invalid={field.state.meta.errors.length > 0}
                />
                <FieldInfo {field} />
              </Field.Field>
            {/snippet}
          </form.Field>
        </div>
      </CardContent>
      <CardFooter>
        <Button class="w-full" type="submit" disabled={result?.fetching}>
          Log in
        </Button>
      </CardFooter>
    </Card>
  </form>
</div>
