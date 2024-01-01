<script lang="ts">
  import { onMount } from "svelte";
  import Footer from "./lib/footer.svelte";
  import { invoke } from "@tauri-apps/api";
  import { GetStudios, InstallStudio, StudioInstalled } from "./lib/interface/index";
  import Installer, { GetLatestversion, GetValidClients } from "./lib/interface/client";
  import Loading from "./lib/loading.svelte";

  onMount(async () => {
    let installer = new Installer("2020", await GetLatestversion());
    await installer.Download();
  });
</script>

<div class="drag_bar" data-tauri-drag-region></div>
<h1 class="logo">
  SYNTAX
  <p class="bang-line">experience joy with freedom</p>
</h1>

<Loading />

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
