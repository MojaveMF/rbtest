import { invoke } from "@tauri-apps/api";
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

export async function GetManifest(year: string): Promise<{ [key: string]: string }> {
  let validClients = await GetValidClients();
  if (!validClients.includes(year)) throw "Bad client year";

  let res = await invoke("get_client_manifest", { year });
  console.log(res);
  return await manifestSchema.parseAsync(res);
}
