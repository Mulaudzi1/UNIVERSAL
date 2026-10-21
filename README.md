# UNIVERSAL

[![CI](https://github.com/Mulaudzi1/UNIVERSAL/actions/workflows/ci.yml/badge.svg)](https://github.com/Mulaudzi1/UNIVERSAL/actions/workflows/ci.yml)
![Rust](https://img.shields.io/badge/Rust-stable-000000?logo=rust&logoColor=white)
![Status](https://img.shields.io/badge/status-v0.1%20experimental-orange)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)

UNIVERSAL is an experimental, deterministic, intent-oriented general-purpose programming language. Its goal is to let developers express **what** software should do in a readable form without turning arbitrary English into executable code.

This repository is the V0.1 compiler/interpreter foundation. It is intentionally small, but it is structured as a real language implementation rather than a disposable parser.

## Why Rust

Rust is the implementation language because compiler infrastructure benefits from memory safety, explicit ownership, predictable performance, strong enums/pattern matching for syntax trees, cross-platform tooling, and the ability to grow into native/WASM backends without requiring a garbage-collected compiler runtime. The trade-off is a steeper contributor learning curve and more implementation ceremony than Python or TypeScript.

## V0.1 pipeline

```text
source -> lexer -> tokens -> parser -> AST -> semantic/type analysis -> interpreter
                                      |
                                      +-> future Universal IR -> target backends
```

The IR boundary is architectural in V0.1; lowering into a standalone IR is scheduled after the semantic model stabilizes. This avoids prematurely freezing a poor IR while preserving the compiler boundary.

## Syntax choice

Blocks end with `END`. Indentation is strongly recommended by the formatter style but is not semantic. This keeps source readable while avoiding indentation-sensitive grammar in the bootstrap compiler.

```universal
ENTITY Employee
    name: Text
    scorecard: Scorecard?
END

employee = Employee(
    name: "John"
)

WHEN employee has a scorecard
    print "Employee has a scorecard"
OTHERWISE
    validate "Employee must have a scorecard"
END
```

`employee has a scorecard` is not interpreted by AI. The parser recognizes the formal `has-condition` production and semantic analysis verifies that `Employee` actually declares `scorecard`.

## Build and run

Prerequisite: Rust stable (edition 2021) and Cargo.

```bash
cargo test --workspace
cargo run -p universal -- check examples/first.univ
cargo run -p universal -- run examples/first.univ
```

Expected V0.1 output for `first.univ` is written to stderr as a structured validation event:

```text
validation: Employee must have a scorecard
```

Other commands:

```bash
cargo run -p universal -- build examples/first.univ
cargo run -p universal -- test
```

`format` and `repl` are reserved by the CLI but intentionally return “not implemented” in V0.1.

## Repository

- `compiler/` – lexer, parser, AST, semantic analysis, type model, interpreter, diagnostics
- `cli/` – `universal` command-line executable
- `examples/` – executable `.univ` programs
- `docs/` – language specification, architecture, roadmap, feature docs
- `runtime/` – future host/runtime services boundary
- `standard-library/` – future standard library
- `packages/` – future package tooling
- `rfc/` – language change proposals

## Scope

V0.1 supports entity declarations, primitive values, optional entity properties, variables, entity construction, `WHEN` / `ELSE WHEN` / `OTHERWISE`, `AND` / `OR` / `NOT`, property access, readable `HAS` / `IS` conditions, functions, returns, validation events, action phrases, comments, arithmetic, and console output.

Lists, maps, money/date literals, modules, events, loops, structured validation declarations, a standalone IR, formatter, REPL, FFI, database/web/AI layers, and code-generation backends remain future work. Their type/runtime extension points are documented rather than faked.

## License

Licensed under the [Apache License 2.0](LICENSE): permissive for commercial and open-source use, with an explicit patent grant.

## Visual Studio 2022

Windows developers can open **`Universal.sln`** at the repository root. The solution uses Visual Studio Makefile projects to invoke Cargo directly, so the Rust compiler remains the single source of truth.

Prerequisites:

- Visual Studio 2022 with the **Desktop development with C++** workload
- Rust installed using `rustup`
- stable `cargo`, `rustc`, `rustfmt`, and `clippy`

Bootstrap and verify the machine:

```powershell
.\scripts\bootstrap.ps1
```

Then open `Universal.sln` and use `Build > Build Solution`. See [docs/visual-studio.md](docs/visual-studio.md) for details.

## Engineering gates

The repository now includes the following production-engineering gates around the pre-1.0 language implementation:

- pinned stable Rust toolchain configuration
- formatting checks with `rustfmt`
- static linting with `clippy`
- workspace unit/integration tests
- Debug and Release build paths
- GitHub Actions validation on Linux, Windows, and macOS
- security policy and responsible disclosure guidance
- changelog and RFC-based contribution model
- PowerShell and shell build/test scripts
- Visual Studio orchestration without duplicating compiler implementation

UNIVERSAL **0.1 remains a language foundation**, not a production-stable language specification. Production-grade repository practices do not make unfinished language semantics stable. Database, network, filesystem, concurrency, package signing, sandboxing, FFI, WASM/native backends, formatter/LSP, and compatibility guarantees must reach their roadmap gates before a 1.0 production-runtime claim is appropriate.
