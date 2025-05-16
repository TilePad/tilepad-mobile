<script lang="ts">
  import type { TileModel } from "$lib/api/types/tiles";
  import type { DisplayContext } from "$lib/api/types/plugin";

  import { tap, type TapCustomEvent } from "svelte-gestures";

  import TileIcon from "./TileIcon.svelte";
  import TileLabelElm from "./TileLabel.svelte";
  import { DESIRED_TILE_WIDTH } from "./TileGrid.svelte";
  import { getServerContext } from "../ServerProvider.svelte";

  type Props = {
    tile: TileModel;
    tileSize: number;
    gap: number;
    onClick: VoidFunction;
  };

  const { tile, tileSize, gap, onClick }: Props = $props();
  const serverContext = getServerContext();

  const config = $derived(tile.config);
  const displayCtx: DisplayContext = $derived({
    device_id: serverContext.deviceId,
    tile_id: tile.id,
    action_id: tile.action_id,
    plugin_id: tile.plugin_id,
  });

  const { tileX, tileY, tileZ, tileWidth, tileHeight, sizeAdjust } =
    $derived.by(() => {
      const position = tile.position;
      const tileWidth =
        tileSize * position.column_span + gap * (position.column_span - 1);
      const tileHeight =
        tileSize * position.row_span + gap * (position.row_span - 1);

      const ratioX = (tileWidth - DESIRED_TILE_WIDTH) / DESIRED_TILE_WIDTH;
      const ratioY = (tileHeight - DESIRED_TILE_WIDTH) / DESIRED_TILE_WIDTH;

      const sizeAdjustX = 1 + ratioX;
      const sizeAdjustY = 1 + ratioY;
      const sizeAdjust = Math.min(sizeAdjustX, sizeAdjustY);

      const tileX = tileSize * position.column + gap * position.column;
      const tileY = tileSize * position.row + gap * position.row;
      const tileZ =
        (position.row + position.row_span) *
        (position.column + position.column_span);

      return {
        tileX,
        tileY,
        tileZ,
        tileWidth,
        tileHeight,
        sizeAdjust,
      };
    });

  function onTap(event: TapCustomEvent) {
    onClick();

    const button = event.currentTarget;
    if (!(button instanceof HTMLButtonElement)) return;

    const circle = document.createElement("span");
    const diameter = Math.max(button.clientWidth, button.clientHeight);
    const radius = diameter / 2;

    circle.style.width = circle.style.height = `${diameter}px`;
    circle.style.left = `${event.detail.x - radius}px`;
    circle.style.top = `${event.detail.y - radius}px`;
    circle.classList.add("ripple");

    // Remove old ripple if it exists
    const oldRipple = button.querySelector(".ripple");
    if (oldRipple) {
      oldRipple.remove();
    }

    button.appendChild(circle);

    // Remove after animation ends
    circle.addEventListener("animationend", () => {
      circle.remove();
    });
  }
</script>

<div
  class="tile-container"
  style="--tile-size-adjustment: {sizeAdjust}; --tile-width: {tileWidth}px; --tile-height: {tileHeight}px; --tile-x: {tileX}px; --tile-y: {tileY}px; --tile-z: {tileZ}"
>
  <button
    style="--tile-border-color: {config.icon_options.border_color}"
    class="tile"
    use:tap={() => ({ timeframe: 1000 })}
    ontap={onTap}
  >
    <TileIcon
      ctx={displayCtx}
      icon={tile.config.icon}
      iconOptions={tile.config.icon_options}
    />
    <TileLabelElm label={config.label} />
  </button>
</div>

<style>
  .tile-container {
    position: absolute;
    top: 0;
    left: 0;

    transform: translate(var(--tile-x), var(--tile-y));
    width: var(--tile-width);
    height: var(--tile-height);
    z-index: calc(var(--tile-z));
  }

  .tile {
    position: relative;
    border-radius: 5px;
    display: flex;
    justify-content: center;
    align-items: center;
    width: var(--tile-width);
    height: var(--tile-height);

    color: #ccc;

    border: 2px solid var(--tile-border-color);
    cursor: pointer;

    background-color: #151318;

    user-select: none;
    overflow: hidden;
  }

  :global(.ripple) {
    position: absolute;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.5);
    transform: scale(0);
    animation: ripple-animation 600ms linear;
    pointer-events: none;
  }

  @keyframes ripple-animation {
    to {
      transform: scale(4);
      opacity: 0;
    }
  }
</style>
