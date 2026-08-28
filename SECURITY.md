# Security Policy

## Supported versions

UNIVERSAL is pre-1.0. Security fixes are applied to the latest development release only.

## Reporting a vulnerability

Do not open a public issue for a suspected vulnerability. Use the repository's private security advisory mechanism or contact the maintainers privately.

Include the affected version, reproduction steps, impact, and any proposed mitigation. Maintainers should acknowledge reports, triage severity, prepare a fix and coordinated disclosure, and add a regression test where practical.

## Secure-development rules

- `unsafe` Rust is forbidden in the compiler workspace unless an RFC explicitly changes this policy.
- External capabilities such as filesystem, network, process execution, databases and AI must remain explicit runtime capabilities.
- Compiler input is untrusted input: parsers and diagnostics must not panic on malformed source.
- Secrets must never be embedded in UNIVERSAL source examples or test fixtures.
