use proptest::prelude::*;
use nanogate::{Nanogate, GateInput, IdentityContext, ReferenceFrame};

proptest! {
    // Property 1: Same observer + same policy always admits
    #[test]
    fn stable_context_always_admits(
        agent in ".*",
        session in ".*",
        mem in 0..1000u64,
        policy in ".*",
        chain in any::<Vec<String>>(),
        ext in 0..1000u64,
        ts1 in 0..1000000u64,
        ts2 in 0..1000000u64,
        nonce1 in 0..1000u64,
        nonce2 in 0..1000u64,
    ) {
        prop_assume!(ts2 > ts1);
        let mut gate = Nanogate::new();
        let input1 = GateInput {
            identity: IdentityContext { agent_id: agent.clone(), session_id: session.clone(), memory_state: mem },
            reference: ReferenceFrame { policy_version: policy.clone(), delegation_chain: chain.clone(), external_state_hash: ext },
            timestamp_ns: ts1,
            nonce: nonce1,
        };
        let input2 = GateInput {
            identity: IdentityContext { agent_id: agent, session_id: session, memory_state: mem },
            reference: ReferenceFrame { policy_version: policy, delegation_chain: chain, external_state_hash: ext },
            timestamp_ns: ts2,
            nonce: nonce2,
        };
        let out1 = gate.evaluate(input1);
        let out2 = gate.evaluate(input2);
        assert!(out1.admit);
        assert!(out2.admit);
    }

    // Property 2: Any observer mutation should deny
    #[test]
    fn observer_mutation_denies(
        agent1 in ".*",
        agent2 in ".*",
        session in ".*",
        mem in 0..1000u64,
        policy in ".*",
        chain in any::<Vec<String>>(),
        ext in 0..1000u64,
        ts1 in 0..1000000u64,
        ts2 in 0..1000000u64,
        nonce1 in 0..1000u64,
        nonce2 in 0..1000u64,
    ) {
        prop_assume!(agent1 != agent2);
        prop_assume!(ts2 > ts1);
        let mut gate = Nanogate::new();
        let input1 = GateInput {
            identity: IdentityContext { agent_id: agent1, session_id: session.clone(), memory_state: mem },
            reference: ReferenceFrame { policy_version: policy.clone(), delegation_chain: chain.clone(), external_state_hash: ext },
            timestamp_ns: ts1,
            nonce: nonce1,
        };
        let input2 = GateInput {
            identity: IdentityContext { agent_id: agent2, session_id: session, memory_state: mem },
            reference: ReferenceFrame { policy_version: policy, delegation_chain: chain, external_state_hash: ext },
            timestamp_ns: ts2,
            nonce: nonce2,
        };
        gate.evaluate(input1);
        let out = gate.evaluate(input2);
        assert!(!out.admit);
        assert_eq!(out.reason, "Identity drift detected");
    }

    // Property 3: Any policy mutation should deny
    #[test]
    fn policy_mutation_denies(
        agent in ".*",
        session in ".*",
        mem in 0..1000u64,
        policy1 in ".*",
        policy2 in ".*",
        chain in any::<Vec<String>>(),
        ext in 0..1000u64,
        ts1 in 0..1000000u64,
        ts2 in 0..1000000u64,
        nonce1 in 0..1000u64,
        nonce2 in 0..1000u64,
    ) {
        prop_assume!(policy1 != policy2);
        prop_assume!(ts2 > ts1);
        let mut gate = Nanogate::new();
        let input1 = GateInput {
            identity: IdentityContext { agent_id: agent.clone(), session_id: session.clone(), memory_state: mem },
            reference: ReferenceFrame { policy_version: policy1, delegation_chain: chain.clone(), external_state_hash: ext },
            timestamp_ns: ts1,
            nonce: nonce1,
        };
        let input2 = GateInput {
            identity: IdentityContext { agent_id: agent, session_id: session, memory_state: mem },
            reference: ReferenceFrame { policy_version: policy2, delegation_chain: chain, external_state_hash: ext },
            timestamp_ns: ts2,
            nonce: nonce2,
        };
        gate.evaluate(input1);
        let out = gate.evaluate(input2);
        assert!(!out.admit);
        assert_eq!(out.reason, "Reference frame drift detected");
    }

    // Property 4: Timestamp alone never causes denial (monotonic increase is fine)
    #[test]
    fn timestamp_monotonic_never_denies(
        agent in ".*",
        session in ".*",
        mem in 0..1000u64,
        policy in ".*",
        chain in any::<Vec<String>>(),
        ext in 0..1000u64,
        ts1 in 0..1000000u64,
        ts2 in 0..1000000u64,
        nonce1 in 0..1000u64,
        nonce2 in 0..1000u64,
    ) {
        prop_assume!(ts2 > ts1);
        let mut gate = Nanogate::new();
        let input1 = GateInput {
            identity: IdentityContext { agent_id: agent.clone(), session_id: session.clone(), memory_state: mem },
            reference: ReferenceFrame { policy_version: policy.clone(), delegation_chain: chain.clone(), external_state_hash: ext },
            timestamp_ns: ts1,
            nonce: nonce1,
        };
        let input2 = GateInput {
            identity: IdentityContext { agent_id: agent, session_id: session, memory_state: mem },
            reference: ReferenceFrame { policy_version: policy, delegation_chain: chain, external_state_hash: ext },
            timestamp_ns: ts2,
            nonce: nonce2,
        };
        let out1 = gate.evaluate(input1);
        let out2 = gate.evaluate(input2);
        assert!(out1.admit);
        assert!(out2.admit);
    }
}
