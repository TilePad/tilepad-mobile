<script lang="ts">
  import { t } from "svelte-i18n";
  import Aside from "$lib/components/Aside.svelte";
  import { getErrorMessage } from "$lib/utils/error";
  import { createDevice, createDevicesQuery } from "$lib/api/devices";
  import SettingsSection from "$lib/components/SettingsSection.svelte";
  import SkeletonList from "$lib/components/skeleton/SkeletonList.svelte";
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

    <div class="actions">
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

    <SettingsSection />
  </div>

  <div class="right">
    <h1 class="header">{$t("devices")}</h1>

    {#if $devicesQuery.isLoading}
      <SkeletonList />
    {:else if $devicesQuery.isError}
      <Aside severity="error" style="margin: 1rem;">
        {$t("devices_error", {
          values: { error: getErrorMessage($devicesQuery.error) },
        })}
      </Aside>
    {:else if $devicesQuery.isSuccess}
      <div class="devices-wrapper">
        <div class="devices">
          {#each $devicesQuery.data as device}
            <DeviceListItem {device} onConnect={connect} />
          {/each}
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  .logo {
    max-width: 10rem;
    margin-bottom: 1rem;
  }

  h1 {
    margin-bottom: 0.5rem;
  }

  .layout {
    display: flex;
    height: 100%;
    overflow: hidden;
  }

  .left {
    padding: 1rem;
    width: 20rem;
    background-color: #131316;
    border-right: 1px solid #333;
  }

  .right {
    display: flex;
    flex-flow: column;
    height: 100%;
    flex: auto;
  }

  .header {
    display: flex;
    flex-flow: row;
    flex-shrink: 0;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
    padding: 0.5rem 1rem;
    border-bottom: 1px solid #333;
    background-color: #29262e;
  }

  .devices-wrapper {
    overflow: auto;
    flex: auto;
  }

  .devices {
    display: flex;
    flex-flow: column;
    gap: 1rem;
    padding: 1rem;
  }

  .actions {
    display: flex;
    flex-flow: column;
    gap: 1rem;
  }
</style>
