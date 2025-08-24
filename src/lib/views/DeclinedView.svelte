<script lang="ts">
  import { i18nContext } from "$lib/i18n/i18n.svelte";
  import Button from "$lib/components/input/Button.svelte";
  import { getTilepadSocket } from "$lib/components/WebsocketProvider.svelte";

  const i18n = i18nContext.get();

  const { disconnect } = getTilepadSocket();
</script>

<div class="layout">
  <div class="modal">
    <h1>{i18n.f("declined")}</h1>
    <p>{i18n.f("declined_desc")}</p>
    <Button onclick={disconnect}>{i18n.f("back")}</Button>
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
</style>
