import type { TileModel } from "./tiles";
import type { FolderModel } from "./folders";
import type { DisplayContext } from "./plugin";

export type ClientDeviceMessage =
  | {
      type: "InitiateHandshake";
      name: string;
      public_key: number[];
    }
  | {
      type: "AuthenticateChallengeResponse";
      challenge: number[];
      nonce: number[];
    }
  | {
      type: "Encrypted";
      message: number[];
      nonce: number[];
    }
  | { type: "Error"; message: string };

export type ServerDeviceMessage =
  | {
      type: "AuthenticateChallenge";
      server_public_key: number[];
      challenge: number[];
      nonce: number[];
    }
  | {
      type: "EncryptedMessage";

      message: number[];
      nonce: number[];
    }
  | { type: "Error"; message: string };

export type ClientDeviceMessageEncrypted =
  | { type: "RequestTiles" }
  | { type: "TileClicked"; tile_id: string }
  | { type: "RecvFromDisplay"; ctx: DisplayContext; message: object };

export type ServerDeviceMessageEncrypted =
  | { type: "ApprovalRequested" }
  | { type: "Declined" }
  | { type: "Approved"; device_id: string }
  | { type: "Revoked" }
  | { type: "Authenticated"; device_id: string }
  | { type: "Tiles"; tiles: TileModel[]; folder: FolderModel }
  | { type: "RecvFromPlugin"; ctx: DisplayContext; message: object };
