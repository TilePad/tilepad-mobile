<script lang="ts">
  import type { TileModel } from "$lib/api/types/tiles";
  import type { FolderModel } from "$lib/api/types/folders";

  import TileGrid from "$lib/components/tiles/TileGrid.svelte";

  import EmptyTile from "./EmptyTile.svelte";
  import FilledTile from "./FilledTile.svelte";

  type Props = {
    tiles: TileModel[];
    folder: FolderModel;
    onClick: (tileId: string) => void;
  };

  const { tiles, folder, onClick }: Props = $props();

  function getTile(tiles: TileModel[], row: number, column: number) {
    return tiles.find((tile) => tile.row === row && tile.column === column);
  }
</script>

<div class="grid-wrapper">
  <TileGrid rows={folder.config.rows} columns={folder.config.columns}>
    {#snippet tile(row, column)}
      {@const tile = getTile(tiles, row, column) ?? null}
      {#if tile !== null}
        <FilledTile {tile} onClick={() => onClick(tile.id)} />
      {:else}
        <EmptyTile />
      {/if}
    {/snippet}
  </TileGrid>
</div>

<style>
  .grid-wrapper {
    display: flex;
    flex: auto;
    flex-flow: column;

    height: 100%;

    overflow: hidden;
    padding: 1rem;
  }
</style>
