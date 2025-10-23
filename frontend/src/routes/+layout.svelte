<script lang="ts">
  import '../app.css'
  import favicon from '$lib/assets/favicon.svg'
  import {
    Client,
    setContextClient,
    cacheExchange,
    fetchExchange,
  } from '@urql/svelte'
  import { Toaster } from '$lib/components/ui/sonner'
  import { ModeWatcher } from 'mode-watcher'

  let { children } = $props()

  const client = new Client({
    url: '/api/graphql',
    exchanges: [cacheExchange, fetchExchange],
  })

  setContextClient(client)
</script>

<svelte:head>
  <link rel="icon" href={favicon} />
</svelte:head>

<Toaster position="bottom-center" richColors />
<ModeWatcher />

{@render children?.()}
