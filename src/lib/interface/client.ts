import { invoke, path } from "@tauri-apps/api";
import { promise, z } from "zod";

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

async function GetClientFolder(year: string, version: string): Promise<string> {
  return await invoke("get_client_folder", { year, version });
}

async function download_zip(fileName: string) {
  try {
    return await invoke("download_zip", { fileName });
  } catch (err) {
    console.log(fileName, "Failed with", err);
    throw err;
  }
}

async function extract_zip(fileName: string, location: string) {
  console.log(fileName, location);
  try {
    return await invoke("extract_zip", { fileName, location });
  } catch (err) {
    console.log(fileName, location, "failed with", err);
    throw err;
  }
}

async function prepare_client(year: string, version: string, manifest: { [key: string]: string }) {
  return await invoke("prepare_client", { year, version, manifest });
}

export async function InstallClient(year: string) {
  let version = await GetLatestversion();
  let folder = await GetClientFolder(year, version);
  let manifest = await GetManifest(year);

  try {
    await prepare_client(year, version, manifest);
  } catch (err) {}

  let downloads = [];
  for (let key of Object.keys(manifest)) {
    downloads.push(download_zip(`${version}-${key}`));
  }
  await Promise.all(downloads);

  let extraction = [];
  for (let [filename, location] of Object.entries(manifest)) {
    extraction.push(extract_zip(`${version}-${filename}`, await path.join(folder, location)));
  }
  await Promise.all(extraction);
}
