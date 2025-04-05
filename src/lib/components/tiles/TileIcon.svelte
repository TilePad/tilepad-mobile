<script lang="ts">
  import type { TilepadSocketDetails } from "$lib/api/socket.svelte";

  import { TileIconType, type TileIcon } from "$lib/api/types/tiles";
  import {
    getIconAssetPath,
    getPluginAssetPath,
    getUploadedIconAssetPath,
  } from "$lib/utils/url";

  type Props = {
    icon: TileIcon;
    connection: TilepadSocketDetails;
  };

  const { icon, connection }: Props = $props();
</script>

{#if icon.type === TileIconType.PluginIcon}
  <img
    class="tile__icon"
    src={getPluginAssetPath(connection, icon.plugin_id, icon.icon)}
    alt="Tile Icon"
  />
{:else if icon.type === TileIconType.IconPack}
  <img
    class="tile__icon"
    src={getIconAssetPath(connection, icon.pack_id, icon.path)}
    alt="Tile Icon"
  />
{:else if icon.type === TileIconType.Uploaded}
  <img
    class="tile__icon"
    src={getUploadedIconAssetPath(connection, icon.path)}
    alt="Tile Icon"
  />
{/if}

<style>
  .tile__icon {
    width: 100%;
    height: 100%;
    object-fit: contain;
  }
</style>
