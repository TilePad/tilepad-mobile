<script lang="ts">
  import type { TileModel } from "$lib/api/types/tiles";
  import type { FolderModel } from "$lib/api/types/folders";
  import type { TilepadSocketDetails } from "$lib/api/socket.svelte";

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

<div class="layout">
  <button onclick={disconnect}>Disconnect</button>
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
</style>
