<script lang="ts">
  import InitialView from "$lib/views/InitialView.svelte";
  import RevokedView from "$lib/views/RevokedView.svelte";
  import DeclinedView from "$lib/views/DeclinedView.svelte";
  import ConnectingView from "$lib/views/ConnectingView.svelte";
  import ServerTrustView from "$lib/views/ServerTrustView.svelte";
  import AuthenticatedView from "$lib/views/AuthenticatedView.svelte";
  import RequestingApproval from "$lib/views/RequestingApproval.svelte";
  import AuthenticatingView from "$lib/views/AuthenticatingView.svelte";
  import ConnectionLostView from "$lib/views/ConnectionLostView.svelte";
  import ConnectionFailedView from "$lib/views/ConnectionFailedView.svelte";
  import { getTilepadSocket } from "$lib/components/WebsocketProvider.svelte";

  const { state: stateRef, details: detailsRef } = getTilepadSocket();

  let state = $derived.by(stateRef);
  let details = $derived.by(detailsRef);
</script>

{#if state.type === "Initial"}
  <InitialView />
{:else if state.type === "Connecting"}
  <ConnectingView />
{:else if state.type === "RequestingApproval"}
  <RequestingApproval />
{:else if state.type === "Authenticating"}
  <AuthenticatingView />
{:else if state.type === "ConfirmChangedServerKey"}
  <ServerTrustView
    serverPublicKey={state.serverPublicKey}
    lastServerPublicKey={state.lastServerPublicKey}
    onConfirm={state.onConfirm}
    onCancel={state.onCancel}
  />
{:else if state.type === "Authenticated"}
  {#if details != null && state.folder !== null}
    <AuthenticatedView
      {details}
      folder={state.folder}
      tiles={state.tiles}
      deviceId={state.deviceId}
    />
  {/if}
{:else if state.type === "Declined"}
  <DeclinedView />
{:else if state.type === "Revoked"}
  <RevokedView />>
{:else if state.type === "ConnectionFailed"}
  <ConnectionFailedView />
{:else if state.type === "ConnectionLost"}
  <ConnectionLostView />
{/if}
