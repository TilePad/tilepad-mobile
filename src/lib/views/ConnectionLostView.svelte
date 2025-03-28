<script lang="ts">
  import Button from "$lib/components/input/Button.svelte";
  import { getTilepadSocket } from "$lib/components/WebsocketProvider.svelte";

  const { disconnect } = getTilepadSocket();
</script>

<div class="layout">
  <div class="modal">
    <h1>Connection Lost</h1>
    <p>Lost connection</p>
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
