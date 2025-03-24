<script lang="ts">
  import type { DeviceModel } from "$lib/api/types";
  import type { TilepadSocketDetails } from "$lib/api/socket.svelte";

  import { invoke } from "@tauri-apps/api/core";

  type Props = {
    device: DeviceModel;
    onConnect: (details: TilepadSocketDetails) => void;
  };

  const { device, onConnect }: Props = $props();

  function removeDevices(deviceId: string) {
    return invoke("devices_remove_device");
  }
</script>

<button
  onclick={() =>
    onConnect({
      deviceId: device.id,
      host: device.host,
      port: device.port,
      accessToken: device.access_token,
    })}
>
  {device.name}

  <span>
    {device.host}:{device.port}
  </span>
</button>
