<script lang="ts">
  import { queryClient } from "$lib/api/client";
  import "$lib/styles/app.css";
  import "@fontsource-variable/roboto";
  import AppToaster from "$lib/components/AppToaster.svelte";
  import { QueryClientProvider } from "@tanstack/svelte-query";
  import { createI18n, i18nContext } from "$lib/i18n/i18n.svelte";
  import SettingsLoader from "$lib/components/SettingsLoader.svelte";
  import I18nProvider from "$lib/components/i18n/I18nProvider.svelte";
  import { SvelteQueryDevtools } from "@tanstack/svelte-query-devtools";
  import SettingsProvider from "$lib/components/SettingsProvider.svelte";
  import WebsocketProvider from "$lib/components/WebsocketProvider.svelte";

  import Pages from "./Pages.svelte";

  const i18n = createI18n();
  i18nContext.set(i18n);
</script>

<QueryClientProvider client={queryClient}>
  <SettingsLoader>
    {#snippet children({ settings })}
      <SettingsProvider {settings}>
        <I18nProvider locale={settings.language}>
          <WebsocketProvider>
            <main class="main" id="mainContent">
              <Pages />
            </main>
          </WebsocketProvider>

          <AppToaster />
        </I18nProvider>
      </SettingsProvider>
    {/snippet}
  </SettingsLoader>

  <SvelteQueryDevtools buttonPosition="bottom-left" position="bottom" />
</QueryClientProvider>

<style>
  .main {
    height: 100%;
    position: relative;
    overflow: hidden;
  }
</style>
