<script lang="ts">
  import { GetBootstrapperInfo } from "./interface/index";
</script>

<footer class="footer_container">
  <span class="footer_text">
    {#await GetBootstrapperInfo()}
      Waiting for info
    {:then { base_url, compile_time, pkg_version }}
      base_url: {base_url}; compile_time: {compile_time}; version: {pkg_version}
    {:catch err}
      Failed to get bootstrapper info err = {err}
    {/await}
  </span>
</footer>
