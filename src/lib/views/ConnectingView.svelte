<script lang="ts">
  import Button from "$lib/components/input/Button.svelte";
  import PulseLoader from "$lib/components/PulseLoader.svelte";
  import { getTilepadSocket } from "$lib/components/WebsocketProvider.svelte";

  const { socket: socketRef, disconnect } = getTilepadSocket();
  const socket = $derived.by(socketRef);
</script>

<div class="layout">
  <div class="modal">
    <PulseLoader />

    {#if socket !== null}
      <p class="host">{socket.details.host}:{socket.details.port}</p>
    {/if}

    <h1>Connecting</h1>

    <Button onclick={disconnect}>Cancel</Button>
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

  .host {
    color: #999;
    margin-top: 1rem;
  }

  .modal {
    display: flex;
    flex-flow: column;
    gap: 0.25rem;
    align-items: center;
  }

  h1 {
    color: #fff;
  }

  p {
    color: #ccc;
    margin-bottom: 0.5rem;
  }
</style>
