<script lang="ts">
  import type { DeviceModel } from "$lib/api/types/devices";
  import type { TilepadSocketDetails } from "$lib/api/socket.svelte";

  import { toast } from "svelte-sonner";
  import { removeDevice } from "$lib/api/devices";
  import { i18nContext } from "$lib/i18n/i18n.svelte";
  import { toastErrorMessage } from "$lib/utils/error";

  import Button from "../input/Button.svelte";

  type Props = {
    device: DeviceModel;
    onConnect: (details: TilepadSocketDetails) => void;
  };

  const { device, onConnect }: Props = $props();

  const i18n = i18nContext.get();

  function onRemove() {
    const removePromise = removeDevice(device.id);

    toast.promise(removePromise, {
      loading: i18n.f("device_removing"),
      success: i18n.f("device_removed"),
      error: toastErrorMessage(i18n.f("device_remove_error")),
    });
  }
</script>

<div class="device">
  <span class="device__name">{device.name}</span>
  <span class="device__host">
    {device.host}:{device.port}
  </span>

  <div class="actions">
    <Button
      onclick={() =>
        onConnect({
          deviceId: device.id,
          host: device.host,
          port: device.port,
          clientPrivateKey: device.client_private_key,
          serverPublicKey: device.server_public_key,
        })}
    >
      {i18n.f("connect")}
    </Button>

    <Button onclick={onRemove}>{i18n.f("remove")}</Button>
  </div>
</div>

<style>
  .actions {
    display: flex;
    gap: 1rem;
  }

  .device {
    display: flex;
    flex-flow: column;
    gap: 0.5rem;
    align-items: flex-start;

    padding: 1rem;
    border-radius: 0.5rem;
    background-color: #2f2c36;
    border: none;
  }

  .device__host {
    color: #ccc;
    font-size: 0.8rem;
  }

  .device__name {
    color: #fff;
    font-size: 1.2rem;
  }
</style>
