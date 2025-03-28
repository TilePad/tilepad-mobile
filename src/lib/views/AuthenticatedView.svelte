<script lang="ts">
  import type { TileModel } from "$lib/api/types/tiles";
  import type { FolderModel } from "$lib/api/types/folders";
  import type { TilepadSocketDetails } from "$lib/api/socket.svelte";

  import { fly } from "svelte/transition";
  import Button from "$lib/components/input/Button.svelte";
  import TilesView from "$lib/components/tiles/TilesView.svelte";
  import { getTilepadSocket } from "$lib/components/WebsocketProvider.svelte";

  type Props = {
    details: TilepadSocketDetails;
    tiles: TileModel[];
    folder: FolderModel | null;
  };

  const { details, tiles, folder }: Props = $props();

  const { disconnect, clickTile } = getTilepadSocket();
</script>

<div class="layout" in:fly={{ x: -100, duration: 250 }}>
  <div class="actions">
    <Button onclick={disconnect}>Disconnect</Button>
  </div>

  <div class="tiles">
    <TilesView
      connection={details}
      {tiles}
      {folder}
      onClick={(tileId) => {
        clickTile(tileId);
      }}
    />
  </div>
</div>

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

  .actions {
    width: 100%;
    display: flex;
    gap: 1rem;
    width: 100%;
    background-color: #322e38;
    padding: 0.5rem;
    align-items: center;
  }
</style>
