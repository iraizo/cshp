<h1 style="text-align: center;">CSHP</h1>

![Logo](assets/logo.png)

This is a cross-platform tool to historicize different branches/depots/manifests and generating pseudocode for it to compare different game updates.

## Roadmap

This project is WIP but will get updates frequently  
listing the features below.

- [x] Fetching depots from a steamDB API.
- [x] Download all the manifests of the depo.
- [x] Only download certain assemblies based on user configuration.
- [x] Dissasemble assemblies
  - [x] Dissasemble assemblies without hardcoding the names.
- [x] Safety of the code (anything that panics isn't a recoverable error).
- [ ] Support for multiple branches.
- [x] Support for uploading pseudo using git.
  - [x] Create different branches for different manifests.
  - [ ] Multithreaded downloading/dissasembly.
- [x] Remove hardcoded values.

### Support

This project supports x86_64/arm64 CPU's.  
The Operation systems/Kernels supported are: Linux, Windows,macOS.

If you need any manual help/issues please create an GitHub issue or contact me on [matrix](https://matrix.org) under `@raizo:matrix.org`.

### Requirements

- [DepotDownloader](https://github.com/SteamRE/DepotDownloader/releases/tag/DepotDownloader_2.4.5) in a `downloader` directory.
- [ILspycmd](https://github.com/icsharpcode/ILSpy/tree/master/ICSharpCode.Decompiler.Console) set the path in main.rs (hardcoding will be removed later)
- .NET 6.0 SDK/Runtime


### Usage

- Move `DepotDownloader` into the `downloader` folder.

#### Setting up the folders

- Create a download directory (Where the manifests are getting downloaded in).
- Create a pseudo directory (Where the psuedo files are getting dissasembled into).
- Create a repo directory (Where the git repository resides in).

#### Setting up the repository

- Run these commands in your favorite shell

```bash
cd repo_direcotry_name
git init
git remote add origin remote_url_here

```

- Create branches for all the manifests using either GitHub `git checkout -b branch_name_here main_manifest_branch`.
`main_manifest_branch` would be the latest game version/manifest. 

#### Setting up the config

- Rename `example_config.json` to `config.json`.
- Change the information in `config.json`.

#### Setting up the filter

- Change what assembly paths you want dissasembled in the filter.
- Change the name/path of your filter in `config.json`.

### Running

- Run `cargo run` in the root directory of this project.