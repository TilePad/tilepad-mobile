<script lang="ts">
  import Button from "$lib/components/input/Button.svelte";
  import { getTilepadSocket } from "$lib/components/WebsocketProvider.svelte";

  const { reconnect, disconnect } = getTilepadSocket();
</script>

<div class="layout">
  <div class="modal">
    <h1>Connection Lost</h1>
    <p>Lost connection</p>
    <div class="actions">
      <Button onclick={disconnect}>Back</Button>
      <Button onclick={reconnect}>Reconnect</Button>
    </div>
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
    align-items: center;
  }

  h1 {
    color: #fff;
  }

  p {
    color: #ccc;
    margin-bottom: 0.5rem;
  }

  .actions {
    display: flex;
    flex-flow: row;
    gap: 1rem;
  }
</style>
