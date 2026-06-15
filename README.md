# Nanogate – Sub‑microsecond Runtime Governance Gate

[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-100%25-brightgreen)](https://github.com/a1k7/nanogate/actions)

**Nanogate** is a software‑only gate that evaluates eight governance invariants (Time, Continuity, Alignment, Genesis, Boundary, Reference, Causality, Consciousness, Coherence) in **~530 nanoseconds** on a standard CPU, emitting a signed, replayable proof.

It answers the question that most governance systems ignore:  
> *“Does the agent still deserve to execute right now?”*

Approval is not enough. Continuity must be re‑tested at every action.

---

## ✨ Key Features

- ⚡ **530 ns decision latency** – >1.9 million evaluations/sec per core
- 🔒 **Deterministic drift detection** – identity, policy, delegation, external state
- 📝 **Replayable BLAKE3 proofs** – signed, verifiable, court‑admissible
- 🧪 **Adversarially validated** – 0 false admits / 100k mutations, 0 false denies / 100k stable traces
- 🧩 **Lightweight** – no hardware attestation, no external dependencies (Rust only)

---

## 📊 Performance & Correctness

| Metric | Result |
|--------|--------|
| Median evaluation latency | **530 ns** (Criterion) |
| Throughput per core | **~1.9 million ops/sec** |
| Stable traces (100k) | **100% admitted** – 0 false denies |
| Mutated traces (100k) | **100% denied** – 0 false admits |
| Property tests | 4/4 passing (stable, observer drift, policy drift, monotonic timestamp) |
| Unit tests | 4/4 passing |

---

## 🚀 Quick Start

### Build and run the CLI

```bash
cargo build --release
cargo run --release
Run all tests

bash
cargo test --release
Run performance benchmark

bash
cargo bench
Expected output:

text
nanogate evaluate       time:   [528.91 ns 530.01 ns 531.18 ns]
🧠 How it works

Nanogate maintains a continuity hash chain across evaluations. Each call receives:

identity (agent_id, session_id, memory_state)
reference_frame (policy_version, delegation_chain, external_state_hash)
timestamp_ns (monotonic)
nonce (replay protection)
It then:

Hashes identity and reference using xxHash64 (fast, non‑cryptographic)
Compares with previous state
If unchanged and timestamp increased → admit = true
Else → admit = false with a reason (identity drift, policy drift, etc.)
Emits a BLAKE3 proof hash of all inputs
The gate is stateless except for the last verified hashes.

🧪 Validation Suite

Test Type	Cases	Result
Unit tests	4	✅ pass
Property tests	4	✅ pass (stable context, drift, timestamp)
Adversarial mutation (false admits)	100,000	✅ 0 false admits
Stable continuity (false denies)	100,000	✅ 0 false denies
Run:

bash
cargo test --release
## 💼 Commercial Licensing

Nanogate is open source under **MIT / Apache‑2.0** for non‑commercial and internal use.

For embedding Nanogate inside **proprietary agent runtimes**, **commercial products**, or **high‑frequency trading / real‑time robotics** environments, a commercial license is required.

### Commercial License Includes:
- ✅ Perpetual use in one proprietary product
- ✅ Email support for 12 months
- ✅ Right to modify and redistribute as part of your product (not as a standalone library)

### Pricing
| Type | Fee |
|------|-----|
| One‑time perpetual license | $5,000 (per product) |
| Annual support renewal | $1,000 (optional) |

### Consulting & Audits
- **Governance Flight Recorder Audit** – run our verifier on your agent traces, get a report with score, drift detection, and recommendations.  
  *Fixed fee: $3,000 – $5,000 per engagement.*



*All open‑source contributions remain MIT/Apache‑2.0. The commercial license only applies to embedding the core gate inside proprietary systems.*
Contact: warikakhilesh319@gmail.com for a quote.

🔗 Related Projects

DecisionAssure – trace engine and verifier
Governance Score CLI – 0‑100 governance scoring
Continuous Admissibility Protocol (CAP) – lightweight open standard for runtime admissibility
📄 License

Source code: MIT OR Apache‑2.0
Commercial use: requires a separate commercial license (see above)
🙏 Acknowledgements

xxHash for ultra‑fast hashing
BLAKE3 for cryptographic proofs
The Rust community for an excellent ecosystem
Continuity first.
Approval is not enough.
