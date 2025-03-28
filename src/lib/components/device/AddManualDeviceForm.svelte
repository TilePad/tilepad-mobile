<script lang="ts">
  import Button from "../input/Button.svelte";
  import TextInput from "../input/TextInput.svelte";
  import NumberInput from "../input/NumberInput.svelte";

  type Props = {
    onAddDevice: (name: string, host: string, port: number) => void;
  };

  const { onAddDevice }: Props = $props();

  let name = $state("My Device");
  let host = $state("localhost");
  let port = $state(59371);

  function onSubmit(event: Event) {
    event.preventDefault();
    onAddDevice(name, host, port);
  }
</script>

<form onsubmit={onSubmit}>
  <label for="name">Name</label>
  <TextInput id="name" type="text" bind:value={name} />

  <label for="host">Host</label>
  <TextInput id="host" type="text" bind:value={host} />

  <label for="port">Port</label>
  <NumberInput id="port" bind:value={port} />

  <Button type="submit">Create</Button>
</form>
