<script lang="ts">
  import type { TileModel, FolderModel } from "$lib/api/types";
  import type { TilepadSocketDetails } from "$lib/api/socket.svelte";

  import TileGrid from "$lib/components/tiles/TileGrid.svelte";

  import EmptyTile from "./EmptyTile.svelte";
  import FilledTile from "./FilledTile.svelte";

  type Props = {
    tiles: TileModel[];
    folder: FolderModel;
    connection: TilepadSocketDetails;
    onClick: (tileId: string) => void;
  };

  const { tiles, folder, connection, onClick }: Props = $props();

  function getTile(tiles: TileModel[], row: number, column: number) {
    return tiles.find((tile) => tile.row === row && tile.column === column);
  }
</script>

<div class="grid-wrapper">
  <TileGrid rows={folder.config.rows} columns={folder.config.columns}>
    {#snippet tile(row, column)}
      {@const tile = getTile(tiles, row, column) ?? null}
      {#if tile !== null}
        <FilledTile {connection} {tile} onClick={() => onClick(tile.id)} />
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
