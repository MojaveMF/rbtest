<script lang="ts">
  import { listen } from "@tauri-apps/api/event";

  let progress = 0;
  let progress_msg = "Taking the cat out the hat";

  listen("set_taskbar", (event) => {
    console.log("Taskbar Update", event);
    if (typeof event.payload === "string") {
      progress_msg = event.payload;
    }
    if (typeof event.payload === "number") {
      progress = event.payload;
    }
  });
</script>

<div>
  <div class="bar_holder">
    <div class="load_bar">
      <div class="progress" style="width: {progress}%;"></div>
    </div>
  </div>
  <div class="bar_holder">
    <p class="bang-line">{progress_msg}</p>
  </div>
</div>
