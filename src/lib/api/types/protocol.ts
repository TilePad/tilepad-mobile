import type { TileModel } from "./tiles";
import type { FolderModel } from "./folders";
import type { DisplayContext } from "./plugin";

export type ClientDeviceMessage =
  | { type: "RequestApproval"; name: string }
  | { type: "RequestTiles" }
  | { type: "Authenticate"; access_token: string }
  | { type: "TileClicked"; tile_id: string }
  | { type: "RecvFromDisplay"; ctx: DisplayContext; message: object };

export type ServerDeviceMessage =
  | { type: "Declined" }
  | { type: "Approved"; device_id: string; access_token: string }
  | { type: "Revoked" }
  | { type: "Authenticated"; device_id: string }
  | { type: "InvalidAccessToken" }
  | { type: "Tiles"; tiles: TileModel[]; folder: FolderModel }
  | { type: "RecvFromPlugin"; ctx: DisplayContext; message: object };
