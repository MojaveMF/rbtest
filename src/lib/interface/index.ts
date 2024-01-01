import { invoke } from "@tauri-apps/api";

export * from "./studio";
export * from "./client";
export * from "./launch";

type BootstrapperInfo = {
  base_url: String;
  compile_time: String;
  pkg_version: String;
};

let cache: BootstrapperInfo | undefined;
export async function GetBootstrapperInfo(): Promise<BootstrapperInfo> {
  if (cache !== undefined) return cache;
  cache = await invoke("get_bootstrapper_info");
  return cache!;
}

export async function CreateShortcuts(studioVersions: string[]) {
  await invoke("create_shortcuts", { studioVersions });
}

export async function CreateUri() {
  await invoke("create_uri");
}
