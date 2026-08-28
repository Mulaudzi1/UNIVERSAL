# Getting Started

Install stable Rust and Cargo, then from the repository root run:

```bash
cargo test --workspace
cargo run -p universal -- check examples/first.univ
cargo run -p universal -- run examples/first.univ
```

Start with `examples/first.univ`. The program declares two entities, constructs an employee without a scorecard, checks the readable `HAS` condition, and emits a validation result.
