<script lang="ts">
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
  <input id="name" type="text" bind:value={name} />

  <label for="host">Host</label>
  <input id="host" type="text" bind:value={host} />

  <label for="port">Port</label>
  <input id="port" type="number" bind:value={port} />

  <button type="submit">Create</button>
</form>
