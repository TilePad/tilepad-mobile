import type { Uuid } from "./shared";
import type { PluginId } from "./plugin";
import type { ActionId } from "./actions";
import type { FolderId } from "./folders";

export type TileId = Uuid;

export interface TileModel {
  id: TileId;
  config: TileConfig;
  folder_id: FolderId;
  row: number;
  column: number;
}

export interface TileConfig {
  plugin_id: PluginId;
  action_id: ActionId;
  icon: TileIcon;
  icon_options: TileIconOptions;
  properties: object;
  label: TileLabel;
}

export interface TileIconOptions {
  padding: number;
  background_color: string;
}

export type TileIconNone = object;
export type TileIconPluginIcon = {
  plugin_id: PluginId;
  icon: string;
};
export type TileIconIconPack = {
  pack_id: string;
  path: string;
};
export type TileIconUploaded = {
  path: string;
};

export enum TileIconType {
  None = "None",
  PluginIcon = "PluginIcon",
  IconPack = "IconPack",
  Uploaded = "Uploaded",
}

export type TileIcon =
  | ({ type: TileIconType.None } & TileIconNone)
  | ({ type: TileIconType.PluginIcon } & TileIconPluginIcon)
  | ({ type: TileIconType.IconPack } & TileIconIconPack)
  | ({ type: TileIconType.Uploaded } & TileIconUploaded);

export type CreateTile = Omit<TileModel, "id">;

export type UpdateTile = Partial<Omit<TileModel, "id">>;

export interface TileLabel {
  enabled: boolean;
  label: string;
  align: LabelAlign;
  font: string;
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
