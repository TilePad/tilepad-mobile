<script lang="ts">
  import AddScanDeviceForm from "./AddScanDeviceForm.svelte";
  import AddManualDeviceForm from "./AddManualDeviceForm.svelte";

  type Props = {
    open: boolean;
    onAddDevice: (name: string, host: string, port: number) => void;
  };

  let { open = $bindable(), onAddDevice }: Props = $props();

  const Tab = {
    Scan: 0,
    Manual: 1,
  };

  let tab = $state(Tab.Scan);

  function onClose() {
    open = false;
  }
</script>

{#if open}
  <div class="overlay"></div>
  <div class="dialog">
    <button onclick={onClose}>Cancel</button>

    <div>
      <button onclick={() => (tab = Tab.Scan)}>Scan</button>
      <button onclick={() => (tab = Tab.Manual)}>Manual</button>
    </div>

    <div>
      {#if tab === Tab.Scan}
        <!-- Scan -->
        <AddScanDeviceForm {onAddDevice} />
      {:else if tab === Tab.Manual}
        <!-- Manual -->
        <AddManualDeviceForm {onAddDevice} />
      {/if}
    </div>
  </div>
{/if}

<style>
  .overlay {
    background-color: #00000050;
  }

  .dialog {
    position: absolute;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%);
    background-color: #000000;
    padding: 1rem;
  }
</style>
