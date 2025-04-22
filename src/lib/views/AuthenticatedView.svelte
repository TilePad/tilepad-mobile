<script lang="ts">
  import type { TileModel } from "$lib/api/types/tiles";
  import type { FolderModel } from "$lib/api/types/folders";

  import { onMount } from "svelte";
  import { fly } from "svelte/transition";
  import { impactFeedback } from "@tauri-apps/plugin-haptics";
  import TilesView from "$lib/components/tiles/TilesView.svelte";
  import { keepScreenOn } from "tauri-plugin-keep-screen-on-api";
  import { swipe, type SwipeCustomEvent } from "svelte-gestures";
  import ActionDrawer from "$lib/components/ActionDrawer.svelte";
  import { getTilepadSocket } from "$lib/components/WebsocketProvider.svelte";

  type Props = {
    tiles: TileModel[];
    folder: FolderModel;
  };

  const { tiles, folder }: Props = $props();

  const { clickTile } = getTilepadSocket();

  onMount(() => {
    keepScreenOn(true);

    return () => {
      keepScreenOn(false);
    };
  });

  let drawerOpen = $state(false);

  function onSwipe(event: SwipeCustomEvent) {
    const direction = event.detail.direction;
    if (direction === "right") {
      drawerOpen = true;
    } else if (direction === "left") {
      drawerOpen = false;
    }
  }
</script>

<ActionDrawer open={drawerOpen} onClose={() => (drawerOpen = false)} />

<svelte:body use:swipe={() => ({})} onswipe={onSwipe} />

<div class="layout" in:fly={{ x: -100, duration: 250 }}>
  <div class="tiles">
    <TilesView
      {tiles}
      {folder}
      onClick={(tileId) => {
        try {
          impactFeedback("medium");
        } catch (_err) {
          //
        }
        clickTile(tileId);
      }}
    />
  </div>
</div>

<style>
  .layout {
    height: 100%;
    width: 100%;
  }

  .tiles {
    overflow: hidden;
    height: 100%;
  }
</style>
