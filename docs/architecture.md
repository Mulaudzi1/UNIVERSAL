# Architecture

## Compiler boundaries

1. **Lexer** converts UTF-8 source text into tokens and source spans.
2. **Parser** recognizes the deterministic grammar and constructs an AST. It performs no business inference.
3. **Semantic analyzer** builds symbol information, resolves entity/property references, validates names, and performs the initial type checks.
4. **Universal IR** is the next compiler boundary. V0.1 intentionally keeps the interface conceptual while semantics settle; V0.5 should make lowering/backends production-grade.
5. **Interpreter** is the bootstrap execution backend.
6. **Runtime services** will host effectful capabilities (files, network, database, HTTP, messaging, AI) behind explicit capability interfaces.
7. **Target backends** will later lower IR to WASM, JavaScript, LLVM/native, or other ecosystems.

## No-magic rule

Intent syntax is grammar, not natural-language interpretation. `employee has a scorecard` lowers to an AST `Has(subject=employee, property=scorecard)`. Semantic analysis must prove that `scorecard` is a declared property.

## Action boundary

V0.1 parses a line beginning with an identifier that is neither assignment nor call as an `Action` phrase. The interpreter records the action deterministically. Future packages will register typed action signatures; arbitrary runtime guessing is prohibited.

## Effects

Future effectful operations should use typed capability interfaces rather than special-casing databases, HTTP, files, AI, or operating systems in the core grammar. This keeps the language portable and makes sandboxing possible.
