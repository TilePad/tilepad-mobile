<script lang="ts">
  import type { Snippet, Component } from "svelte";

  import { fade, scale } from "svelte/transition";
  import { Dialog, type WithoutChild } from "bits-ui";

  import Button from "../input/Button.svelte";

  export type DialogProps = Dialog.RootProps & {
    buttonLabel?: { text?: string; icon?: Component };
    button?: Snippet<[{ props: Record<string, unknown> }]>;

    title?: Snippet;
    description?: Snippet;
    actions?: Snippet;
    triggerProps?: WithoutChild<Dialog.TriggerProps>;
    contentProps?: WithoutChild<Dialog.ContentProps>;
    // ...other component props if you wish to pass them
  };

  let {
    open = $bindable(false),
    children,
    buttonLabel,
    button,
    contentProps,
    triggerProps,
    title,
    description,
    actions,
    ...restProps
  }: DialogProps = $props();
</script>

<Dialog.Root bind:open {...restProps}>
  <Dialog.Trigger {...triggerProps}>
    {#snippet child({ props })}
      {#if button}
        {@render button({ props })}
      {:else if buttonLabel}
        <Button {...props} type="button">
          {#if buttonLabel.icon}
            <buttonLabel.icon />
          {/if}

          {buttonLabel.text}
        </Button>
      {/if}
    {/snippet}
  </Dialog.Trigger>

  {#if open}
    <Dialog.Portal>
      <Dialog.Overlay>
        {#snippet child({ props })}
          <div
            {...props}
            class="overlay"
            transition:fade={{ duration: 150 }}
          ></div>
        {/snippet}
      </Dialog.Overlay>
      <Dialog.Content {...contentProps}>
        {#snippet child({ props })}
          <div {...props} class="content" transition:scale={{}}>
            {#if title}
              <Dialog.Title>
                {#snippet child({ props })}
                  <h3 {...props} class="title">{@render title()}</h3>
                {/snippet}
              </Dialog.Title>
            {/if}

            {#if description}
              <Dialog.Description>
                {#snippet child({ props })}
                  <p {...props} class="description">{@render description()}</p>
                {/snippet}
              </Dialog.Description>
            {/if}

            {@render children?.()}

            {#if actions}
              <div class="actions">
                {@render actions()}
              </div>
            {/if}
          </div>
        {/snippet}
      </Dialog.Content>
    </Dialog.Portal>
  {/if}
</Dialog.Root>

<style>
  .overlay {
    position: fixed;
    left: 0;
    top: 0;
    width: 100vw;
    height: 100vh;
    background-color: #f4f6f8;
    background-color: rgba(0, 0, 0, 0.5);
    backdrop-filter: blur(2px);
    z-index: 49;
  }

  .content {
    display: flex;
    flex-flow: column;

    position: fixed;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%);

    background-color: #18161b;
    border: 1px solid #222;
    border-radius: 0.25rem;

    z-index: 50;
  }

  .title {
    background-color: #222;
    padding: 1rem;
    border-bottom: 1px solid #222;
    color: #999;
    font-size: 1rem;
  }

  .description {
    color: #ccc;
    padding: 1rem;
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    padding: 1rem;
    gap: 1rem;
  }
</style>
