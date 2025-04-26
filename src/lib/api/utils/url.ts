export function getPluginAssetPath(
  serverURL: string,
  pluginId: string,
  path: string,
) {
  return new URL(`/plugins/${pluginId}/assets/${path}`, serverURL).toString();
}

export function getIconAssetPath(
  serverURL: string,
  packId: string,
  path: string,
) {
  return new URL(`/icons/${packId}/assets/${path}`, serverURL).toString();
}

export function getUploadedIconAssetPath(serverURL: string, path: string) {
  return new URL(`/uploaded-icons/${path}`, serverURL).toString();
}
