<script lang="ts">
  import "$lib/styles/app.css";

  let { children } = $props();
</script>

<main class="main">
  {@render children()}
</main>

<style>
  .main {
    height: 100%;
    position: relative;
    overflow: hidden;
  }
</style>
