<script lang="ts">
  import type { TileModel, ConnectionDetails } from "$lib/api/types";

  import { ripple } from "svelte-ripple-action";
  import { getPluginAssetPath } from "$lib/utils/url";

  import TileLabelElm from "./TileLabelElm.svelte";

  type Props = {
    tile: TileModel;
    connection: ConnectionDetails;
    onClick: VoidFunction;
  };

  const { connection, tile, onClick }: Props = $props();

  const config = $derived(tile.config);
</script>

<button class="tile" onclick={onClick} use:ripple>
  {#if config.icon.type === "PluginIcon"}
    <img
      class="tile__icon"
      src={getPluginAssetPath(
        connection,
        config.icon.plugin_id,
        config.icon.icon,
      )}
      alt="Tile Icon"
    />
  {/if}

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

  .tile__icon {
    width: 100%;
  }
</style>
