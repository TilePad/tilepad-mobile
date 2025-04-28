<script lang="ts">
  import { t } from "svelte-i18n";

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
  let name = $state($t("default_device_name"));
  let host = $state("localhost");
  let port = $state(59371);

  function onSubmit(event: Event) {
    event.preventDefault();
    onAddDevice(name, host, port);
    onClose();
  }

  function onClose() {
    // Reset on close
    open = false;
    name = $t("default_device_name");
    host = "localhost";
    port = 59371;
  }
</script>

<Dialog {open} onOpenChange={(value) => (open = value)}>
  {#snippet button({ props })}
    <Button {...props}>
      <img src="/form.svg" alt="Form Icon" width="32px" height="32px" />
      {$t("manual_add")}
    </Button>
  {/snippet}

  {#snippet children()}
    <div class="content">
      <form onsubmit={onSubmit}>
        <label for="name">{$t("name")}</label>
        <TextInput id="name" type="text" bind:value={name} />

        <label for="host">{$t("host")}</label>
        <TextInput id="host" type="text" bind:value={host} />

        <label for="port">{$t("port")}</label>
        <NumberInput id="port" bind:value={port} />

        <div class="actions">
          <DialogCloseButton buttonLabel={{ text: $t("cancel") }} />
          <Button type="submit">{$t("create")}</Button>
        </div>
      </form>
    </div>
  {/snippet}
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
