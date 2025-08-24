<script lang="ts">
  import type { TileId } from "$lib/api/types/tiles";

  import { scale } from "svelte/transition";
  import { DeviceIndicator } from "$lib/api/types/protocol";
  import SolarCheckCircleBoldDuotone from "~icons/solar/check-circle-bold-duotone";
  import SolarCloseCircleBoldDuotone from "~icons/solar/close-circle-bold-duotone";
  import SolarDangerTriangleBoldDuotone from "~icons/solar/danger-triangle-bold-duotone";

  import LoadingSpinner from "../loading/LoadingSpinner.svelte";
  import { getTilepadSocket } from "../WebsocketProvider.svelte";

  type Props = {
    tile_id: TileId;
  };

  const { tile_id: target_tile_id }: Props = $props();
  const { events } = getTilepadSocket();

  type IndicatorState = {
    indicator: DeviceIndicator;
    timeout: number;
  };

  let currentState: IndicatorState | null = $state(null);

  const onIndicator = (
    tile_id: string,
    indicator: DeviceIndicator,
    duration: number,
  ) => {
    if (tile_id !== target_tile_id) {
      return;
    }

    if (currentState !== null) {
      clearTimeout(currentState.timeout);
    }

    // Don't show anything for the None indicator, just clear the state
    if (indicator === DeviceIndicator.None) {
      currentState = null;
      return;
    }

    const timeout = setTimeout(() => {
      currentState = null;
    }, duration);

    currentState = { indicator, timeout };
  };

  $effect(() => {
    events.on("indicator", onIndicator);
    return () => {
      events.off("indicator", onIndicator);
    };
  });
</script>

{#if currentState !== null}
  <div
    class="indicator"
    data-indicator={currentState.indicator}
    in:scale={{ duration: 100 }}
    out:scale={{ duration: 200 }}
  >
    {#if currentState.indicator === DeviceIndicator.Error}
      <SolarCloseCircleBoldDuotone />
    {:else if currentState.indicator === DeviceIndicator.Success}
      <SolarCheckCircleBoldDuotone />
    {:else if currentState.indicator === DeviceIndicator.Warning}
      <SolarDangerTriangleBoldDuotone />
    {:else if currentState.indicator === DeviceIndicator.Loading}
      <div class="spinner">
        <LoadingSpinner size={24} />
      </div>
    {/if}
  </div>
{/if}

<style>
  .indicator {
    position: absolute;
    top: 0;
    right: 0;
    font-size: calc(24px * var(--tile-size-adjustment));
    background-color: rgba(0, 0, 0, 0.5);
    padding: calc(2px * var(--tile-size-adjustment));
    border-radius: calc(4px * var(--tile-size-adjustment));
  }

  .spinner {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
  }

  .indicator[data-indicator="Error"] {
    color: #db5858;
  }

  .indicator[data-indicator="Success"] {
    color: #74e358;
  }

  .indicator[data-indicator="Loading"] {
    color: #74e358;
  }

  .indicator[data-indicator="Warning"] {
    color: #e0a800;
  }
</style>
