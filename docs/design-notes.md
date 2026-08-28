# Design Notes and Reality Check

## Requirement tensions resolved

- **Human-readable vs deterministic:** only a closed grammar is executable. Articles `a`/`an` are optional grammar noise in `HAS`; arbitrary English is rejected.
- **Indentation vs reliability:** indentation is style; `END` defines block structure.
- **Actions vs magic:** free-looking action phrases parse deterministically but V0.1 merely records them. Future typed action registries must define signatures/effects.
- **Strong typing vs beginner syntax:** entities are nominal and optionality explicit. Progressive sugar may lower to the same typed AST rather than creating separate language levels.
- **IR now vs premature design:** preserve a lowering boundary now; stabilize concrete IR after semantic tests establish behavior.
- **Exceptions vs Results:** expected failure should use `Result`; validation is data; unrecoverable runtime faults are separate.

## Inspiration

Python contributes readability; Rust contributes safety and explicitness; Go contributes tooling simplicity; TypeScript/Kotlin/Swift/C# contribute pragmatic type ergonomics; SQL contributes declarative intent; Haskell contributes explicit effects/type reasoning; Lisp demonstrates small-core extensibility; Elixir demonstrates message/event-oriented design; LLVM and WebAssembly inform backend boundaries. UNIVERSAL should borrow principles, not clone any one syntax or runtime.

## Hard problems

Interoperability and runtime ABI stability, generic/effect type design, deterministic natural-looking syntax, flow-sensitive typing, safe concurrency, database impedance mismatch, browser/native capability parity, package supply-chain security, debugger/source-map quality, and backward-compatible language evolution are major engineering programs rather than small features.
