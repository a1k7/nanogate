use criterion::{criterion_group, criterion_main, Criterion};
use nanogate::{Nanogate, GateInput, IdentityContext, ReferenceFrame};
use std::time::{SystemTime, UNIX_EPOCH};

fn bench_gate(c: &mut Criterion) {
    let mut gate = Nanogate::new();
    let input = GateInput {
        identity: IdentityContext { agent_id: "agent_alice".to_string(), session_id: "session_1".to_string(), memory_state: 12345 },
        reference: ReferenceFrame { policy_version: "v1".to_string(), delegation_chain: vec!["root".to_string()], external_state_hash: 98765 },
        timestamp_ns: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64,
        nonce: 42,
    };
    c.bench_function("nanogate evaluate", |b| b.iter(|| gate.evaluate(input.clone())));
}

criterion_group!(benches, bench_gate);
criterion_main!(benches);
