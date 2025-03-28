<script lang="ts">
  import type { EncodedInterfaces } from "$lib/api/types/server";

  import { testServerConnection } from "$lib/api/server";
  import * as barcode from "@tauri-apps/plugin-barcode-scanner";

  import Button from "../input/Button.svelte";
  import PulseLoader from "../PulseLoader.svelte";
  import DialogCloseButton from "../dialog/DialogCloseButton.svelte";

  type Props = {
    onAddDevice: (name: string, host: string, port: number) => void;
  };

  const { onAddDevice }: Props = $props();

  const State = {
    Initial: 0,
    Scanning: 1,
    Scanned: 2,
    Checking: 2,
    NothingValid: 3,
    InvalidQR: 4,
  };

  let currentState = $state(State.Initial);
  let name = $state("");
  let host = $state("");
  let port = $state(0);

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
    if (!Array.isArray(parsed.addr)) {
      currentState = State.InvalidQR;
      return;
    }

    for (const addr of parsed.addr) {
      // Test for a valid connection
      const connection = await testServerConnection(addr, parsed.port);
      if (connection.result === "error") {
        continue;
      }

      name = connection.name;
      host = addr;
      port = parsed.port;
      currentState = State.Scanned;
      return;
    }

    console.error("All connections were invalid");
    currentState = State.NothingValid;
  }

  function onSubmit(event: Event) {
    event.preventDefault();
    onAddDevice(name, host, port);
  }
</script>

{#if currentState === State.Initial}
  <Button onclick={onClickScan}>Scan</Button>
{:else if currentState === State.Scanning}
  <p>Scanning</p>
{:else if currentState === State.Checking}
  <PulseLoader />
  <p>Checking addresses...</p>
{:else if currentState === State.Scanned}
  <form onsubmit={onSubmit}>
    <label for="name">Name</label>
    <input id="name" type="text" bind:value={name} />

    <label for="host">Host</label>
    <input id="host" type="text" bind:value={host} />

    <label for="port">Port</label>
    <input id="port" type="number" bind:value={port} />

    <DialogCloseButton type="submit" buttonLabel={{ text: "Save" }} />
  </form>
{:else if currentState === State.NothingValid}
  <p>None of the scanned addresses were connectable</p>
{:else if currentState === State.InvalidQR}
  <p>QR code is not a valid Tilepad QR</p>
{/if}
