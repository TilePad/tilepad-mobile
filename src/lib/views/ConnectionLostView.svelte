<script lang="ts">
  import { i18nContext } from "$lib/i18n/i18n.svelte";
  import Button from "$lib/components/input/Button.svelte";
  import { getTilepadSocket } from "$lib/components/WebsocketProvider.svelte";

  const i18n = i18nContext.get();

  const { reconnect, disconnect } = getTilepadSocket();
</script>

<div class="layout">
  <div class="modal">
    <h1>{i18n.f("connection_lost")}</h1>
    <p>{i18n.f("connection_lost_desc")}</p>
    <div class="actions">
      <Button onclick={disconnect}>{i18n.f("back")}</Button>
      <Button onclick={reconnect}>{i18n.f("reconnect")}</Button>
    </div>
  </div>
</div>

<style>
  .layout {
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;

    flex-flow: column;
  }

  .modal {
    display: flex;
    flex-flow: column;
    gap: 0.5rem;
    align-items: center;
  }

  h1 {
    color: #fff;
  }

  p {
    color: #ccc;
    margin-bottom: 0.5rem;
  }

  .actions {
    display: flex;
    flex-flow: row;
    gap: 1rem;
  }
</style>
