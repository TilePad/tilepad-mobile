import { x25519 } from "@noble/curves/ed25519.js";
import { equalBytes } from "@noble/curves/utils.js";
import { decode, encode } from "@msgpack/msgpack";
import { EventEmitter } from "$lib/utils/eventEmitter";

import type { TileModel } from "./types/tiles";
import type { FolderModel } from "./types/folders";
import type { DisplayContext } from "./types/plugin";
import type {
  DeviceIndicator,
  ClientDeviceMessage,
  ServerDeviceMessage,
  ClientDeviceMessageEncrypted,
  ServerDeviceMessageEncrypted,
} from "./types/protocol";

import { setDeviceServerPublicKey } from "./devices";
import {
  decryptMessage,
  decryptPayload,
  encryptMessage,
  encryptPayload,
} from "./encryption";

type SocketState =
  //  Initial disconnected state
  | { type: "Initial" }
  // Connecting to the socket
  | { type: "Connecting" }
  // User must confirm the changed server key
  | {
      type: "ConfirmChangedServerKey";
      serverPublicKey: number[];
      lastServerPublicKey: number[] | null;

      onConfirm: VoidFunction;
      onCancel: VoidFunction;
    }
  // Requesting approval from the server
  | { type: "RequestingApproval" }
  // Authenticating with known token
  | { type: "Authenticating" }
  // Authenticated and ready
  | {
      type: "Authenticated";
      deviceId: string;
      tiles: TileModel[];
      folder: FolderModel | null;
    }
  // Request for approval was declined
  | { type: "Declined" }
  // Server revoked the current access
  | { type: "Revoked" }
  // Failed to connect
  | { type: "ConnectionFailed" }
  // Connection lost
  | { type: "ConnectionLost" };

// Details about a tilepad socket we are connecting to
export interface TilepadSocketDetails {
  deviceId: string;
  host: string;
  port: number;
  clientPrivateKey: number[];
  serverPublicKey: number[] | null;
}

type TilepadSocketEvents = {
  recv_from_plugin: (ctx: DisplayContext, message: object) => void;
  indicator: (
    tile_id: string,
    indicator: DeviceIndicator,
    duration: number,
  ) => void;
};

export type TilepadSocket = {
  events: EventEmitter<TilepadSocketEvents>;
  state: () => SocketState;
  details: () => TilepadSocketDetails | null;
  connect: (details: TilepadSocketDetails) => void;
  disconnect: () => void;
  reconnect: () => void;
  clickTile: (tileId: string) => void;
  sendDisplayMessage: (ctx: DisplayContext, message: object) => void;
};

type DisconnectFunction = VoidFunction;
type ClickTileFunction = (tileId: string) => void;
type SendDisplayMessageFunction = (
  ctx: DisplayContext,
  message: object,
) => void;

