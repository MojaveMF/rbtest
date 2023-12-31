# Syntax Launcher

![build_status](https://img.shields.io/github/actions/workflow/status/mojavemf/syntax/main.yaml
)

### This launcher will not work on outdated systems

## This is not the offical launcher

<details>
<summary>Why should i use this over the official launcher?</summary>

The offical launcher works fine but it is far from perfect.

The official launcher is flawed. Has no user interface and is destructive on non windows platforms.

I mean no offence to Austin but he clearly isnt a rust programmer since the project is badly formatted.

I have also improved upon the launcher in speed by running multiple download jobs in sync and same with the extraction providing a faster launch speed.

</details>

## How do i build?

Requirements:
 - [tauri](https://tauri.app/)
 - [rust / cargo](https://www.rust-lang.org/tools/install)
 - [nodejs and npm](https://nodejs.org/en/download/current)

### Instructions

1. Pull the project
`git clone https://github.com/MojaveMF/syntax.git`

2. Open the project in a cli `cd syntax`

3. Install dependencies `npm i`

4. Build the project `npm run tauri build`

