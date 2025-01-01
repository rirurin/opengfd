# globals
$cs_project = "metaphor.opengfd"
$rust_project = "opengfd-reloaded"
$rust_project_underscore = "opengfd_reloaded"
$target_triple = "x86_64-pc-windows-msvc"

# build Persona Multiplayer Rust project
Push-Location "./$rust_project"
$env:RUST_BACKTRACE = 1
$env:RUSTFLAGS = "-C panic=abort -C lto=fat -C embed-bitcode=yes -C target_cpu=native"
cargo +nightly rustc --lib --release -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --crate-type cdylib --target $target_triple
Pop-Location
# build Unreal Essentials
Push-Location "./$cs_project"
dotnet build "./$cs_project.csproj" -v q -c Debug 
Pop-Location
# copy required files from target
Push-Location "./target/$target_triple/release"
Copy-Item "$rust_project_underscore.dll" -Destination "$env:RELOADEDIIMODS\$cs_project"
Copy-Item "$rust_project_underscore.dll.lib" -Destination "$env:RELOADEDIIMODS\$cs_project"
Copy-Item "$rust_project_underscore.dll.exp" -Destination "$env:RELOADEDIIMODS\$cs_project"
Pop-Location
