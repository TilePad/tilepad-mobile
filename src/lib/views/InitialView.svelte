<script lang="ts">
  import DeviceList from "$lib/components/device/DeviceList.svelte";
  import { getTilepadSocket } from "$lib/components/WebsocketProvider.svelte";

  const { connect } = getTilepadSocket();
</script>

<DeviceList onConnect={connect} />
