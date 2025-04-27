<script lang="ts">
  import type { TileLabel } from "$lib/api/types/tiles";

  import { useFontLoader } from "$lib/api/fonts";

  import { getServerContext } from "../ServerProvider.svelte";

  type Props = {
    label: TileLabel;
  };

  const { label }: Props = $props();
  const serverContext = getServerContext();
  const serverURL = $derived(serverContext.serverURL);

  const options = $derived({
    serverURL,
    fontName: label.font,
    bold: label.bold,
    italic: label.italic,
  });

  const fontQuery = useFontLoader(() => options);
</script>

{#if label.enabled && label.label.length > 0}
  <p
    class="label"
    style="font-size: calc({label.font_size}pt * var(--tile-size-adjustment)); font-family: {label.font}, 'Roboto'; color: {label.color}; --outline-color: {label.outline_color};"
    data-align={label.align}
    class:label--bold={label.bold}
    class:label--italic={label.italic}
    class:label--underline={label.underline}
    class:label--outline={label.outline}
  >
    {label.label}

    <!-- Hack required to make sure the font query is marked as active -->
    {#if $fontQuery.isSuccess}{/if}
  </p>
{/if}

<style>
  .label {
    position: absolute;
    text-align: center;
    font-weight: normal;
  }

  .label[data-align="Bottom"] {
    bottom: 5%;
  }

  .label[data-align="Middle"] {
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
  }

  .label[data-align="Top"] {
    top: 5%;
  }

  .label--bold {
    font-weight: bold;
  }

  .label--italic {
    font-style: italic;
  }

  .label--underline {
    text-decoration: underline;
  }

  .label--outline {
    text-shadow:
      -1px -1px 0 var(--outline-color),
      1px -1px 0 var(--outline-color),
      -1px 1px 0 var(--outline-color),
      1px 1px 0 var(--outline-color);
  }
</style>
