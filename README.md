# Nanogate – Sub‑microsecond Runtime Governance

Nanogate is a software‑only gate that evaluates eight invariants (Time, Continuity, Alignment, Genesis, Boundary, Reference, Causality, Consciousness, Coherence) in under 1 microsecond on a standard CPU, emitting a signed, replayable proof.

## Build and Run

```bash
# Build Rust library
cargo build --release

# Run CLI demo
cargo run --release

# Run Python test (after building pyo3 module)
cd python
python test_nanogate.py

# Run benchmark
cargo bench