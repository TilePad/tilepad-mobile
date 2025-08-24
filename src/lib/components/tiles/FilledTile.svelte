<script lang="ts">
  import type { TileModel } from "$lib/api/types/tiles";

  import { tap, type TapCustomEvent } from "svelte-gestures";
  import { type DisplayContext } from "$lib/api/types/plugin";
  import { serverContext } from "$lib/contexts/server.context";

  import TileIcon from "./TileIcon.svelte";
  import TileLabelElm from "./TileLabel.svelte";
  import TileContainer from "./TileContainer.svelte";
  import TileIndicator from "./TileIndicator.svelte";

  type Props = {
    tile: TileModel;
    tileSize: number;
    gap: number;
    onClick: VoidFunction;
  };

  const { tile, tileSize, gap, onClick }: Props = $props();

  const currentServerContext = serverContext.get();

  const config = $derived(tile.config);
  const displayCtx: DisplayContext = $derived({
    device_id: currentServerContext.deviceId,
    tile_id: tile.id,
    action_id: tile.action_id,
    plugin_id: tile.plugin_id,
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

<TileContainer position={tile.position} {tileSize} {gap}>
  <button
    use:tap={() => ({ timeframe: 1000 })}
    ontap={onTap}
    style="--tile-border-color: {config.icon_options.border_color}"
    class="tile"
    aria-roledescription="button"
    data-drop-zone="filledTile"
    data-row={tile.position.row}
    data-column={tile.position.column}
  >
    <TileIcon
      ctx={displayCtx}
      icon={config.icon}
      iconOptions={config.icon_options}
    />
    <TileLabelElm {...config.label} />
    <TileIndicator tile_id={tile.id} />
  </button>
</TileContainer>

<style>
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
    transition: all 0.1s ease;
  }

  /* Disable pointer events for children to make dragging work properly */
  .tile > :global(*) {
    pointer-events: none;
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
