<script lang="ts">
  import type { TileModel } from "$lib/api/types/tiles";
  import type { FolderModel } from "$lib/api/types/folders";

  import TileGrid from "$lib/components/tiles/TileGrid.svelte";

  type Props = {
    tiles: TileModel[];
    folder: FolderModel;
    onClick: (tileId: string) => void;
  };

  const { tiles, folder, onClick }: Props = $props();
</script>

<div class="grid-wrapper">
  <TileGrid
    {tiles}
    rows={folder.config.rows}
    columns={folder.config.columns}
    onClickTile={(tile) => {
      onClick(tile.id);
    }}
  />
</div>

<style>
  .grid-wrapper {
    display: flex;
    flex: auto;
    flex-flow: column;

    height: 100%;
    width: 100%;

    overflow: hidden;
    padding: 1rem;
  }
</style>
