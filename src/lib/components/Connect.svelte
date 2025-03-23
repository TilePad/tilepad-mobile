<script lang="ts">
  import type { EncodedInterfaces } from "$lib/api/types";

  import { testServerConnection } from "$lib/api/server";
  import * as barcode from "@tauri-apps/plugin-barcode-scanner";

  type Props = {
    onConnect: (host: string, port: number) => void;
  };

  const { onConnect }: Props = $props();

  let host: string = $state("localhost");
  let port: number = $state(59371);

  /**
   * Handle scan QR code button
   */
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
      onScanned(content);
    } catch (err) {
      console.error("failed to scan qr", err);
    }
  }

  /**
   * QR code scanned successfully, attempt connecting
   * to the tilepad server
   *
   * @param content Contents of the scanned QR code
   */
  async function onScanned(content: string) {
    const parsed: EncodedInterfaces = JSON.parse(content);

    // Missing addresses list for a tilepad QR
    if (!Array.isArray(parsed.addr)) throw new Error("Invalid QR code");

    for (const addr of parsed.addr) {
      // Test for a valid connection
      const connection = await testServerConnection(addr, parsed.port);
      if (!connection) {
        continue;
      }

      onConnect(addr, parsed.port);
      return;
    }

    console.error("All connections were invalid");
  }
</script>

<div>
  <h1>Connect to Tilepad</h1>
  <div>
    <!-- Scan -->
    <button onclick={onClickScan}> Scan </button>

    <!-- Manual -->
    <div>
      Host: <input type="text" bind:value={host} />
      Port: <input type="number" bind:value={port} />
      <button onclick={() => onConnect(host, port)}> Connect </button>
    </div>
  </div>
</div>
