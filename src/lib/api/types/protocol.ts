import type { TileModel } from "./tiles";
import type { FolderModel } from "./folders";

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
