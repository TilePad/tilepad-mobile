<script lang="ts">
  import type { TileModel } from "$lib/api/types/tiles";
  import type { FolderModel } from "$lib/api/types/folders";
  import type { TilepadSocketDetails } from "$lib/api/socket.svelte";

  import { onMount } from "svelte";
  import { fly } from "svelte/transition";
  import { loadFont } from "$lib/utils/fontLoader";
  import { impactFeedback } from "@tauri-apps/plugin-haptics";
  import { serverContext } from "$lib/contexts/server.context";
  import TilesView from "$lib/components/tiles/TilesView.svelte";
  import { keepScreenOn } from "tauri-plugin-keep-screen-on-api";
  import ActionDrawer from "$lib/components/ActionDrawer.svelte";
  import { useSwipe, type SwipeCustomEvent } from "svelte-gestures";
  import { getTilepadSocket } from "$lib/components/WebsocketProvider.svelte";

  type Props = {
    details: TilepadSocketDetails;
    tiles: TileModel[];
    folder: FolderModel;
    deviceId: string;
  };

  const { details, tiles, folder, deviceId }: Props = $props();
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

  serverContext.set({
    get serverURL() {
      return serverURL;
    },
    get deviceId() {
      return deviceId;
    },
  });

  const { swipe, onswipe, onswipedown, onswipemove, onswipeup } = useSwipe(
    onSwipe,
    () => ({}),
    {},
    true,
  );
</script>

{/* @ts-expect-error https://github.com/Rezi/svelte-gestures/issues/38#issuecomment-3315953573 */ null}
<svelte:body
  {@attach swipe}
  {onswipe}
  {onswipedown}
  {onswipemove}
  {onswipeup}
/>

<ActionDrawer open={drawerOpen} onClose={() => (drawerOpen = false)} />

<div class="layout" in:fly={{ x: -100, duration: 250 }}>
  <TilesView
    {tiles}
    {folder}
    onClick={(tileId) => {
      try {
        impactFeedback("medium");
      } catch {
        //
      }
      clickTile(tileId);
    }}
  />
</div>

<style>
  .layout {
    height: 100%;
    width: 100%;
    overflow: hidden;
  }
</style>
