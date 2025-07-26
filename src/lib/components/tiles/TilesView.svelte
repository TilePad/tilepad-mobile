<script lang="ts">
  import type { TileModel } from "$lib/api/types/tiles";
  import type { FolderModel } from "$lib/api/types/folders";

  import { fly } from "svelte/transition";
  import TileGrid from "$lib/components/tiles/TileGrid.svelte";

  type Props = {
    tiles: TileModel[];
    folder: FolderModel;
    onClick: (tileId: string) => void;
  };

  const { tiles, folder, onClick }: Props = $props();
</script>

<div class="container">
  {#key folder.id}
    <div
      class="grid-wrapper"
      in:fly={{ x: -50, duration: 300, opacity: 0 }}
      out:fly={{ x: 50, duration: 300, opacity: 0 }}
    >
      <TileGrid
        {tiles}
        rows={folder.config.rows}
        columns={folder.config.columns}
        onClickTile={(tile) => {
          onClick(tile.id);
        }}
      />
    </div>
  {/key}
</div>

<style>
  .container {
    height: 100%;
    width: 100%;

    overflow: hidden;
  }

  .grid-wrapper {
    position: absolute;
    display: flex;
    flex: auto;
    flex-flow: column;

    height: 100%;
    width: 100%;

    overflow: hidden;
    padding: 1rem;
  }
</style>
