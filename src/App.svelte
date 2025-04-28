<script lang="ts">
  import { queryClient } from "$lib/api/client";
  import "$lib/styles/app.css";
  import "@fontsource-variable/roboto";
  import AppToaster from "$lib/components/AppToaster.svelte";
  import { QueryClientProvider } from "@tanstack/svelte-query";
  import I18nProvider from "$lib/components/i18n/I18nProvider.svelte";
  import { SvelteQueryDevtools } from "@tanstack/svelte-query-devtools";
  import SettingsProvider from "$lib/components/SettingsProvider.svelte";
  import WebsocketProvider from "$lib/components/WebsocketProvider.svelte";

  import Pages from "./Pages.svelte";
</script>

<QueryClientProvider client={queryClient}>
  <SettingsProvider>
    <I18nProvider>
      <WebsocketProvider>
        <main class="main" id="mainContent">
          <Pages />
        </main>
      </WebsocketProvider>

      <AppToaster />
    </I18nProvider>
  </SettingsProvider>

  <SvelteQueryDevtools buttonPosition="bottom-left" position="bottom" />
</QueryClientProvider>

<style>
  .main {
    height: 100%;
    position: relative;
    overflow: hidden;
  }
</style>
