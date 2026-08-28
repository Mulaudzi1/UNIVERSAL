$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $PSScriptRoot
Push-Location $root
try {
    cargo run -p universal -- run examples/first.univ
} finally {
    Pop-Location
}
