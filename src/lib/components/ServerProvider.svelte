<script module lang="ts">
  import { getContext, setContext, type Snippet } from "svelte";

  const serverContextKey = Symbol("SERVER_CONTEXT");

  interface ServerProviderContext {
    serverURL: string;
    deviceId: string;
  }

  export function getServerContext(): ServerProviderContext {
    return getContext(serverContextKey);
  }
</script>

<script lang="ts">
  type Props = {
    serverURL: string;
    deviceId: string;
    children?: Snippet;
  };

  const { serverURL, deviceId, children }: Props = $props();

  setContext(serverContextKey, {
    get serverURL() {
      return serverURL;
    },
    get deviceId() {
      return deviceId;
    },
  });
</script>

{@render children?.()}
