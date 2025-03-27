<script lang="ts">
  import { getTilepadSocket } from "$lib/components/WebsocketProvider.svelte";

  const { disconnect } = getTilepadSocket();
</script>

<div class="layout">
  <h1>Declined</h1>
  <p>Device was declined access to tilepad</p>
  <button onclick={disconnect}>Back</button>
</div>

<style>
  .layout {
    height: 100%;
    display: flex;

    flex-flow: column;
  }
</style>
