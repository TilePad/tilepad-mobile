import { hostname } from "@tauri-apps/plugin-os";

import type { TileModel } from "./types/tiles";
import type { FolderModel } from "./types/folders";
import type {
  ClientDeviceMessage,
  ServerDeviceMessage,
} from "./types/protocol";

import { updateDevice } from "./devices";

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

export type TilepadSocket = {
  state: () => SocketState;
  details: () => TilepadSocketDetails | null;
  connect: (details: TilepadSocketDetails) => void;
  disconnect: () => void;
  clickTile: (tileId: string) => void;
};

type DisconnectFunction = VoidFunction;
type ClickTileFunction = (tileId: string) => void;

export function createTilepadSocket(): TilepadSocket {
  let detailsState: TilepadSocketDetails | null = $state(null);
  let state: SocketState = $state({ type: "Initial" });

  let onDisconnect: DisconnectFunction | undefined;
  let onClickTile: ClickTileFunction | undefined;

  const disconnect = () => {
    state = { type: "Initial" };

    // Call current disconnect callback
    if (onDisconnect) {
      onDisconnect();
      onDisconnect = undefined;
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
      let name = await hostname();
      if (!name) name = "Tilepad Device";

      sendMessage({ type: "RequestApproval", name });

      // Clear current access token
      updateDevice(details.deviceId, { access_token: null });
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
          updateDevice(details.deviceId, { access_token: token });
          setState({ type: "Authenticating" });

          // Authenticate with the current token
          sendMessage({ type: "Authenticate", access_token: token });
          break;
        }

        case "Authenticated": {
          setState({ type: "Authenticated", tiles: [], folder: null });
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
      }
    }
  };

  return {
    state() {
      return state;
    },
    details() {
      return detailsState;
    },
    connect,
    disconnect,

    clickTile: (tileId: string) => {
      if (onClickTile) onClickTile(tileId);
    },
  };
}
