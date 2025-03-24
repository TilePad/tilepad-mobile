<script lang="ts">
  import { createTilepadSocket } from "$lib/api/socket.svelte";
  import TilesView from "$lib/components/tiles/TilesView.svelte";
  import DeviceList from "$lib/components/device/DeviceList.svelte";

  const {
    state: stateRef,
    socket: socketRef,
    connect,
    disconnect,
    clickTile,
  } = createTilepadSocket();

  let state = $derived.by(stateRef);
  let socket = $derived.by(socketRef);
</script>

{#if state.type === "Initial"}
  <DeviceList onConnect={connect} />
{:else if state.type === "Connecting"}
  <p>Connecting...</p>
{:else if state.type === "RequestingApproval"}
  <p>Waiting for approval...</p>
{:else if state.type === "Authenticating"}
  <p>Authenticating...</p>
{:else if state.type === "Authenticated"}
  <button onclick={disconnect}>Disconnect</button>
  <div class="layout">
    <div class="tiles">
      {#if socket != null && state.folder !== null}
        <TilesView
          connection={socket.details}
          tiles={state.tiles}
          folder={state.folder}
          onClick={(tileId) => {
            clickTile(tileId);
          }}
        />
      {/if}
    </div>
  </div>
{:else if state.type === "Declined"}
  <button onclick={disconnect}>Back</button>
{:else if state.type === "Revoked"}
  <button onclick={disconnect}>Back</button>
{/if}

<style>
  .layout {
    height: 100%;
    display: flex;

    flex-flow: column;
  }

  .tiles {
    flex: auto;
    overflow: hidden;
  }
</style>
