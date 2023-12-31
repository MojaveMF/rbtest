import "./styles.css";
import App from "./App.svelte";
import { window } from "@tauri-apps/api";

/* Prevent the right click menu since they could cause issues */
//document.addEventListener("contextmenu", (event) => event.preventDefault());

/* Load the app */
const app = new App({
  target: document.getElementById("app") as any,
});

export default app;
