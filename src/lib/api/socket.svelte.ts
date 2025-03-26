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
  | { type: "Revoked" };

// Details about a tilepad socket we are connecting to
export interface TilepadSocketDetails {
  deviceId: string;
  host: string;
  port: number;
  accessToken: string | null;
}

type OnSocketMessage = (msg: ServerDeviceMessage) => void;

class Socket {
  ws: WebSocket;
  details: TilepadSocketDetails;

  onOpen: VoidFunction | undefined;
  onClose: VoidFunction | undefined;
  onMessage: OnSocketMessage | undefined;

  constructor(details: TilepadSocketDetails) {
    this.ws = new WebSocket(`ws://${details.host}:${details.port}/devices/ws`);
    this.details = details;

    this.ws.onopen = () => {
      if (this.onOpen) this.onOpen();
    };
    this.ws.onclose = () => {
      if (this.onClose) this.onClose();
    };
    this.ws.onmessage = (event) => {
      const msg: ServerDeviceMessage = JSON.parse(event.data);
      if (this.onMessage) this.onMessage(msg);
    };
  }

  sendMessage(message: ClientDeviceMessage) {
    this.ws.send(JSON.stringify(message));
  }

  close() {
    this.ws.close();
  }
}

export type TilepadSocket = {
  state: () => SocketState;
  socket: () => Socket | null;
  connect: (details: TilepadSocketDetails) => void;
  disconnect: () => void;
  clickTile: (tileId: string) => void;
};

export function createTilepadSocket(): TilepadSocket {
  let socket: Socket | null = $state(null);
  let state: SocketState = $state({ type: "Initial" });

  const connect = (details: TilepadSocketDetails) => {
    // Disconnect if already connected
    disconnect();

    // Establish connection
    socket = new Socket(details);
    state = { type: "Connecting" };

    const requestApproval = async (socket: Socket) => {
      state = { type: "RequestingApproval" };

      // Request the device name
      let name = await hostname();
      if (!name) name = "Tilepad Device";

      socket.sendMessage({ type: "RequestApproval", name });

      // Clear current access token
      updateDevice(socket.details.deviceId, { access_token: null });
    };

    socket.onMessage = (msg) => {
      if (!socket) return;

      switch (msg.type) {
        // Server declined approval
        case "Declined": {
          disconnect();
          state = { type: "Declined" };
          break;
        }

        // Server revoked access
        case "Revoked": {
          disconnect();
          state = { type: "Revoked" };
          break;
        }

        // Server approved access
        case "Approved": {
          const token = msg.access_token;

          // Update current device access token
          updateDevice(socket.details.deviceId, { access_token: token });
          state = { type: "Authenticating" };

          // Authenticate with the current token
          socket.sendMessage({ type: "Authenticate", access_token: token });
          break;
        }

        case "Authenticated": {
          state = { type: "Authenticated", tiles: [], folder: null };
          socket.sendMessage({ type: "RequestTiles" });
          break;
        }

        case "InvalidAccessToken": {
          requestApproval(socket);
          break;
        }
        case "Tiles": {
          if (state.type === "Authenticated") {
            state = { ...state, tiles: msg.tiles, folder: msg.folder };
          }

          break;
        }
      }
    };

    // Handle connection opened
    socket.onOpen = () => {
      if (!socket) return;

      if (details.accessToken) {
        state = { type: "Authenticating" };
        socket.sendMessage({
          type: "Authenticate",
          access_token: details.accessToken,
        });
      } else {
        requestApproval(socket);
      }
    };

    // Handle connection closed
    socket.onClose = () => {
      state = { type: "Initial" };
    };
  };

  const disconnect = () => {
    if (socket) {
      socket.close();
      socket = null;
    }

    state = { type: "Initial" };
  };

  const clickTile = (tileId: string) => {
    if (!socket) return;
    socket.sendMessage({
      type: "TileClicked",
      tile_id: tileId,
    });
  };

  return {
    state() {
      return state;
    },
    socket() {
      return socket;
    },
    connect,
    disconnect,

    clickTile,
  };
}
