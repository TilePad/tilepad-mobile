<script lang="ts">
  import { i18nContext } from "$lib/i18n/i18n.svelte";

  import Button from "../input/Button.svelte";
  import Dialog from "../dialog/Dialog.svelte";
  import TextInput from "../input/TextInput.svelte";
  import NumberInput from "../input/NumberInput.svelte";
  import DialogCloseButton from "../dialog/DialogCloseButton.svelte";

  type Props = {
    onAddDevice: (name: string, host: string, port: number) => void;
  };

  const { onAddDevice }: Props = $props();

  const i18n = i18nContext.get();

  let open = $state(false);
  let name = $state(i18n.f("default_device_name"));
  let host = $state("localhost");
  let port = $state(8532);

  function onSubmit(event: Event) {
    event.preventDefault();
    onAddDevice(name, host, port);
    onClose();
  }

  function onClose() {
    // Reset on close
    open = false;
    name = i18n.f("default_device_name");
    host = "localhost";
    port = 8532;
  }
</script>

<Dialog {open} onOpenChange={(value) => (open = value)}>
  {#snippet button({ props })}
    <Button {...props}>
      <img src="/form.svg" alt="Form Icon" width="32px" height="32px" />
      {i18n.f("manual_add")}
    </Button>
  {/snippet}

  <div class="content">
    <form onsubmit={onSubmit}>
      <label for="name">{i18n.f("name")}</label>
      <TextInput id="name" type="text" bind:value={name} />

      <label for="host">{i18n.f("host")}</label>
      <TextInput id="host" type="text" bind:value={host} />

      <label for="port">{i18n.f("port")}</label>
      <NumberInput
        id="port"
        value={port}
        oninput={(event) => (port = event.currentTarget.valueAsNumber)}
      />

      <div class="actions">
        <DialogCloseButton buttonLabel={{ text: i18n.f("cancel") }} />
        <Button type="submit">{i18n.f("create")}</Button>
      </div>
    </form>
  </div>
</Dialog>

<style>
  .content {
    display: flex;
    flex-flow: column;
    gap: 0.5rem;
    padding: 1rem;
    min-width: 25rem;
  }

  form {
    display: flex;
    flex-flow: column;
    gap: 0.5rem;
  }

  .actions {
    margin-top: 0.5rem;
    display: flex;

    gap: 1rem;
  }

  .actions :global(> *) {
    flex: auto;
  }
</style>
