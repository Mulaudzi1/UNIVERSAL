# Conditions

V0.1 supports `WHEN`, repeated `ELSE WHEN`, `OTHERWISE`, `AND`, `OR`, `NOT`, comparisons, `HAS`, `DOES NOT HAVE`, `IS`, `IS NOT`, and `.exists`-style `EXISTS` postfix semantics.

Readable conditions are grammar sugar over typed AST nodes and are validated against entity declarations.
