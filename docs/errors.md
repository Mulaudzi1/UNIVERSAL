# Errors and Results

UNIVERSAL should prefer explicit `Result<T, E>` for expected failure and reserve traps/panics for broken invariants. Validation is separate from exceptional failure.

The bootstrap compiler assigns diagnostics stable families: `U1xxx` lexer, `U2xxx` parser, `U3xxx` semantic/type, `U4xxx` runtime.

Every diagnostic stores a span and can carry a suggested fix. Rich source-snippet rendering is a CLI tooling milestone.
