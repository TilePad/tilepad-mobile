<script lang="ts">
  import { t } from "svelte-i18n";
  import { Dialog } from "bits-ui";
  import { fade, slide } from "svelte/transition";

  import Button from "./input/Button.svelte";
  import { getTilepadSocket } from "./WebsocketProvider.svelte";

  type Props = {
    open: boolean;
    onClose: VoidFunction;
  };

  const { open, onClose }: Props = $props();

  const { disconnect } = getTilepadSocket();
</script>

<Dialog.Root
  {open}
  onOpenChange={(value) => {
    if (!value) onClose();
  }}
>
  <Dialog.Portal>
    <Dialog.Overlay forceMount>
      {#snippet child({ props, open })}
        {#if open}
          <div
            {...props}
            class="overlay"
            transition:fade={{ duration: 150 }}
          ></div>
        {/if}
      {/snippet}
    </Dialog.Overlay>
    <Dialog.Content trapFocus={false} forceMount>
      {#snippet child({ props, open })}
        {#if open}
          <div
            {...props}
            class="content"
            transition:slide={{ axis: "x", duration: 150 }}
          >
            <div class="content-inner">
              <img src="/tilepad-logo.svg" alt="Tilepad Logo" class="logo" />
              <Button onclick={disconnect}>{$t("disconnect")}</Button>
            </div>
          </div>
        {/if}
      {/snippet}
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>

<style>
  .logo {
    max-width: 10rem;
    margin-bottom: 1.5rem;
  }

  .overlay {
    position: fixed;
    left: 0;
    top: 0;
    width: 100vw;
    height: 100vh;
    background-color: #f4f6f8;
    background-color: rgba(0, 0, 0, 0.5);
    z-index: 19;
  }

  .content {
    position: fixed;
    left: 0;
    top: 0;
    width: 20rem;
    height: 100%;

    overflow: hidden;

    background-color: #18161b;
    border: 1px solid #222;
    border-radius: 0.25rem;

    z-index: 20;
  }

  .content-inner {
    display: flex;
    flex-flow: column;

    overflow: hidden;

    width: 20rem;
    height: 100%;
    padding: 1rem;
  }
</style>
