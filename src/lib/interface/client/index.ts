import { path } from "@tauri-apps/api";
import { emit } from "@tauri-apps/api/event";
import { GetClientFolder, GetManifest, download_zip, extract_zip, prepare_client } from "./utility";

export * from "./utility";

export default class Installer {
  private Year: string;
  private Version: string;
  private Verbose: boolean;

  private Manifest: { [k: string]: string } | undefined;

  private async Taskbar(...args: Array<string | number>) {
    if (!this.Verbose) return;
    for (let arg of args) {
      emit("set_taskbar", arg);
    }
  }

  private async DownloadManifest() {
    this.Manifest = await GetManifest(this.Year);
  }

  private async PrepareDownload() {
    if (!this.Manifest) throw "Manifest undefined";
    await prepare_client(this.Year, this.Version, this.Manifest);
  }

  private async DownloadFiles(start_number = 10) {
    if (!this.Manifest) throw "Manifest undefined";

    let downloads = [];
    for (let key of Object.keys(this.Manifest)) {
      this.Taskbar(`Downloading ${key}`, (start_number += 2.5));
      downloads.push(download_zip(`${this.Version}-${key}`));
    }
    await Promise.all(downloads);
  }

  private async ExtractFiles(start_number = 60) {
    if (!this.Manifest) throw "Manifest undefined";

    let client_folder = await GetClientFolder(this.Year, this.Version);
    let extraction = [];

    for (let [filename, location] of Object.entries(this.Manifest)) {
      this.Taskbar(`Extracting ${filename}`, (start_number += 1.25));
      extraction.push(
        extract_zip(`${this.Version}-${filename}`, await path.join(client_folder, location))
      );
    }
    await Promise.all(extraction);
  }

  public async Download() {
    this.Taskbar("Downloading client manifest", 0);

    await this.DownloadManifest();

    this.Taskbar("Manifest downloaded. Preparing for download", 5);

    await this.PrepareDownload();

    this.Taskbar("Preparations finished downloading files", 10);

    await this.DownloadFiles();

    this.Taskbar("Downloads finished, extracing files", 60);

    await this.ExtractFiles();

    this.Taskbar("Download finished", 100);
  }

  constructor(year: string, version: string, verbose: boolean = true) {
    this.Year = year;
    this.Version = version;
    this.Verbose = verbose;
  }
}
