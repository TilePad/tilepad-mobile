<script lang="ts">
  import HTabs from "../HTabs.svelte";
  import Dialog from "../dialog/Dialog.svelte";
  import AddScanDeviceForm from "./AddScanDeviceForm.svelte";
  import AddManualDeviceForm from "./AddManualDeviceForm.svelte";
  import DialogCloseButton from "../dialog/DialogCloseButton.svelte";

  type Props = {
    onAddDevice: (name: string, host: string, port: number) => void;
  };

  let { onAddDevice: onAddDeviceOuter }: Props = $props();

  let open = $state(false);

  function onAddDevice(name: string, host: string, port: number) {
    onAddDeviceOuter(name, host, port);
    onClose();
  }

  function onClose() {
    open = false;
  }
</script>

{#snippet scanContent()}
  <p>Scan connection from a QR code</p>
  <AddScanDeviceForm {onAddDevice} />
{/snippet}

{#snippet manualContent()}
  <p>Manually enter connection details</p>
  <AddManualDeviceForm {onAddDevice} />
{/snippet}

<Dialog
  {open}
  onOpenChange={(value) => (open = value)}
  buttonLabel={{ text: "Create" }}
>
  {#snippet children()}
    <div class="content">
      <DialogCloseButton buttonLabel={{ text: "Cancel" }} />

      <HTabs
        tabs={[
          {
            value: "scan",
            label: "Scan",
            content: scanContent,
          },
          {
            value: "manual",
            label: "Manual",
            content: manualContent,
          },
        ]}
      />
    </div>
  {/snippet}
</Dialog>

<style>
  .content {
    padding: 1rem;
    min-width: 25rem;
  }
</style>
