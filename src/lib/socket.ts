class DeviceSocket {
  socket: WebSocket;

  constructor(socket: WebSocket) {
    this.socket = socket;
    this.socket.onmessage = this.onMessageEvent.bind(this);
    this.socket.onclose = () => {
      console.debug("device socket closed");
    };
  }

  onMessageEvent(event: MessageEvent<string>) {
    const raw = event.data;
    const msg: ServerDeviceMessage = JSON.parse(raw);
    this.onMessage(msg);
  }

  onMessage(msg: ServerDeviceMessage) {
    switch (msg.type) {
      case "Declined":
      case "Approved":
      case "Revoked":
      case "Authenticated":
      case "InvalidAccessToken":
      case "Tiles":
    }
  }

  sendMessage(message: ClientDeviceMessage) {
    const msg = JSON.stringify(message);
    this.socket.send(msg);
  }
}

type ClientDeviceMessage =
  | { type: "RequestApproval"; name: string }
  | { type: "RequestTiles" }
  | { type: "Authenticate"; access_token: string }
  | { type: "TileClicked"; tile_id: string };

type ServerDeviceMessage =
  | { type: "Declined" }
  | { type: "Approved"; device_id: string; access_token: string }
  | { type: "Revoked" }
  | { type: "Authenticated" }
  | { type: "InvalidAccessToken" }
  | { type: "Tiles"; tiles: TileModel[] };

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
  icon: unknown;
  properties: object;
}
