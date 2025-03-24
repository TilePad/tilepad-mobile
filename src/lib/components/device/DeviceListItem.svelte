<script lang="ts">
  import type { DeviceModel } from "$lib/api/types";

  import { invoke } from "@tauri-apps/api/core";

  type Props = {
    device: DeviceModel;
    onConnect: (
      deviceId: string,
      name: string,
      host: string,
      port: number,
      access_token: string | null,
    ) => void;
  };

  const { device, onConnect }: Props = $props();

  function removeDevices(deviceId: string) {
    return invoke("devices_remove_device");
  }
</script>

<button
  onclick={() =>
    onConnect(
      device.id,
      device.name,
      device.host,
      device.port,
      device.access_token,
    )}
>
  {device.name}

  <span>
    {device.host}:{device.port}
  </span>
</button>