export function createTilepadSocket(
  getDeviceName: () => string,
): TilepadSocket {
  const events = new EventEmitter<TilepadSocketEvents>();
  let detailsState: TilepadSocketDetails | null = $state(null);
  let state: SocketState = $state({ type: "Initial" });

  let onDisconnect: DisconnectFunction | undefined;
  let onClickTile: ClickTileFunction | undefined;
  let sendDisplayMessage: SendDisplayMessageFunction | undefined;

  let sharedSecretState: Uint8Array | null = null;

  const disconnect = () => {
    state = { type: "Initial" };

    // Call current disconnect callback
    if (onDisconnect) {
      onDisconnect();
      onDisconnect = undefined;
    }
  };
  const reconnect = () => {
    if (detailsState !== null) {
      connect(detailsState);
    }
  };

  const connect = (details: TilepadSocketDetails) => {
    // Disconnect if already connected
    disconnect();

    function setState(currentState: SocketState) {
      state = currentState;

      switch (state.type) {
        // When authenticated setup the tile click handler
        case "Authenticated": {
          onClickTile = (tileId) => {
            sendEncryptedMessage({
              type: "TileClicked",
              tile_id: tileId,
            });
          };

          sendDisplayMessage = (ctx, message) => {
            sendEncryptedMessage({
              type: "RecvFromDisplay",
              ctx,
              message,
            });
          };

          break;
        }

        // Clear tile click handler
        default: {
          onClickTile = undefined;
          break;
        }
      }
    }

    setState({ type: "Connecting" });

    detailsState = details;

    // Establish connection
    const socket = new WebSocket(
      `ws://${details.host}:${details.port}/devices/ws`,
    );

    socket.binaryType = "arraybuffer";

    socket.onopen = () => {
      // Connection opened
      setState({ type: "Authenticating" });

      const clientPublicKey = x25519.getPublicKey(
        new Uint8Array(details.clientPrivateKey),
      );

      sendMessage({
        type: "InitiateHandshake",
        name: getDeviceName(),
        public_key: new Array(...clientPublicKey),
      });
    };

    socket.onmessage = (event) => {
      const msg = decode(event.data) as ServerDeviceMessage;
      onMessage(msg);
    };

    socket.onclose = () => {
      if (state.type === "Connecting") {
        setState({ type: "ConnectionFailed" });
      } else if (state.type !== "Initial") {
        setState({ type: "ConnectionLost" });
      }
    };

    // Setup disconnect handler
    onDisconnect = () => {
      setState({ type: "Initial" });
      socket.close();
    };

    const sendMessage = (msg: ClientDeviceMessage) => {
      socket.send(encode(msg));
    };

    const sendEncryptedMessage = (msg: ClientDeviceMessageEncrypted) => {
      // Ensure we are in the correct state to perform encryption
      if (sharedSecretState === null) {
        disconnect();
        return;
      }

      // Encrypt it with our key
      const { message, nonce } = encryptPayload(msg, sharedSecretState);

      sendMessage({
        type: "Encrypted",
        message: Array.from(message),
        nonce: Array.from(nonce),
      });
    };

    function onEncryptedMessage(msg: ServerDeviceMessageEncrypted) {
      switch (msg.type) {
        // Pending approval from the server
        case "ApprovalRequested": {
          setState({ type: "RequestingApproval" });
          break;
        }

        // Server declined approval
        case "Declined": {
          setState({ type: "Declined" });
          break;
        }

        // Server revoked access
        case "Revoked": {
          setState({ type: "Revoked" });
          break;
        }

        // Server approved access
        case "Approved": {
          setState({
            type: "Authenticated",
            deviceId: msg.device_id,
            tiles: [],
            folder: null,
          });
          sendEncryptedMessage({ type: "RequestTiles" });
          break;
        }

        case "Authenticated": {
          setState({
            type: "Authenticated",
            deviceId: msg.device_id,
            tiles: [],
            folder: null,
          });
          sendEncryptedMessage({ type: "RequestTiles" });
          break;
        }

        case "Tiles": {
          if (state.type === "Authenticated") {
            setState({ ...state, tiles: msg.tiles, folder: msg.folder });
          }

          break;
        }

        case "RecvFromPlugin": {
          events.emit("recv_from_plugin", msg.ctx, msg.message);
          break;
        }

        case "DisplayIndicator": {
          events.emit("indicator", msg.tile_id, msg.indicator, msg.duration);
          break;
        }
      }
    }

    function completeChallenge(
      serverPublicKey: Uint8Array,
      clientPrivateKey: Uint8Array,
      encryptedChallenge: Uint8Array,
      serverNonce: Uint8Array,
    ) {
      const sharedSecret = x25519.getSharedSecret(
        clientPrivateKey,
        serverPublicKey,
      );

      // Decrypt the server challenge
      const decryptedChallenge = decryptMessage(
        sharedSecret,
        encryptedChallenge,
        serverNonce,
      );

      // Encrypt it with our cipher
      const clientEncryptedChallenge = encryptMessage(
        sharedSecret,
        decryptedChallenge,
      );

      sharedSecretState = sharedSecret;

      // Send the completed challenge to the server
      sendMessage({
        type: "AuthenticateChallengeResponse",
        challenge: Array.from(clientEncryptedChallenge.message),
        nonce: Array.from(clientEncryptedChallenge.nonce),
      });
    }

    function onMessage(msg: ServerDeviceMessage) {
      switch (msg.type) {
        case "AuthenticateChallenge": {
          const { server_public_key, challenge, nonce } = msg;

          const clientPrivateKey = new Uint8Array(details.clientPrivateKey);

          const serverPublicKey = new Uint8Array(server_public_key);
          const encryptedChallenge = new Uint8Array(challenge);
          const serverNonce = new Uint8Array(nonce);

          const onConfirm = () => {
            // Store the confirmed public key
            setDeviceServerPublicKey(details.deviceId, server_public_key)
              //
              .catch(console.error);

            // Complete the challenge and continue
            completeChallenge(
              serverPublicKey,
              clientPrivateKey,
              encryptedChallenge,
              serverNonce,
            );
          };

          const onCancel = () => {
            disconnect();
          };

          const lastServerPublicKey = details.serverPublicKey
            ? new Uint8Array(details.serverPublicKey)
            : null;

          if (
            lastServerPublicKey !== null &&
            !equalBytes(serverPublicKey, lastServerPublicKey)
          ) {
            setState({
              type: "ConfirmChangedServerKey",
              serverPublicKey: msg.server_public_key,
              lastServerPublicKey: details.serverPublicKey,
              onConfirm,
              onCancel,
            });
            return;
          } else {
            // TOFU (Trust On First Use) or server is still trusted
            onConfirm();
          }

          break;
        }
        case "EncryptedMessage": {
          const { message, nonce } = msg;

          // Ensure we are in the correct state to perform decryption
          if (sharedSecretState === null) {
            disconnect();
            return;
          }

          const payload = decryptPayload<ServerDeviceMessageEncrypted>(
            sharedSecretState,
            new Uint8Array(message),
            new Uint8Array(nonce),
          );

          onEncryptedMessage(payload);
          break;
        }
        case "Error": {
          console.error(msg.message);
          disconnect();
          break;
        }
      }
    }
  };

  return {
    events,
    state() {
      return state;
    },
    details() {
      return detailsState;
    },
    connect,
    disconnect,
    reconnect,

    clickTile: (tileId: string) => {
      if (onClickTile) onClickTile(tileId);
    },
    sendDisplayMessage: (ctx: DisplayContext, message: object) => {
      if (sendDisplayMessage) sendDisplayMessage(ctx, message);
    },
  };
}
