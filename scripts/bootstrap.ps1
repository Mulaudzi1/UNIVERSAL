$ErrorActionPreference = "Stop"
if (-not (Get-Command rustup -ErrorAction SilentlyContinue)) {
    throw "Rustup is required. Install Rust from https://rustup.rs and rerun this script."
}
$root = Split-Path -Parent $PSScriptRoot
Push-Location $root
try {
    rustup show
    rustup component add rustfmt clippy
    cargo --version
    rustc --version
    cargo test --workspace --all-targets
} finally {
    Pop-Location
}
