import { z } from "zod";
import { invoke } from "@tauri-apps/api";

export async function joinGame(
  version: string,
  { client_year, auth_ticket, join_script }: LaunchArguments
) {
  await invoke("join_game", {
    version: version,
    clientYear: client_year,
    authTicket: auth_ticket,
    joinScript: join_script,
  });
}

const launchArguments = z.array(z.string()).min(2);
const clientYears = ["2020", "2018", "2016", "2014"] as const;
const clientYear = z.enum(clientYears);

export type LaunchArguments = {
  launch_mode: string;
  auth_ticket: string;
  join_script: string;
  client_year: (typeof clientYears)[number];
};

export async function GetLaunchArguments(): Promise<LaunchArguments> {
  const matches = await invoke("get_launch");
  const valid = launchArguments.parse(matches);

  const args = valid[1].replace("syntax-player://", "").split("+");

  let launch_mode: string | undefined;
  let auth_ticket: string | undefined;
  let join_script: string | undefined;
  let client_year: (typeof clientYears)[number] | undefined;

  console.log("PARSINGS ", args);

  for (let arg of args) {
    let index = arg.indexOf(":");
    let first = arg.substring(0, index),
      last = arg.substring(index + 1);

    console.log([first, last]);
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
        client_year = clientYear.parse(last);
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
  } else if (client_year === undefined) {
    throw "Client year undefined";
  }

  console.log(args);

  return {
    launch_mode,
    auth_ticket,
    join_script,
    client_year,
  };
}
