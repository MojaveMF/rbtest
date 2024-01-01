import { invoke } from "@tauri-apps/api";
import { GetStudios, InstallStudio, LaunchStudio, StudioInstalled } from "./studio";
import { CreateShortcuts, CreateUri, GetLatestversion, GetValidClients, clientInstalled } from ".";
import Installer, { LaunchClient, SetTaskbar } from "./client";
import { exit } from "@tauri-apps/api/process";
let args_cache: string[] | undefined;
async function GetLaunchArguments(): Promise<string[]> {
  if (args_cache === undefined) {
    args_cache = await invoke("get_cli");
  }
  return args_cache!;
}

export async function IsStudio() {
  let args = await GetLaunchArguments();
  if (args.length < 3) {
    return false;
  }
  return args[1] === "--studio";
}

async function GetStudioVersions(): Promise<string[]> {
  return Object.keys(await GetStudios());
}

async function GetStudioLaunch(): Promise<string> {
  let launch_args = await GetLaunchArguments();
  let launched_version = launch_args[2];

  if (!(await GetStudioVersions()).includes(launched_version))
    throw `${launched_version} dose not have a studio`;
  return launched_version;
}

export type LaunchArguments = {
  launch_mode: string;
  auth_ticket: string;
  join_script: string;
  client_year: string;
};

let cachedParsed: LaunchArguments | undefined;

export async function GetPlayerLaunchArguments(): Promise<LaunchArguments> {
  if (cachedParsed !== undefined) return cachedParsed;
  let uri = (await GetLaunchArguments())[1];
  if (!uri || !uri.startsWith("syntax-player://")) throw "No player launch arguments";

  let uriArguments = uri.split("+");

  let launch_mode: string | undefined;
  let auth_ticket: string | undefined;
  let join_script: string | undefined;
  let client_year: string | undefined;

  let validClients = GetValidClients();

  for (let arg of uriArguments) {
    let index = arg.indexOf(":");
    let first = arg.substring(0, index),
      last = arg.substring(index + 1);

    switch (first) {
      case "launchmode": {
        launch_mode = last;
        continue;
      }
      case "gameinfo": {
        auth_ticket = last;
        continue;
      }
      case "placelauncherurl": {
        join_script = last;
        continue;
      }
      case "clientyear": {
        client_year = last;
        continue;
      }
      default:
        continue;
    }
  }

  if (launch_mode === undefined) {
    throw "Launchmode undefined";
  } else if (auth_ticket === undefined) {
    throw "Authticket undefined";
  } else if (join_script === undefined) {
    throw "Joinscript undefined";
  } else if (client_year === undefined || !(await validClients).includes(client_year)) {
    throw "Client year undefined or invalid";
  }

  cachedParsed = {
    launch_mode,
    auth_ticket,
    join_script,
    client_year,
  };
  return GetPlayerLaunchArguments();
}

export async function GetLaunchedVersion(): Promise<string> {
  if (await IsStudio()) {
    return GetStudioLaunch();
  }
  return (await GetPlayerLaunchArguments()).client_year;
}

export async function HandleLaunch() {
  try {
    let launched_version = await GetLaunchedVersion();
    if (await IsStudio()) {
      SetTaskbar(`Studio ${launched_version} launched`, 0);
      if (await StudioInstalled(launched_version)) {
        SetTaskbar(`Studio ${launched_version} installed`, 100);
        await LaunchStudio(launched_version);
        return;
      }
      await InstallStudio(launched_version);
    } else {
      SetTaskbar(`Client ${launched_version} launched`, 0);
      let latest_version = await GetLatestversion();
      if (await clientInstalled(launched_version, latest_version)) {
        SetTaskbar(`Client ${launched_version} installed`, 0);
        await LaunchClient(launched_version, latest_version, await GetPlayerLaunchArguments());
        return;
      }
      let installer = new Installer(launched_version, latest_version, true);
      await installer.Download();
    }
    return HandleLaunch();
  } catch (err) {
    if (String(err) !== `No player launch arguments`) throw err;
    SetTaskbar(`Registering uri`, 0);
    await CreateUri();
    SetTaskbar(`Creating shortcuts`, 50);
    await CreateShortcuts(await GetStudioVersions());
    SetTaskbar(`Done closing...`, 100);
    setTimeout(async () => {
      await exit(0);
    }, 2000);
  }
}
