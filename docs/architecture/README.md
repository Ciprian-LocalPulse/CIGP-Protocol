# Architecture

The v0.1 reference implementation is split into a Rust domain layer (`cigp-core`), cryptographic primitives (`cigp-crypto`), proof engine (`cigp-proof`), ledger (`cigp-ledger`), CLI, and virtual-credit demo simulator.

Julia, Python, and TypeScript directories contain small, dependency-light reference utilities. They are independent analysis or verification surfaces, not casino services.
