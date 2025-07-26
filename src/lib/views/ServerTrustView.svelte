<script lang="ts">
  import { t } from "svelte-i18n";
  import Button from "$lib/components/input/Button.svelte";
  import PulseLoader from "$lib/components/PulseLoader.svelte";
  import { getTilepadSocket } from "$lib/components/WebsocketProvider.svelte";

  type Props = {
    serverPublicKey: number[];
    lastServerPublicKey: number[] | null;

    onConfirm: VoidFunction;
    onCancel: VoidFunction;
  };

  const { serverPublicKey, lastServerPublicKey, onConfirm, onCancel }: Props =
    $props();

  const { details: detailsRef } = getTilepadSocket();
  const details = $derived.by(detailsRef);

  async function fingerprint(keyBytes: Uint8Array) {
    const hash = await crypto.subtle.digest("SHA-256", keyBytes);
    const hashArray = Array.from(new Uint8Array(hash));
    return hashArray.map((b) => b.toString(16).padStart(2, "0")).join(":");
  }
</script>

<div class="layout">
  <div class="modal">
    <PulseLoader />

    {#if details !== null}
      <p class="host">{details.host}:{details.port}</p>
    {/if}

    <h1>{$t("server_key_changed_title")}</h1>
    <p>{$t("server_key_changed")}</p>

    {#await fingerprint(new Uint8Array(serverPublicKey)) then print}
      <p class="print">{$t("new")}:<br />{print}</p>
    {/await}

    {#if lastServerPublicKey}
      {#await fingerprint(new Uint8Array(lastServerPublicKey)) then print}
        <p class="print">{$t("previous")}:<br />{print}</p>
      {/await}
    {/if}

    <div class="actions">
      <Button onclick={onCancel}>
        {$t("cancel")}
      </Button>
      <Button variant="error" onclick={onConfirm}>
        {$t("trust_new_key")}
      </Button>
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

  .print {
    word-break: break-all;
    white-space: pre-wrap;
    font-family: monospace;
    padding: 0.5rem;
    max-width: 100%;
    overflow-wrap: break-word;
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

  .actions {
    display: flex;
    flex-flow: row;
    gap: 1rem;
  }
</style>
