<script lang="ts">
  import type { AnyFieldApi } from '@tanstack/svelte-form'
  import { Error } from './ui/field'

  let { field }: { field: AnyFieldApi } = $props()
</script>

{#if field.state.meta.errors.length > 0}
  <div class="flex flex-col gap-1">
    {#each field.state.meta.errors as error}
      <Error>{error.message}</Error>
    {/each}
  </div>
{/if}
