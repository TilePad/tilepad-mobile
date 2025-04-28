<script lang="ts">
  import { t } from "svelte-i18n";
  import Button from "$lib/components/input/Button.svelte";
  import { getTilepadSocket } from "$lib/components/WebsocketProvider.svelte";

  const { disconnect } = getTilepadSocket();
</script>

<div class="layout">
  <div class="modal">
    <h1>{$t("declined")}</h1>
    <p>{$t("declined_desc")}</p>
    <Button onclick={disconnect}>{$t("back")}</Button>
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
