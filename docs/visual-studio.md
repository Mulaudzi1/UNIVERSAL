# Visual Studio Development

UNIVERSAL is implemented in Rust. `Universal.sln` exists for developers who prefer Visual Studio 2022 on Windows.

## Requirements

1. Visual Studio 2022 with **Desktop development with C++** installed. The solution uses Makefile-style `.vcxproj` projects so Visual Studio delegates builds to Cargo.
2. Rust installed through `rustup` with the stable toolchain.
3. `cargo`, `rustc`, `rustfmt`, and `clippy` available on `PATH`.

Run `scripts/bootstrap.ps1` from PowerShell after installing Rust.

## Open the solution

Open:

```text
Universal.sln
```

The solution contains:

- `Universal.Compiler` — builds the real `universal-compiler` Rust crate.
- `Universal.Cli` — builds the real `universal` CLI crate and depends on the compiler project.
- `Universal.Tests` — executes `cargo test --workspace --all-targets`.
- `Repository` — solution items such as the README, changelog, security policy, and workspace manifests.

## Build configurations

- **Debug | x64** invokes normal Cargo debug builds.
- **Release | x64** invokes `cargo build --release` for compiler and CLI projects.

`Build > Build Solution` therefore builds the actual Rust implementation. The Visual Studio projects are orchestration/project-system files only; they do not duplicate or replace the Rust source.

## Running UNIVERSAL

After a Debug build:

```powershell
.\target\debug\universal.exe run .\examples\first.univ
```

Or:

```powershell
cargo run -p universal -- run examples/first.univ
```

## Testing

Build the `Universal.Tests` project, or execute:

```powershell
.\scripts\test.ps1
```
