import { invoke } from "@tauri-apps/api";
import { emit } from "@tauri-apps/api/event";
import { z } from "zod";

let validClientsCache: Array<string> | undefined;
let manifestSchema = z.record(z.string().endsWith(".zip"), z.string());

export async function GetValidClients(): Promise<Array<string>> {
  if (validClientsCache === undefined) {
    validClientsCache = await invoke("get_valid_clients");
    return GetValidClients();
  }
  return validClientsCache;
}

let manifestCache: Map<string, { [key: string]: string }> = new Map();

export async function GetManifest(year: string): Promise<{ [key: string]: string }> {
  let validClients = await GetValidClients();
  if (!validClients.includes(year)) throw "Bad client year";
  if (manifestCache.has(year)) return manifestCache.get(year)!;

  let res = await invoke("get_client_manifest", { year });
  manifestCache.set(year, await manifestSchema.parseAsync(res));

  return GetManifest(year);
}

let latest_version: string | undefined;
export async function GetLatestversion(): Promise<string> {
  if (latest_version) return latest_version;
  latest_version = await invoke("get_latest_version");
  return GetLatestversion();
}

export async function GetClientFolder(year: string, version: string): Promise<string> {
  return await invoke("get_client_folder", { year, version });
}

export async function download_zip(fileName: string) {
  try {
    return await invoke("download_zip", { fileName });
  } catch (err) {
    console.log(fileName, "Failed with", err);
    throw err;
  }
}

export async function extract_zip(fileName: string, location: string) {
  console.log(fileName, location);
  try {
    return await invoke("extract_zip", { fileName, location });
  } catch (err) {
    console.log(fileName, location, "failed with", err);
    throw err;
  }
}

export async function prepare_client(
  year: string,
  version: string,
  manifest: { [key: string]: string }
) {
  console.log(manifest);
  return await invoke("prepare_client", { year, version, manifest });
}

export async function clientInstalled(year: string, version: string): Promise<boolean> {
  return await invoke("client_installed", { year, version });
}

export async function SetTaskbar(...args: Array<string | number>) {
  for (let arg of args) {
    await emit("set_taskbar", arg);
  }
}
