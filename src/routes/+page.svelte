<script lang="ts">
  import type {
    TileModel,
    FolderModel,
    ConnectionDetails,
    ClientDeviceMessage,
    ServerDeviceMessage,
  } from "$lib/api/types";

  import { updateDevice } from "$lib/api/devices";
  import { hostname } from "@tauri-apps/plugin-os";
  import TilesView from "$lib/components/tiles/TilesView.svelte";
  import DeviceList from "$lib/components/device/DeviceList.svelte";

  enum Status {
    Pending = "Pending",
    PendingApproval = "PendingApproval",
    Connecting = "Connecting",
    Connected = "Connected",
    Declined = "Declined",
    Revoked = "Revoked",
  }

  let status: Status = Status.Pending;
  let socket: WebSocket | null = null;

  let data: { tiles: TileModel[]; folder: FolderModel } | null = null;
  let connection: ConnectionDetails | undefined = undefined;

  function onConnect(
    deviceId: string,
    name: string,
    host: string,
    port: number,
    accessToken: string | null,
  ) {
    connection = { deviceId, host, port, accessToken };
    status = Status.Connecting;
    const ws = new WebSocket(`ws://${host}:${port}/devices/ws`);
    socket = ws;

    ws.onopen = () => {
      status = Status.Connected;

      if (accessToken) {
        sendMessage({ type: "Authenticate", access_token: accessToken });
      } else {
        status = Status.PendingApproval;
        sendMessage({ type: "RequestApproval", name: "Test Device" });
      }
    };

    ws.onclose = () => {
      socket = null;
      status = Status.Pending;
    };

    ws.onmessage = (event) => {
      const msg: ServerDeviceMessage = JSON.parse(event.data);
      onMessage(msg);
    };
  }

  function sendMessage(message: ClientDeviceMessage) {
    const msg = JSON.stringify(message);
    if (socket) socket.send(msg);
  }

  function onMessage(msg: ServerDeviceMessage) {
    switch (msg.type) {
      case "Declined": {
        if (socket) socket.close();
        socket = null;
        status = Status.Declined;
        break;
      }
      case "Approved": {
        const token = msg.access_token;

        if (connection) {
          connection = { ...connection, accessToken: token };
          updateDevice(connection.deviceId, {
            access_token: token,
          });
        }

        sendMessage({ type: "Authenticate", access_token: token });
        break;
      }
      case "Revoked": {
        if (socket) socket.close();
        socket = null;
        status = Status.Revoked;
        break;
      }
      case "Authenticated": {
        sendMessage({ type: "RequestTiles" });
        break;
      }
      case "InvalidAccessToken": {
        hostname().then((hostname) => {
          if (!hostname) return;

          sendMessage({ type: "RequestApproval", name: hostname });

          if (connection) {
            connection = { ...connection, accessToken: null };
            updateDevice(connection.deviceId, {
              access_token: null,
            });
          }
        });

        break;
      }
      case "Tiles": {
        data = { tiles: msg.tiles, folder: msg.folder };
        break;
      }
    }
  }
</script>

{#if status === Status.Pending}
  <DeviceList {onConnect} />
{:else if status === Status.PendingApproval}
  <p>Waiting for approval</p>
{:else if status === Status.Connected}
  <div class="layout">
    <div class="tiles">
      {#if data && connection}
        <TilesView
          {connection}
          tiles={data.tiles}
          folder={data.folder}
          onClick={(tileId) => {
            sendMessage({
              type: "TileClicked",
              tile_id: tileId,
            });
          }}
        />
      {/if}
    </div>
  </div>
{/if}

<style>
  .layout {
    height: 100%;
    display: flex;

    flex-flow: column;
  }

  .tiles {
    flex: auto;
    overflow: hidden;
  }
</style>
