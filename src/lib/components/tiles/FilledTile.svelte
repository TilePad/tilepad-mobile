<script lang="ts">
  import type { TileModel } from "$lib/api/types/tiles";

  import { tap, type TapCustomEvent } from "svelte-gestures";

  import TileIcon from "./TileIcon.svelte";
  import TileLabelElm from "./TileLabelElm.svelte";

  type Props = {
    tile: TileModel;
    onClick: VoidFunction;
  };

  const { tile, onClick }: Props = $props();

  const config = $derived(tile.config);

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

<button
  style="--tile-border-color: {config.icon_options.border_color}"
  class="tile"
  use:tap={() => ({ timeframe: 1000 })}
  ontap={onTap}
>
  <TileIcon icon={tile.config.icon} iconOptions={tile.config.icon_options} />
  <TileLabelElm label={config.label} />
</button>

<style>
  .tile {
    position: relative;
    background-color: #151318;
    border: 2px solid var(--tile-border-color);
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
