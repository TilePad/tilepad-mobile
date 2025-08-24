<script lang="ts">
  import type { EncodedInterfaces } from "$lib/api/types/server";

  import { t } from "svelte-i18n";
  import { Portal } from "bits-ui";
  import { onDestroy } from "svelte";
  import { testServerConnection } from "$lib/api/server";
  import { cancel } from "@tauri-apps/plugin-barcode-scanner";
  import * as barcode from "@tauri-apps/plugin-barcode-scanner";

  import Button from "../input/Button.svelte";
  import Dialog from "../dialog/Dialog.svelte";
  import PulseLoader from "../PulseLoader.svelte";
  import TextInput from "../input/TextInput.svelte";
  import NumberInput from "../input/NumberInput.svelte";

  type Props = {
    onAddDevice: (name: string, host: string, port: number) => void;
  };

  const { onAddDevice }: Props = $props();

  const State = {
    Initial: 0,
    Scanning: 1,
    Scanned: 2,
    Checking: 3,
    NothingValid: 4,
    InvalidQR: 5,
  };

  let open = $state(false);
  let currentState = $state(State.Initial);
  let name = $state("");
  let host = $state("");
  let port = $state(0);

  let restoreTransparency = () => {};

  /**
   * Handle scan QR code button
   */
  async function onClickScan() {
    try {
      const hasPermission = await barcode.checkPermissions();
      if (hasPermission !== "granted") {
        const result = await barcode.requestPermissions();
        if (result === "denied") {
          console.error("camera permission denied");
          barcode.openAppSettings();
          return;
        }
      }

      const restore = createTemporaryTransparency();
      restoreTransparency = restore;

      const scanned = await barcode.scan({
        windowed: true,
        formats: [barcode.Format.QRCode],
      });

      restore();
      restoreTransparency = () => {};

      const content = scanned.content;
      onScanned(content);
    } catch (err) {
      console.error("failed to scan qr", err);
    }
  }

  function createTemporaryTransparency() {
    const changes: {
      element: HTMLElement;
      originalOpacity: string;
    }[] = [];

    function makeTransparent(el: HTMLElement) {
      const style = getComputedStyle(el);
      changes.push({
        element: el,
        originalOpacity: style.opacity,
      });

      el.style.opacity = "0";
    }

    const originalBodyBg = document.body.style.backgroundColor;
    document.body.style.backgroundColor = "transparent";

    makeTransparent(document.getElementById("mainContent")!);

    return () => {
      document.body.style.backgroundColor = originalBodyBg;

      for (const { element, originalOpacity } of changes) {
        element.style.opacity = originalOpacity;
      }
    };
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

    currentState = State.Checking;

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
    onClose();
  }

  function onClose() {
    open = false;
    currentState = State.Initial;
    onCancel();
  }

  function onCancel() {
    if (currentState === State.Scanning) cancel();
    restoreTransparency();
    restoreTransparency = () => {};
  }

  function onOpen() {
    open = true;
    currentState = State.Initial;
    onClickScan();
  }

  onDestroy(() => {
    onCancel();
  });
</script>

{#if !open || currentState !== State.Initial}
  <Dialog {open} onOpenChange={(value) => (open = value)}>
    {#snippet button({ props })}
      <Button {...props} onclick={onOpen}>
        <img src="/qr-icon.svg" alt="QR Icon" width="32px" height="32px" />
        {$t("scan_qr_code")}
      </Button>
    {/snippet}

    {#snippet children()}
      <div class="content">
        {#if currentState === State.Scanning}
          <p>{$t("scanning")}</p>
          <div class="actions">
            <Button onclick={onClose}>{$t("cancel")}</Button>
          </div>
        {:else if currentState === State.Checking}
          <PulseLoader />
          <p>{$t("checking_addresses")}</p>
          <div class="actions">
            <Button onclick={onClose}>{$t("cancel")}</Button>
          </div>
        {:else if currentState === State.Scanned}
          <form onsubmit={onSubmit}>
            <label for="name">{$t("name")}</label>
            <TextInput id="name" type="text" bind:value={name} />

            <label for="host">{$t("host")}</label>
            <TextInput id="host" type="text" bind:value={host} />

            <label for="port">{$t("port")}</label>
            <NumberInput
              id="port"
              value={port}
              oninput={(event) => (port = event.currentTarget.valueAsNumber)}
            />

            <div class="actions">
              <Button onclick={onClose}>{$t("cancel")}</Button>
              <Button type="submit">{$t("save")}</Button>
            </div>
          </form>
        {:else if currentState === State.NothingValid}
          <p>{$t("scanned_none")}</p>

          <div class="actions">
            <Button onclick={onClose}>{$t("cancel")}</Button>
          </div>
        {:else if currentState === State.InvalidQR}
          <p>{$t("invalid_qr")}</p>

          <div class="actions">
            <Button onclick={onClose}>{$t("cancel")}</Button>
          </div>
        {/if}
      </div>
    {/snippet}
  </Dialog>
{/if}

{#if open && currentState === State.Initial}
  <Portal to={document.body}>
    <div class="scan-overlay">
      <div class="top">
        <h1>{$t("scan_qr_code")}</h1>
        <Button onclick={onClose}>{$t("cancel")}</Button>
      </div>
      <div class="skeleton" style="width: 100%; height: 0.5rem"></div>
    </div>
  </Portal>
{/if}

<style>
  .content {
    padding: 1rem;
    min-width: 25rem;
  }

  .scan-overlay {
    padding: 1rem;
    display: flex;
    gap: 1rem;
    flex-direction: column;
    position: fixed;
    top: 0;
    left: 0;
    background-color: #000000;
    border-bottom-right-radius: 0.5rem;
  }

  .top {
    display: flex;
    gap: 1rem;
  }

  form {
    display: flex;
    flex-flow: column;
    gap: 0.5rem;
  }

  .actions {
    margin-top: 0.5rem;
    display: flex;

    gap: 1rem;
  }

  .actions :global(> *) {
    flex: auto;
  }
</style>
