import type { ProfileId } from "./profiles";
import type { Uuid, DeepPartial } from "./shared";

export type FolderId = Uuid;

export interface FolderModel {
  id: FolderId;
  name: string;
  config: FolderConfig;
  profile_id: ProfileId;
  default: boolean;
  order: number;
}

export interface FolderConfig {
  rows: number;
  columns: number;
}

export type CreateFolder = Omit<FolderModel, "id" | "config"> & {
  config: Partial<FolderConfig>;
};

export type UpdateFolder = DeepPartial<Omit<FolderModel, "id">>;
