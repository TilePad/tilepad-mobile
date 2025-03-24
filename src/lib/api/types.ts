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

export type ClientDeviceMessage =
  | { type: "RequestApproval"; name: string }
  | { type: "RequestTiles" }
  | { type: "Authenticate"; access_token: string }
  | { type: "TileClicked"; tile_id: string };

export type ServerDeviceMessage =
  | { type: "Declined" }
  | { type: "Approved"; device_id: string; access_token: string }
  | { type: "Revoked" }
  | { type: "Authenticated" }
  | { type: "InvalidAccessToken" }
  | { type: "Tiles"; tiles: TileModel[]; folder: FolderModel };

export type ConnectionDetails = {
  deviceId: string;
  host: string;
  port: number;
  accessToken: string | null;
};

export interface TileModel {
  id: string;
  config: TileConfig;
  folder_id: string;
  row: number;
  column: number;
}

export interface TileConfig {
  plugin_id: string;
  action_id: string;
  icon: TileIcon;
  properties: object;
  label: TileLabel;
}

export type TileIconNone = object;
export type TileIconPluginIcon = {
  plugin_id: string;
  icon: string;
};

export enum TileIconType {
  None = "None",
  PluginIcon = "PluginIcon",
}

export type TileIcon =
  | ({ type: TileIconType.None } & TileIconNone)
  | ({ type: TileIconType.PluginIcon } & TileIconPluginIcon);

export interface FolderModel {
  id: string;
  name: string;
  config: FolderConfig;
  profile_id: string;
  default: boolean;
  order: number;
}

export interface FolderConfig {
  rows: number;
  columns: number;
}

export interface TileLabel {
  enabled: boolean;
  label: string;
  align: LabelAlign;
  font_size: number;
  bold: boolean;
  italic: boolean;
  underline: boolean;
  color: string;
}

export enum LabelAlign {
  Bottom = "Bottom",
  Middle = "Middle",
  Top = "Top",
}

export interface EncodedInterfaces {
  addr: string[];
  port: number;
}
