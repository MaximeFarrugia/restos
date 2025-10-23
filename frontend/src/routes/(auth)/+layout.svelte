<script lang="ts">
  import * as Sidebar from '$lib/components/ui/sidebar'
  import {
    BlocksIcon,
    ChefHatIcon,
    ChevronUpIcon,
    PanelLeftCloseIcon,
    PanelLeftOpenIcon,
    UserIcon,
  } from 'lucide-svelte'
  import type { LayoutProps } from './$types'
  import * as DropdownMenu from '$lib/components/ui/dropdown-menu'
  import { Button } from '$lib/components/ui/button'

  let { children, data }: LayoutProps = $props()

  let open = $state(true)

  let username = $derived(data.user.email.split('@')[0])

  const items = [
    {
      title: 'Stock',
      url: '/stock',
      icon: BlocksIcon,
    },
  ]
</script>

<Sidebar.Provider bind:open>
  <Sidebar.Root collapsible="icon">
    <Sidebar.Header>
      <Sidebar.Menu>
        <Sidebar.MenuItem>
          <Sidebar.MenuButton size="lg">
            {#snippet child({ props })}
              <a href="/dashboard" {...props}>
                <div
                  class="bg-sidebar-primary text-sidebar-primary-foreground flex aspect-square size-8 items-center justify-center rounded-lg"
                >
                  <ChefHatIcon />
                </div>
                <span>Rest OS</span>
              </a>
            {/snippet}
          </Sidebar.MenuButton>
        </Sidebar.MenuItem>
      </Sidebar.Menu>
    </Sidebar.Header>
    <Sidebar.Content>
      <Sidebar.Group>
        <Sidebar.GroupContent>
          <Sidebar.Menu>
            {#each items as item}
              <Sidebar.MenuItem>
                <Sidebar.MenuButton>
                  {#snippet child({ props })}
                    <a href={item.url} {...props}>
                      <item.icon />
                      <span>{item.title}</span>
                    </a>
                  {/snippet}
                </Sidebar.MenuButton>
              </Sidebar.MenuItem>
            {/each}
          </Sidebar.Menu>
        </Sidebar.GroupContent>
      </Sidebar.Group>
    </Sidebar.Content>
    <Sidebar.Footer>
      <Sidebar.Menu>
        <Sidebar.MenuItem>
          <DropdownMenu.Root>
            <DropdownMenu.Trigger>
              {#snippet child({ props })}
                <Sidebar.MenuButton
                  {...props}
                  class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
                >
                  <UserIcon />
                  <span>{username}</span>
                  <ChevronUpIcon class="ml-auto" />
                </Sidebar.MenuButton>
              {/snippet}
            </DropdownMenu.Trigger>
            <DropdownMenu.Content
              side="top"
              class="w-(--bits-dropdown-menu-anchor-width)"
            >
              <DropdownMenu.Item variant="destructive">
                <span>Sign out</span>
              </DropdownMenu.Item>
            </DropdownMenu.Content>
          </DropdownMenu.Root>
        </Sidebar.MenuItem>
      </Sidebar.Menu>
    </Sidebar.Footer>
  </Sidebar.Root>

  <main class="p-4">
    <Button onclick={() => (open = !open)} size="sm" variant="ghost" class="mb-4">
      {#if open}
        <PanelLeftCloseIcon />
      {:else}
        <PanelLeftOpenIcon />
      {/if}
      <span>{open ? 'Close' : 'Open'} Sidebar</span>
    </Button>
    {@render children()}
  </main>
</Sidebar.Provider>
