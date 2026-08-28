# Contributing to UNIVERSAL

Every language change must preserve determinism, readability, safety, and backward compatibility unless a documented breaking change is approved.

## Change classes

- Bug fix: implementation correction with no intended language semantic change.
- Library/tooling change: standard library, CLI, diagnostics, formatter, editor support.
- Language change: syntax, grammar, typing, runtime semantics, interoperability contract.
- Breaking change: invalidates previously valid programs or changes observable behavior.

## RFC requirement

Language changes and breaking changes require an RFC in `rfc/NNNN-title.md` describing motivation, grammar, AST/semantic changes, runtime behavior, compatibility, alternatives, security impact, diagnostics, and tests.

Compiler changes must include tests at the lowest affected layer plus at least one integration test when observable language behavior changes.
