import { z } from "zod";
import { invoke } from "@tauri-apps/api";

export * from "./launch";
export * from "./installer";

export type InfoReponseT = {
  compile_time: string;
  base_url: string;
  pkg_version: string;
};

const InfoResponse: z.ZodType<InfoReponseT> = z.object({
  compile_time: z.string(),
  base_url: z.string(),
  pkg_version: z.string(),
});

export async function GetBootstrapperInfo(): Promise<InfoReponseT> {
  let result = await invoke("info");
  console.log("BOOTSTRAP INFO: ", result);
  return InfoResponse.parse(result);
}
