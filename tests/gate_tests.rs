use nanogate::{Nanogate, GateInput, IdentityContext, ReferenceFrame};

#[test]
fn continuity_admits() {
    let mut gate = Nanogate::new();
    let input = GateInput {
        identity: IdentityContext { agent_id: "a".into(), session_id: "s".into(), memory_state: 0 },
        reference: ReferenceFrame { policy_version: "v1".into(), delegation_chain: vec!["root".into()], external_state_hash: 123 },
        timestamp_ns: 1000,
        nonce: 1,
    };
    let out = gate.evaluate(input);
    assert!(out.admit);
}

#[test]
fn policy_drift_denies() {
    let mut gate = Nanogate::new();
    let first = GateInput {
        identity: IdentityContext { agent_id: "a".into(), session_id: "s".into(), memory_state: 0 },
        reference: ReferenceFrame { policy_version: "v1".into(), delegation_chain: vec!["root".into()], external_state_hash: 123 },
        timestamp_ns: 1000,
        nonce: 1,
    };
    gate.evaluate(first);
    let second = GateInput {
        identity: IdentityContext { agent_id: "a".into(), session_id: "s".into(), memory_state: 0 },
        reference: ReferenceFrame { policy_version: "v2".into(), delegation_chain: vec!["root".into()], external_state_hash: 123 },
        timestamp_ns: 2000,
        nonce: 2,
    };
    let out = gate.evaluate(second);
    assert!(!out.admit);
    assert_eq!(out.reason, "Reference frame drift detected");
}

#[test]
fn observer_drift_denies() {
    let mut gate = Nanogate::new();
    let first = GateInput {
        identity: IdentityContext { agent_id: "a".into(), session_id: "s".into(), memory_state: 0 },
        reference: ReferenceFrame { policy_version: "v1".into(), delegation_chain: vec!["root".into()], external_state_hash: 123 },
        timestamp_ns: 1000,
        nonce: 1,
    };
    gate.evaluate(first);
    let second = GateInput {
        identity: IdentityContext { agent_id: "b".into(), session_id: "s".into(), memory_state: 0 },
        reference: ReferenceFrame { policy_version: "v1".into(), delegation_chain: vec!["root".into()], external_state_hash: 123 },
        timestamp_ns: 2000,
        nonce: 2,
    };
    let out = gate.evaluate(second);
    assert!(!out.admit);
    assert_eq!(out.reason, "Identity drift detected");
}

#[test]
fn hash_is_deterministic() {
    let input = GateInput {
        identity: IdentityContext { agent_id: "alice".into(), session_id: "sess".into(), memory_state: 42 },
        reference: ReferenceFrame { policy_version: "v1".into(), delegation_chain: vec!["root".into()], external_state_hash: 999 },
        timestamp_ns: 12345,
        nonce: 7,
    };
    let mut gate = Nanogate::new();
    let out1 = gate.evaluate(input.clone());
    let mut gate2 = Nanogate::new();
    let out2 = gate2.evaluate(input);
    assert_eq!(out1.proof_hash, out2.proof_hash);
}
