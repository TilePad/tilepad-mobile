import { EventEmitter } from "$lib/utils/eventEmitter";

import type { TileModel } from "./types/tiles";
import type { FolderModel } from "./types/folders";
import type { DisplayContext } from "./types/plugin";
import type {
  ClientDeviceMessage,
  ServerDeviceMessage,
} from "./types/protocol";

import { setDeviceAccessToken } from "./devices";

type SocketState =
  //  Initial disconnected state
  | { type: "Initial" }
  // Connecting to the socket
  | { type: "Connecting" }
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
  accessToken: string | null;
}

type TilepadSocketEvents = {
  recv_from_plugin: (ctx: DisplayContext, message: object) => void;
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
            sendMessage({
              type: "TileClicked",
              tile_id: tileId,
            });
          };

          sendDisplayMessage = (ctx, message) => {
            sendMessage({
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

    socket.onopen = () => {
      // Connection opened
      if (details.accessToken) {
        state = { type: "Authenticating" };
        sendMessage({
          type: "Authenticate",
          access_token: details.accessToken,
        });
      } else {
        requestApproval();
      }
    };

    socket.onmessage = (event) => {
      const msg: ServerDeviceMessage = JSON.parse(event.data);
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
      socket.send(JSON.stringify(msg));
    };

    const requestApproval = async () => {
      state = { type: "RequestingApproval" };

      // Request the device name
      const name = getDeviceName();

      sendMessage({ type: "RequestApproval", name });

      // Clear current access token
      setDeviceAccessToken(details.deviceId, null);
    };

    function onMessage(msg: ServerDeviceMessage) {
      switch (msg.type) {
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
          const token = msg.access_token;

          // Update current device access token
          setDeviceAccessToken(details.deviceId, token);
          setState({ type: "Authenticating" });

          // Authenticate with the current token
          sendMessage({ type: "Authenticate", access_token: token });
          break;
        }

        case "Authenticated": {
          setState({
            type: "Authenticated",
            deviceId: msg.device_id,
            tiles: [],
            folder: null,
          });
          sendMessage({ type: "RequestTiles" });
          break;
        }

        case "InvalidAccessToken": {
          requestApproval();
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
