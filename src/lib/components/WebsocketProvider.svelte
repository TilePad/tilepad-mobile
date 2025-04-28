<script module lang="ts">
  const KEY = Symbol("WEBSOCKET");

  export function getTilepadSocket(): TilepadSocket {
    return getContext(KEY);
  }
</script>

<script lang="ts">
  import { getContext, setContext, type Snippet } from "svelte";
  import {
    type TilepadSocket,
    createTilepadSocket,
  } from "$lib/api/socket.svelte";

  import { getSettingsContext } from "./SettingsProvider.svelte";

  type Props = {
    children: Snippet;
  };

  const { children }: Props = $props();
  const settingsContext = getSettingsContext();
  const settings = $derived.by(settingsContext.settings);

  const socket = createTilepadSocket(() => settings.device_name);

  setContext(KEY, socket);
</script>

{@render children()}
