<script lang="ts">
  import type { DisplayContext } from "$lib/api/types/plugin";

  import { type Snippet } from "svelte";

  import { getTilepadSocket } from "../WebsocketProvider.svelte";

  type Props = {
    onMessage: (context: DisplayContext, message: object) => void;
    children?: Snippet;
  };

  const { events } = getTilepadSocket();

  const { onMessage, children }: Props = $props();

  $effect(() => {
    events.on("recv_from_plugin", onMessage);
    return () => {
      events.off("recv_from_plugin", onMessage);
    };
  });
</script>

{@render children?.()}
