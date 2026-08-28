# Compiler

The `compiler` crate owns language semantics and does not depend on the CLI. This is deliberate so IDEs, build tools, tests, servers, and future embedding APIs can call the compiler directly.

The bootstrap interpreter executes the checked AST. A standalone typed IR should be introduced after V0.1 semantic behavior is covered by regression tests, then the interpreter can execute IR rather than AST.

`compiler/src/ir.rs` contains the backend-neutral basic-block IR skeleton (`Instruction`, `Terminator`, `BasicBlock`, `IrFunction`). The bootstrap interpreter does not yet consume it; the next lowering phase should move execution from checked AST to typed IR only after conformance tests are stable.
