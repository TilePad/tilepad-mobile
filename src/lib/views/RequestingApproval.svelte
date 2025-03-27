<script lang="ts">
  import { hostname } from "@tauri-apps/plugin-os";
  import Button from "$lib/components/input/Button.svelte";
  import { getTilepadSocket } from "$lib/components/WebsocketProvider.svelte";

  const { disconnect } = getTilepadSocket();
</script>

<div class="layout">
  <div class="modal">
    <h1>Waiting for approval</h1>
    <p>
      Please approve this device

      {#await hostname() then hostname}
        <span class="device-name">{hostname}</span>
      {/await}
    </p>

    <Button onclick={disconnect}>Back</Button>
  </div>
</div>

<style>
  .layout {
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;

    flex-flow: column;
  }

  .modal {
    display: flex;
    flex-flow: column;
    gap: 0.5rem;
  }

  h1 {
    color: #fff;
  }

  p {
    color: #ccc;
    margin-bottom: 1rem;
  }

  .device-name {
    color: #fcfcfc;
    font-weight: bold;
  }
</style>
