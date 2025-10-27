<script lang="ts">
  import { goto } from '$app/navigation'
  import { Button } from '$lib/components/ui/button'

  const { data } = $props()
</script>

<header
  class="flex justify-between items-center p-4 sticky top-0 bg-background"
>
  <div>
    <h1>Rest OS</h1>
  </div>
  <div>
    {#if data.loggedIn}
      <Button onclick={() => goto('/dashboard')}>Dashboard</Button>
    {:else}
      <Button onclick={() => goto('/login')}>Log in</Button>
    {/if}
  </div>
</header>
