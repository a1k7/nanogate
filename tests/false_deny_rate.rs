use nanogate::{Nanogate, GateInput, IdentityContext, ReferenceFrame};
use rand::Rng;

#[test]
fn false_deny_rate_100k_no_false_denies() {
    let mut rng = rand::thread_rng();
    let mut false_denies = 0;
    let total = 100_000;

    for _ in 0..total {
        // Generate a random stable context (same observer, same policy)
        let agent: String = (0..5).map(|_| rng.gen::<char>()).collect();
        let session: String = (0..5).map(|_| rng.gen::<char>()).collect();
        let mem = rng.gen::<u64>();
        let policy: String = (0..5).map(|_| rng.gen::<char>()).collect();
        let chain = vec!["root".to_string()];
        let ext = rng.gen::<u64>();
        let ts1 = rng.gen::<u64>();
        let nonce1 = rng.gen::<u64>();
        let ts2 = ts1 + rng.gen_range(1..1000); // strictly increasing
        let nonce2 = rng.gen::<u64>();

        let input1 = GateInput {
            identity: IdentityContext { agent_id: agent.clone(), session_id: session.clone(), memory_state: mem },
            reference: ReferenceFrame { policy_version: policy.clone(), delegation_chain: chain.clone(), external_state_hash: ext },
            timestamp_ns: ts1,
            nonce: nonce1,
        };
        let input2 = GateInput {
            identity: IdentityContext { agent_id: agent.clone(), session_id: session.clone(), memory_state: mem },
            reference: ReferenceFrame { policy_version: policy.clone(), delegation_chain: chain, external_state_hash: ext },
            timestamp_ns: ts2,
            nonce: nonce2,
        };

        let mut gate = Nanogate::new();
        let out1 = gate.evaluate(input1);
        let out2 = gate.evaluate(input2);

        // Baseline must admit (genesis)
        assert!(out1.admit, "First evaluation of stable trace should admit");

        // Second evaluation with same identity/reference but later timestamp must also admit
        if !out2.admit {
            false_denies += 1;
            eprintln!(
                "False deny on valid continuity trace: agent={}, session={}, policy={}, ts1={}, ts2={}",
                agent, session, policy, ts1, ts2
            );
        }
    }

    assert_eq!(false_denies, 0, "Expected 0 false denies, got {}", false_denies);
}
