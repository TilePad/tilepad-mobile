<script lang="ts">
  import {
    TileIconType,
    type TileIcon,
    type TileIconOptions,
  } from "$lib/api/types/tiles";
  import {
    getIconAssetPath,
    getPluginAssetPath,
    getUploadedIconAssetPath,
  } from "$lib/utils/url";

  import { getTilepadConnection } from "../WebsocketProvider.svelte";

  type Props = {
    icon: TileIcon;
    iconOptions: TileIconOptions;
  };

  const { icon, iconOptions }: Props = $props();

  const connection = $derived.by(getTilepadConnection());
  const style = $derived(
    `padding: calc(${iconOptions.padding}px * var(--tile-size-adjustment)); background-color: ${iconOptions.background_color}`,
  );

  let error = $state(false);

  function onError(event: Event) {
    error = true;
  }
</script>

{#if icon.type === TileIconType.PluginIcon}
  <img
    class="tile__icon"
    src={getPluginAssetPath(connection, icon.plugin_id, icon.icon)}
    alt="Tile Icon"
    class:tile__icon--error={error}
    onerror={onError}
    {style}
  />
{:else if icon.type === TileIconType.IconPack}
  <img
    class="tile__icon"
    src={getIconAssetPath(connection, icon.pack_id, icon.path)}
    alt="Tile Icon"
    class:tile__icon--error={error}
    onerror={onError}
    {style}
  />
{:else if icon.type === TileIconType.Uploaded}
  <img
    class="tile__icon"
    src={getUploadedIconAssetPath(connection, icon.path)}
    alt="Tile Icon"
    class:tile__icon--error={error}
    onerror={onError}
    {style}
  />
{/if}

<style>
  .tile__icon {
    width: 100%;
    height: 100%;
    object-fit: contain;
  }

  .tile__icon--error {
    display: none;
  }
</style>
