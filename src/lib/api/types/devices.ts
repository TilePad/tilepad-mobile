import type { Uuid } from "./shared";

export type DeviceId = Uuid;

export type DeviceRequestId = Uuid;

export type DeviceSessionId = Uuid;

export type DeviceModel = {
  id: string;
  name: string;
  host: string;
  port: number;
  access_token: string | null;
  order: number;
  created_at: string;
};

export type CreateDevice = {
  name: string;
  host: string;
  port: number;
  access_token: string | null;
  order: number;
};

export type UpdateDevice = {
  name?: string;
  host?: string;
  port?: number;
  access_token?: string | null;
};
