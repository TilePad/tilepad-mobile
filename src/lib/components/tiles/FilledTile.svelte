<script lang="ts">
  import type { TileModel } from "$lib/api/types/tiles";
  import type { TilepadSocketDetails } from "$lib/api/socket.svelte";

  import { ripple } from "svelte-ripple-action";

  import TileIcon from "./TileIcon.svelte";
  import TileLabelElm from "./TileLabelElm.svelte";

  type Props = {
    tile: TileModel;
    connection: TilepadSocketDetails;
    onClick: VoidFunction;
  };

  const { connection, tile, onClick }: Props = $props();

  const config = $derived(tile.config);
</script>

<button class="tile" onclick={onClick} use:ripple>
  <TileIcon {connection} icon={tile.config.icon} />
  <TileLabelElm label={config.label} />
</button>

<style>
  .tile {
    position: relative;
    background-color: #242129;
    border: 2px solid #715c8f;
    border-radius: 5px;
    display: flex;
    justify-content: center;
    align-items: center;
    font-weight: bold;
    text-align: center;
    cursor: pointer;
    width: 100%;
    height: 100%;
    color: #ccc;
    font-size: 1.5rem;
    user-select: none;
    overflow: hidden;
    transition: 0.15s ease transform;
  }

  .tile:active {
    transform: scale(0.95);
  }
</style>
