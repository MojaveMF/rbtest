<script lang="ts">
  import { onMount } from "svelte";
  import Footer from "./lib/footer.svelte";
  import { invoke } from "@tauri-apps/api";
  import {
    GetManifest,
    GetStudios,
    GetValidClients,
    InstallStudio,
    StudioInstalled,
  } from "./lib/interface/index";

  onMount(async () => {
    console.log(await GetManifest("2020"));
  });
</script>

<div class="drag_bar" data-tauri-drag-region></div>
<h1 class="logo">
  SYNTAX
  <p class="bang-line">experience joy with freedom</p>
</h1>

<!--
{#if failure === undefined}
  <Loading />
{:else}
  <center>
    <p>Uncaught exception</p>
    <div class="bang-line">{failure}</div>
  </center>
{/if}
-->
<Footer />
