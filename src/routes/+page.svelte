<script lang="ts">
  import InitialView from "$lib/views/InitialView.svelte";
  import RevokedView from "$lib/views/RevokedView.svelte";
  import DeclinedView from "$lib/views/DeclinedView.svelte";
  import ConnectingView from "$lib/views/ConnectingView.svelte";
  import AuthenticatedView from "$lib/views/AuthenticatedView.svelte";
  import RequestingApproval from "$lib/views/RequestingApproval.svelte";
  import AuthenticatingView from "$lib/views/AuthenticatingView.svelte";
  import { getTilepadSocket } from "$lib/components/WebsocketProvider.svelte";

  const { state: stateRef, socket: socketRef } = getTilepadSocket();

  let state = $derived.by(stateRef);
  let socket = $derived.by(socketRef);
</script>

{#if state.type === "Initial"}
  <InitialView />
{:else if state.type === "Connecting"}
  <ConnectingView />
{:else if state.type === "RequestingApproval"}
  <RequestingApproval />
{:else if state.type === "Authenticating"}
  <AuthenticatingView />
{:else if state.type === "Authenticated"}
  {#if socket != null && state.folder !== null}
    <AuthenticatedView
      details={socket.details}
      folder={state.folder}
      tiles={state.tiles}
    />
  {/if}
{:else if state.type === "Declined"}
  <DeclinedView />
{:else if state.type === "Revoked"}
  <RevokedView />>
{/if}
