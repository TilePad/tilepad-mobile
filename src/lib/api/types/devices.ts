import type { Uuid } from "./shared";

export type DeviceId = Uuid;

export type DeviceRequestId = Uuid;

export type DeviceSessionId = Uuid;

export type DeviceModel = {
  id: string;
  name: string;
  host: string;
  port: number;
  client_private_key: number[];
  server_public_key: number[] | null;
  order: number;
  created_at: string;
};

export type CreateDevice = {
  name: string;
  host: string;
  port: number;
  client_private_key: Uint8Array;
  order: number;
};
