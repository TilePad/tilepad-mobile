<script lang="ts">
  import Button from "../input/Button.svelte";
  import Dialog from "../dialog/Dialog.svelte";
  import TextInput from "../input/TextInput.svelte";
  import NumberInput from "../input/NumberInput.svelte";
  import DialogCloseButton from "../dialog/DialogCloseButton.svelte";

  type Props = {
    onAddDevice: (name: string, host: string, port: number) => void;
  };

  const { onAddDevice }: Props = $props();

  let open = $state(false);
  let name = $state("My Device");
  let host = $state("localhost");
  let port = $state(59371);

  function onSubmit(event: Event) {
    event.preventDefault();
    onAddDevice(name, host, port);
    onClose();
  }

  function onClose() {
    open = false;
  }
</script>

<Dialog {open} onOpenChange={(value) => (open = value)}>
  {#snippet button({ props })}
    <Button {...props}>
      <img src="/form.svg" alt="QR Icon" width="32px" height="32px" />
      Add Manually
    </Button>
  {/snippet}

  {#snippet children()}
    <div class="content">
      <DialogCloseButton buttonLabel={{ text: "Cancel" }} />

      <form onsubmit={onSubmit}>
        <label for="name">Name</label>
        <TextInput id="name" type="text" bind:value={name} />

        <label for="host">Host</label>
        <TextInput id="host" type="text" bind:value={host} />

        <label for="port">Port</label>
        <NumberInput id="port" bind:value={port} />

        <Button type="submit">Create</Button>
      </form>
    </div>
  {/snippet}
</Dialog>

<style>
  .content {
    padding: 1rem;
    min-width: 25rem;
  }
</style>
