# OpenGFD

[INSERT BANNER HERE]

A Rust based reimplementation of Atlus' GFD Engine, used in Persona 5 (Royal), Metaphor: Refantazio, Catherine Full Body and the Persona Dancing games. This project is currently made up of a set of components from GFD which can be used to interoperate with an existing GFD game.
As such, the main use of this crate is as a Reloaded-II mod and dependency.

[TODO]

## Building

### Requirements

- **Rust Nightly**. This can be installed by downloading the [Rust toolchain](https://www.rust-lang.org/tools/install), then entering `rustup toolchain install nightly` in your terminal.
- [**.NET 9.0**](https://dotnet.microsoft.com/en-us/download/dotnet/9.0). 9.0.4 or earlier + it's respective SDK should be installed due to runtime issues in 9.0.5 causing crashes in Reloaded-II.
- [**Powershell**](https://learn.microsoft.com/en-us/powershell/scripting/install/installing-powershell). Used for the build script.
- [OpenGFD](https://github.com/rirurin/opengfd) and [cri-adx-rs](https://github.com/rirurin/cri-adx-rs) should also be cloned.
These are required since they are referenced within the build script by looking up their path within `env.local.yaml`.

### Building

For **Metaphor: Refantazio**, run `BuildMetaphor.ps1` in Powershell.

For **Persona 5 Royal**, run `BuildP5R.ps1` in Powershell.

## Contributions

[TODO]

## Credits and Licenses

[TODO]