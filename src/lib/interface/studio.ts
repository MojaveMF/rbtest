import { invoke } from "@tauri-apps/api";
import { z } from "zod";
import { SetTaskbar } from ".";
import { exit } from "@tauri-apps/api/process";

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

  /* I have no way to actually monitor this */
  await SetTaskbar("Installing studio", 10);
  setTimeout(async () => SetTaskbar("Installing studio", 50), 1000);

  await invoke("install_studio", { year, url });
}

export async function StudioInstalled(year: string): Promise<boolean> {
  return await invoke("studio_installed", { year });
}

/**
 *
 * WARNING THIS WILL TERMINATE THE PROCESS
 */
export async function LaunchStudio(year: string) {
  await invoke("launch_studio", { year });

  await SetTaskbar("Studio launched", 100);
  setTimeout(async () => {
    await exit(0);
  }, 3000);
}
