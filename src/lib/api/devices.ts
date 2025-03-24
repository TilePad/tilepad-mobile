import { invoke } from "@tauri-apps/api/core";

import type { DeviceModel, CreateDevice, UpdateDevice } from "./types";

export function getDevices() {
  return invoke<DeviceModel[]>("devices_get_devices");
}

export function createDevice(create: CreateDevice) {
  return invoke<DeviceModel>("devices_create_device", { create });
}

export function updateDevice(deviceId: string, update: UpdateDevice) {
  return invoke<DeviceModel>("devices_update_device", { deviceId, update });
}
