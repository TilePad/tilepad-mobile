<script lang="ts">
  import type { TilepadSocketDetails } from "$lib/api/socket.svelte";

  import { getErrorMessage } from "$lib/utils/error";
  import { createDevice, createDevicesQuery } from "$lib/api/devices";

  import AddDeviceForm from "./AddDeviceForm.svelte";
  import DeviceListItem from "./DeviceListItem.svelte";

  type Props = {
    onConnect: (details: TilepadSocketDetails) => void;
  };

  const { onConnect }: Props = $props();

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

{#if $devicesQuery.isLoading}
  <p>Loading...</p>
{:else if $devicesQuery.isError}
  <p>Failed to get devices: {getErrorMessage($devicesQuery.error)}</p>
{:else if $devicesQuery.isSuccess}
  <div class="layout">
    <div class="header">
      <h1>Devices</h1>
      <AddDeviceForm
        onAddDevice={(name, host, port) =>
          onAddDevice(name, host, port, $devicesQuery.data.length)}
      />
    </div>

    <div class="devices">
      {#each $devicesQuery.data as device}
        <DeviceListItem {device} {onConnect} />
      {/each}
    </div>
  </div>
{/if}

<style>
  .devices {
    display: flex;
    flex-flow: column;
    gap: 1rem;
  }

  .layout {
    display: flex;
    flex-flow: column;
    gap: 1rem;
    padding: 1rem;
  }

  .header {
    display: flex;
    gap: 1rem;
    align-items: center;
    justify-content: space-between;
  }
</style>
