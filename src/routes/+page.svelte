<script lang="ts">
  import type {
    TileModel,
    FolderModel,
    ConnectionDetails,
    ClientDeviceMessage,
    ServerDeviceMessage,
  } from "$lib/api/types";

  import Connect from "$lib/components/Connect.svelte";
  import TilesView from "$lib/components/tiles/TilesView.svelte";

  enum Status {
    Pending = "Pending",
    Connecting = "Connecting",
    Connected = "Connected",
    Declined = "Declined",
    Revoked = "Revoked",
  }

  let status: Status = Status.Pending;
  let socket: WebSocket | null = null;

  let data: { tiles: TileModel[]; folder: FolderModel } | null = null;
  let connection: ConnectionDetails | undefined = undefined;

  function onConnect(host: string, port: number) {
    connection = { host, port };
    status = Status.Connecting;
    const ws = new WebSocket(`ws://${host}:${port}/devices/ws`);
    socket = ws;

    ws.onopen = () => {
      status = Status.Connected;

      const token = localStorage.getItem("ACCESS_TOKEN");
      if (token) {
        sendMessage({ type: "Authenticate", access_token: token });
      } else {
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
        localStorage.setItem("ACCESS_TOKEN", token);
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
        sendMessage({ type: "RequestApproval", name: "Test Device" });
        localStorage.removeItem("ACCESS_TOKEN");
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
  <Connect {onConnect} />
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
