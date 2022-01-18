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
- [ ] Safety of the code.
- [ ] Support for multiple branches.
- [ ] Support for uploading pseudo using git.
  - [ ] Create different branches for different manifests.
  - [ ] Multithreaded downloading/dissasembly.
- [ ] Remove hardcoded values.

### Support
This project supports x86_64/arm64 CPU's.  
The Operation systems/Kernels supported are: Linux, Windows,macOS.

If you need any manual help/issues please create an GitHub issue or contact me on [matrix](https://matrix.org) under `@raizo:matrix.org`.

### Requirements

- [DepotDownloader](https://github.com/SteamRE/DepotDownloader/releases/tag/DepotDownloader_2.4.5) in a `downloader` directory.
- [ILspycmd](https://github.com/icsharpcode/ILSpy/tree/master/ICSharpCode.Decompiler.Console) set the path in main.rs (hardcoding will be removed later)
- .NET 6.0 SDK/Runtime


### Usage

This will be written later due to me not recommending to use this right now, but all info is in Requirements if you want to test it.
