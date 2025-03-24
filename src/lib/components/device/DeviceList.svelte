<script lang="ts">
  import type { DeviceModel } from "$lib/api/types";

  import { onMount } from "svelte";
  import { getErrorMessage } from "$lib/utils/error";
  import { getDevices, createDevice } from "$lib/api/devices";

  import AddDeviceForm from "./AddDeviceForm.svelte";
  import DeviceListItem from "./DeviceListItem.svelte";

  type Props = {
    onConnect: (
      deviceId: string,
      name: string,
      host: string,
      port: number,
      access_token: string | null,
    ) => void;
  };

  const { onConnect }: Props = $props();

  let devicesPromise: Promise<DeviceModel[]> | undefined = $state();

  onMount(() => {
    devicesPromise = getDevices();
  });

  let create = $state(false);

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

    devicesPromise = getDevices();
  }
</script>

{#if devicesPromise}
  {#await devicesPromise}
    <p>Loading...</p>
  {:then devices}
    <button onclick={() => (create = !create)}>Create</button>
    <AddDeviceForm
      bind:open={create}
      onAddDevice={(name, host, port) =>
        onAddDevice(name, host, port, devices.length)}
    />

    <div class="devices">
      {#each devices as device}
        <DeviceListItem {device} {onConnect} />
      {/each}
    </div>
  {:catch err}
    <p>Failed to get devices: {getErrorMessage(err)}</p>
  {/await}
{/if}
