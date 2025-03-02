import type { ConnectionDetails } from "$lib/api/types";

export function getPluginAssetPath(
  connection: ConnectionDetails,
  pluginId: string,
  path: string,
) {
  return new URL(
    `/plugins/${pluginId}/assets/${path}`,
    `http://${connection.host}:${connection.port}`,
  ).toString();
}
