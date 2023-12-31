import { invoke } from "@tauri-apps/api";
import { emit } from "@tauri-apps/api/event";
import { promise, z } from "zod";

const Targets = z.array(z.string());
const ZodString = z.string();

async function extractZip(version: string, name: string) {
  await invoke("extract_zip", { version, name });
}

async function downloadFile(version: string, name: string) {
  await invoke("download_to_zip", { version, name });
}

async function createDirectorys(version: string) {
  await invoke("create_directorys", { version });
}

export async function getTargets(): Promise<Array<string>> {
  return Targets.parse(await invoke("get_targets"));
}

export async function latestVersion(): Promise<string> {
  return ZodString.parse(await invoke("latest_version"));
}

export async function GenerateAppsettings(version: string) {
  await invoke("generate_appsettings", { version });
}

export async function installed(): Promise<boolean> {
  try {
    let res = await invoke("is_installed");
    if (typeof res !== "boolean") {
      throw "Bad value from invoke";
    }
    return res;
  } catch (err) {
    return false;
  }
}

export async function RegisterURI() {
  await invoke("register_uri");
}

export class Installer {
  private version: string;
  private targets: Array<string>;
  private downloaded: Array<string> = [];
  private extracted: Array<string> = [];
  private downloadPercent: number = 0;
  public verbose: boolean = false;

  private async setTaskbar(...args: Array<string | number>) {
    if (!this.verbose) return;
    for (let arg of args) {
      await emit("set_taskbar", arg);
    }
  }

  private hasTarget(name: string): boolean {
    for (let testName of this.targets) {
      if (name === testName) {
        return true;
      }
    }
    return false;
  }

  private async extractZip(name: string) {
    if (this.extracted.includes(name) || !this.hasTarget(name)) {
      return;
    }

    this.setTaskbar(`Extracting ${name}`, this.downloadPercent);
    await extractZip(this.version, name);
    this.downloadPercent += 2;
    this.setTaskbar(`Extracted ${name}`, this.downloadPercent);
    this.extracted.push(name);
  }

  private async downloadFile(name: string) {
    if (this.downloaded.includes(name) || !this.hasTarget(name)) {
      return;
    }
    this.setTaskbar(`Downloading ${name}`, this.downloadPercent);
    await downloadFile(this.version, name);
    this.downloadPercent += 3;
    this.setTaskbar(`Downloaded ${name}`, this.downloadPercent);
    this.downloaded.push(name);
  }

  public async DownloadFiles() {
    let downloads = [];
    for (let name of this.targets) {
      downloads.push(this.downloadFile(name));
    }
    await Promise.all(downloads);
  }

  public async ExtractFiles() {
    let extracted = [];
    for (let name of this.downloaded) {
      extracted.push(this.extractZip(name));
    }
    await Promise.all(extracted);
  }

  public async Install() {
    this.setTaskbar("Creating directorys", 0);
    await createDirectorys(this.version);

    this.setTaskbar("Generating AppSettings.xml", 5);
    await GenerateAppsettings(this.version);

    this.setTaskbar("Begining downloads", 10);
    await this.DownloadFiles();

    this.setTaskbar("Extracting Downloads");
    await this.ExtractFiles();
    this.setTaskbar("Finished!", 100);
  }

  constructor(version: string, targets: Array<string>) {
    this.targets = Targets.parse(targets);
    this.version = version;
  }
}
