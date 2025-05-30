<script lang="ts">
  import {
    type DisplayContext,
    isDisplayContextEqual,
  } from "$lib/api/types/plugin";

  import type { DisplayMessage } from "./displayMessage";

  import DisplayFrame from "./DisplayFrame.svelte";
  import { getTilepadSocket } from "../WebsocketProvider.svelte";
  import DisplayPluginMessageListener from "./DisplayPluginMessageListener.svelte";

  type Props = {
    ctx: DisplayContext;
    inspector: string;
  };

  const { ctx, inspector }: Props = $props();

  type CurrentFrameData = {
    ctx: DisplayContext;
    send: (data: object) => void;
  };

  let currentFrame: CurrentFrameData | undefined;
  const { sendDisplayMessage } = getTilepadSocket();

  /**
   * Handle the current frame mounting
   *
   * @param ctx
   * @param send
   */
  function onFrameMount(ctx: DisplayContext, send: (data: object) => void) {
    currentFrame = { ctx, send };
  }

  /**
   * Handle a message from the plugin to pass onto the inspector
   *
   * @param ctx
   * @param message
   */
  function onMessage(ctx: DisplayContext, message: object) {
    if (currentFrame && isDisplayContextEqual(ctx, currentFrame.ctx)) {
      currentFrame.send({
        type: "PLUGIN_MESSAGE",
        context: ctx,
        message,
      });
    }
  }

  /**
   * Handle a message from the inspector to pass onto the plugin
   *
   * @param ctx
   * @param message
   */
  function onSendToPlugin(ctx: DisplayContext, message: object) {
    sendDisplayMessage(ctx, message);
  }

  async function onGetTile(ctx: DisplayContext, send: (data: object) => void) {
    send({
      type: "TILE",
      tile: {
        pluginId: ctx.plugin_id,
        tileId: ctx.tile_id,
        actionId: ctx.action_id,
      },
    });
  }
  function onFrameEvent(
    ctx: DisplayContext,
    event: DisplayMessage,
    send: (data: object) => void,
  ) {
    switch (event.type) {
      case "SEND_TO_PLUGIN": {
        onSendToPlugin(ctx, event.message);
        break;
      }

      case "GET_TILE": {
        onGetTile(ctx, send);
        break;
      }
    }
  }
</script>

<DisplayPluginMessageListener {onMessage}>
  <DisplayFrame {onFrameEvent} {onFrameMount} {ctx} {inspector} />
</DisplayPluginMessageListener>
