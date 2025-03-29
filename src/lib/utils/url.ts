import type { TilepadSocketDetails } from "$lib/api/socket.svelte";

export function getPluginAssetPath(
  connection: TilepadSocketDetails,
  pluginId: string,
  path: string,
) {
  return new URL(
    `/plugins/${pluginId}/assets/${path}`,
    `http://${connection.host}:${connection.port}`,
  ).toString();
}

export function getIconAssetPath(
  connection: TilepadSocketDetails,
  packId: string,
  path: string,
) {
  return new URL(
    `/icons/${packId}/assets/${path}`,
    `http://${connection.host}:${connection.port}`,
  ).toString();
}
