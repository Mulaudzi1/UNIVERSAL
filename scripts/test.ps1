$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $PSScriptRoot
Push-Location $root
try {
    cargo fmt --all -- --check
    cargo clippy --workspace --all-targets
    cargo test --workspace --all-targets
} finally {
    Pop-Location
}
