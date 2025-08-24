<script lang="ts">
  import TileContainer from "./TileContainer.svelte";

  type Props = {
    row: number;
    column: number;
    tileSize: number;
    gap: number;
  };

  const { row, column, tileSize, gap }: Props = $props();
</script>

<TileContainer
  position={{ row, column, row_span: 1, column_span: 1 }}
  {tileSize}
  {gap}
  empty
>
  <div
    class="tile"
    data-drop-zone="emptyTile"
    data-row={row}
    data-column={column}
  ></div>
</TileContainer>

<style>
  .tile {
    position: relative;
    background-color: #242129;
    border: 2px solid #36313d;
    border-radius: 5px;
    display: flex;
    justify-content: center;
    align-items: center;
    width: var(--tile-width);
    height: var(--tile-height);
    color: #ccc;

    user-select: none;
    overflow: hidden;

    font-size: 1.5rem;
    text-align: center;
    z-index: 0;
  }
</style>
