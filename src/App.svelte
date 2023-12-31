<script lang="ts">
  import { onMount } from "svelte";
  import Footer from "./lib/footer.svelte";
  import {
    GetLaunchArguments,
    joinGame,
    installed,
  } from "./lib/interface/index";
  import Loading from "./lib/loading.svelte";
  import {
    Installer,
    RegisterURI,
    getTargets,
    latestVersion,
  } from "./lib/interface/installer";
  import { emit } from "@tauri-apps/api/event";

  let failure: string | undefined;

  async function onStart(depth = 0) {
    try {
      /* We always need the version */
      await RegisterURI();

      let version = await latestVersion();
      if (await installed()) {
        console.log("ALREADY INSTALLED LATEST VERSION");
        await emit("set_taskbar", "Already installed");
        await emit("set_taskbar", 100);
        try {
          let launchArguments = await GetLaunchArguments();
          console.log(launchArguments.auth_ticket.length);
          await joinGame(version, launchArguments);
        } catch (err) {
          await emit("set_taskbar", "No launch arguments");
        }

        return;
      }

      let targets = await getTargets();
      let installer = new Installer(version, targets);
      installer.verbose = true;

      await installer.Install();

      if (depth == 0) return onStart(1);
    } catch (err) {
      failure = String(err);
    }
  }

  onMount(onStart);
</script>

<div class="drag_bar" data-tauri-drag-region></div>
<h1 class="logo">
  SYNTAX
  <p class="bang-line">experience joy with freedom</p>
</h1>

{#if failure === undefined}
  <Loading />
{:else}
  <center>
    <p>Uncaught exception</p>
    <div class="bang-line">{failure}</div>
  </center>
{/if}
<Footer />
