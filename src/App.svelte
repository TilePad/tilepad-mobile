<script lang="ts">
  import { queryClient } from "$lib/api/client";
  import "$lib/styles/app.css";
  import AppToaster from "$lib/components/AppToaster.svelte";
  import { QueryClientProvider } from "@tanstack/svelte-query";
  import { SvelteQueryDevtools } from "@tanstack/svelte-query-devtools";
  import WebsocketProvider from "$lib/components/WebsocketProvider.svelte";

  import Pages from "./Pages.svelte";
</script>

<QueryClientProvider client={queryClient}>
  <WebsocketProvider>
    <main class="main" id="mainContent">
      <Pages />
    </main>
  </WebsocketProvider>

  <AppToaster />
  <SvelteQueryDevtools />
</QueryClientProvider>

<style>
  .main {
    height: 100%;
    position: relative;
    overflow: hidden;
  }
</style>
