import { invoke } from "@tauri-apps/api/core";
import { createQuery } from "@tanstack/svelte-query";

import type { DeviceModel, CreateDevice, UpdateDevice } from "./types/devices";

import { queryClient } from "./client";

const devicesKeys = {
  root: ["devices"],
  requests: ["devices", "requests"],
  devices: ["devices", "devices"],
  connectedDevices: ["devices", "connected"],
};

// [REQUESTS] ------------------------------------------------------

export function getDevices() {
  return invoke<DeviceModel[]>("devices_get_devices");
}

export async function createDevice(create: CreateDevice) {
  const model = await invoke<DeviceModel>("devices_create_device", { create });
  invalidateDevices();
  return model;
}

export function updateDevice(deviceId: string, update: UpdateDevice) {
  return invoke<DeviceModel>("devices_update_device", { deviceId, update });
}

export async function removeDevice(deviceId: string) {
  await invoke("devices_remove_device", { deviceId });
  invalidateDevices();
}

// [QUERIES] ------------------------------------------------------

export function createDevicesQuery() {
  return createQuery({
    queryKey: devicesKeys.devices,
    queryFn: getDevices,
  });
}

// [MUTATORS] ------------------------------------------------------

export function invalidateDevices() {
  queryClient.invalidateQueries({
    queryKey: devicesKeys.devices,
    exact: false,
  });
}
