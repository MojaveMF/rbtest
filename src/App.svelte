<script lang="ts">
  import Footer from "./lib/footer.svelte";
  import Loading from "./lib/loading.svelte";
  import { HandleLaunch } from "./lib/interface/launch";
</script>

<div class="drag_bar" data-tauri-drag-region></div>
<h1 class="logo">
  SYNTAX
  <p class="bang-line">experience joy with freedom</p>
</h1>
<Loading />
{#await HandleLaunch() catch error}
  <center>
    <p>Uncaught exception</p>
    <div class="bang-line">{error}</div>
  </center>
{/await}

<Footer />
