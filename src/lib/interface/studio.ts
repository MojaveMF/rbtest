import { invoke } from "@tauri-apps/api";
import { z } from "zod";

const StudiosValidator = z.record(z.string().length(4), z.string().url());

let StudioCache: { [key: string]: string } | undefined;

export async function GetStudios(): Promise<{ [key: string]: string }> {
  if (StudioCache === undefined) {
    StudioCache = await invoke("get_available_studio").then((data) =>
      StudiosValidator.parseAsync(data)
    );
    return GetStudios();
  }
  return StudioCache;
}

export async function InstallStudio(year: string) {
  if (await StudioInstalled(year)) return;

  let url = (await GetStudios())[year];
  if (url === undefined) throw "Bad version";

  await invoke("install_studio", { year, url });
}

export async function StudioInstalled(year: string): Promise<boolean> {
  return await invoke("studio_installed", { year });
}
