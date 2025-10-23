<script lang="ts">
  import * as InputGroup from '$lib/components/ui/input-group'
  import { Eye, EyeOff } from 'lucide-svelte'
  import type { ComponentProps } from 'svelte'

  let visible = $state(false)
  let props: ComponentProps<typeof InputGroup.Input> = $props()
</script>

<InputGroup.Root>
  <InputGroup.Input
    type={visible ? 'text' : 'password'}
    placeholder="**********"
    {...props}
  />
  <InputGroup.Addon align="inline-end">
    <InputGroup.Button onclick={() => (visible = !visible)}>
      {#if visible}
        <Eye />
      {:else}
        <EyeOff />
      {/if}
    </InputGroup.Button>
  </InputGroup.Addon>
</InputGroup.Root>
