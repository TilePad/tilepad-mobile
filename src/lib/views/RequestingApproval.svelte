<script lang="ts">
  import { t } from "svelte-i18n";
  import { getDeviceName } from "$lib/api/devices";
  import Button from "$lib/components/input/Button.svelte";
  import PulseLoader from "$lib/components/PulseLoader.svelte";
  import { getTilepadSocket } from "$lib/components/WebsocketProvider.svelte";

  const { details: detailsRef, disconnect } = getTilepadSocket();
  const details = $derived.by(detailsRef);
</script>

<div class="layout">
  <div class="modal">
    <PulseLoader />

    {#if details !== null}
      <p class="host">{details.host}:{details.port}</p>
    {/if}

    <h1>{$t("requesting_approval")}</h1>

    <p>
      {$t("requesting_approval_device")}

      {#await getDeviceName() then hostname}
        <span class="device-name">{hostname}</span>
      {/await}
    </p>

    <Button onclick={disconnect}>{$t("cancel")}</Button>
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

  .host {
    color: #999;
    margin-top: 1rem;
  }

  .modal {
    display: flex;
    flex-flow: column;
    gap: 0.25rem;
    align-items: center;
  }

  h1 {
    color: #fff;
  }

  p {
    color: #ccc;
    margin-bottom: 0.5rem;
  }

  .device-name {
    color: #fcfcfc;
    font-weight: bold;
  }
</style>
