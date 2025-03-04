<script lang="ts">
  import type {
    TileModel,
    FolderModel,
    ConnectionDetails,
    EncodedInterfaces,
    ClientDeviceMessage,
    ServerDeviceMessage,
  } from "$lib/api/types";

  import { testServerConnection } from "$lib/api/server";
  import * as barcode from "@tauri-apps/plugin-barcode-scanner";
  import TilesView from "$lib/components/tiles/TilesView.svelte";

  enum Status {
    Pending = "Pending",
    Connecting = "Connecting",
    Connected = "Connected",
    Declined = "Declined",
    Revoked = "Revoked",
  }

  let scanning = false;
  let status: Status = Status.Pending;
  let socket: WebSocket | null = null;

  let data: { tiles: TileModel[]; folder: FolderModel } | null = null;
  let connection: ConnectionDetails | undefined = undefined;

  let host: string = "localhost";
  let port: number = 59371;

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

  async function onClickScan() {
    try {
      const hasPermission = await barcode.checkPermissions();
      if (hasPermission !== "granted") {
        const result = await barcode.requestPermissions();
        if (result === "denied") {
          console.error("DENIED PERMISSION REQUEST");
          barcode.openAppSettings();
          return;
        }
      }

      const scanned = await barcode.scan({
        windowed: false,
        formats: [barcode.Format.QRCode],
      });

      const content = scanned.content;
      const parsed: EncodedInterfaces = JSON.parse(content);

      for (const addr of parsed.addr) {
        const connection = await testServerConnection(addr, parsed.port);
        if (!connection) {
          continue;
        }

        onConnect(addr, parsed.port);
        return;
      }

      console.error("All connections were invalid");
    } catch (err) {
      console.error("failed to scan qr", err);
    }
  }
</script>

<div class="layout">
  <div>
    <button onclick={onClickScan}> Scan </button>
    <button onclick={() => onConnect(host, port)}> Connect </button>
    Host: <input type="text" bind:value={host} />
    Port: <input type="number" bind:value={port} />

    {status}
  </div>

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
