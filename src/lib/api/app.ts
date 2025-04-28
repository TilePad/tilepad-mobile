import { invoke } from "@tauri-apps/api/core";

export function getLicenses() {
  return invoke<string>("app_get_licenses");
}
