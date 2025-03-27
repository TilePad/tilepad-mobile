<script lang="ts">
  import { getTilepadSocket } from "$lib/components/WebsocketProvider.svelte";

  const { disconnect } = getTilepadSocket();
</script>

<div class="layout">
  <h1>Revoked</h1>
  <p>This device has been revoked</p>
  <button onclick={disconnect}>Back</button>
</div>

<style>
  .layout {
    height: 100%;
    display: flex;

    flex-flow: column;
  }
</style>
