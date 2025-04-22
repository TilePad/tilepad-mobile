<script lang="ts">
  import { getErrorMessage } from "$lib/utils/error";
  import Button from "$lib/components/input/Button.svelte";
  import { createDevice, createDevicesQuery } from "$lib/api/devices";
  import DeviceListItem from "$lib/components/device/DeviceListItem.svelte";
  import { getTilepadSocket } from "$lib/components/WebsocketProvider.svelte";
  import AddScanDeviceForm from "$lib/components/device/AddScanDeviceForm.svelte";
  import AddManualDeviceForm from "$lib/components/device/AddManualDeviceForm.svelte";

  const { connect } = getTilepadSocket();

  const devicesQuery = createDevicesQuery();

  async function onAddDevice(
    name: string,
    host: string,
    port: number,
    order: number,
  ) {
    await createDevice({
      name,
      host,
      port,
      access_token: null,
      order,
    });
  }
</script>

<div class="layout">
  <div class="left">
    <img src="/tilepad-logo.svg" alt="Tilepad Logo" class="logo" />
    <h1>Devices</h1>

    <div>
      {#if $devicesQuery.data}
        <AddScanDeviceForm
          onAddDevice={(name, host, port) =>
            onAddDevice(name, host, port, $devicesQuery.data.length)}
        />

        <AddManualDeviceForm
          onAddDevice={(name, host, port) =>
            onAddDevice(name, host, port, $devicesQuery.data.length)}
        />
      {/if}
    </div>
  </div>

  <div class="right">
    {#if $devicesQuery.isLoading}
      <p>Loading...</p>
    {:else if $devicesQuery.isError}
      <p>Failed to get devices: {getErrorMessage($devicesQuery.error)}</p>
    {:else if $devicesQuery.isSuccess}
      <div class="devices">
        {#each $devicesQuery.data as device}
          <DeviceListItem {device} onConnect={connect} />
        {/each}
      </div>
    {/if}
  </div>
</div>

<style>
  .logo {
    max-width: 10rem;
  }

  .layout {
    display: flex;
    height: 100%;
    overflow: hidden;
  }

  .left {
    padding: 1rem;
    max-width: 20rem;
  }

  .right {
    height: 100%;
    overflow: auto;
    flex: auto;
  }

  .devices {
    display: flex;
    flex-flow: column;
    gap: 1rem;
    padding: 1rem;
  }
</style>
