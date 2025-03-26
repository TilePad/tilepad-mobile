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

  type Props = {
    children: Snippet;
  };

  const { children }: Props = $props();

  const socket = createTilepadSocket();

  setContext(KEY, socket);
</script>

{@render children()}
