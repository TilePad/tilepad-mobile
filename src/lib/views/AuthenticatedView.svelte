<script lang="ts">
  import type { TileModel } from "$lib/api/types/tiles";
  import type { FolderModel } from "$lib/api/types/folders";
  import type { TilepadSocketDetails } from "$lib/api/socket.svelte";

  import { onMount } from "svelte";
  import { fly } from "svelte/transition";
  import { loadFont } from "$lib/utils/fontLoader";
  import { impactFeedback } from "@tauri-apps/plugin-haptics";
  import TilesView from "$lib/components/tiles/TilesView.svelte";
  import { keepScreenOn } from "tauri-plugin-keep-screen-on-api";
  import { swipe, type SwipeCustomEvent } from "svelte-gestures";
  import ActionDrawer from "$lib/components/ActionDrawer.svelte";
  import ServerProvider from "$lib/components/ServerProvider.svelte";
  import { getTilepadSocket } from "$lib/components/WebsocketProvider.svelte";

  type Props = {
    details: TilepadSocketDetails;
    tiles: TileModel[];
    folder: FolderModel;
  };

  const { details, tiles, folder }: Props = $props();
  const serverURL = $derived(`http://${details.host}:${details.port}`);

  const { clickTile } = getTilepadSocket();

  onMount(() => {
    let dispose = false;
    let loaded = false;

    keepScreenOn(true).then(() => {
      loaded = true;
      if (dispose) {
        keepScreenOn(false);
      }
    });

    return () => {
      dispose = true;
      if (loaded) keepScreenOn(false);
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

  $effect(() => {
    for (const tile of tiles) {
      const label = tile.config.label;

      // Roboto is a built in font and does not require loading
      if (label.font === "Roboto") continue;

      // Request the tile font be loading
      loadFont(serverURL, label.font, label.bold, label.italic);
    }
  });
</script>

<svelte:body use:swipe={() => ({})} onswipe={onSwipe} />

<ServerProvider {serverURL}>
  <ActionDrawer open={drawerOpen} onClose={() => (drawerOpen = false)} />

  <div class="layout" in:fly={{ x: -100, duration: 250 }}>
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
</ServerProvider>

<style>
  .layout {
    height: 100%;
    width: 100%;
    overflow: hidden;
  }
</style>
