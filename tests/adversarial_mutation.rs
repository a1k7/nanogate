use nanogate::{Nanogate, GateInput, IdentityContext, ReferenceFrame};
use rand::Rng;

#[test]
fn adversarial_mutation_100k_no_false_admits() {
    let mut rng = rand::thread_rng();
    let mut false_admits = 0;
    let total = 100_000;

    for _ in 0..total {
        // Generate a random baseline context
        let agent1: String = (0..5).map(|_| rng.gen::<char>()).collect();
        let session1: String = (0..5).map(|_| rng.gen::<char>()).collect();
        let mem1 = rng.gen::<u64>();
        let policy1: String = (0..5).map(|_| rng.gen::<char>()).collect();
        let chain1 = vec!["root".to_string()];
        let ext1 = rng.gen::<u64>();
        let ts1 = rng.gen::<u64>();
        let nonce1 = rng.gen::<u64>();

        let input1 = GateInput {
            identity: IdentityContext { agent_id: agent1.clone(), session_id: session1.clone(), memory_state: mem1 },
            reference: ReferenceFrame { policy_version: policy1.clone(), delegation_chain: chain1.clone(), external_state_hash: ext1 },
            timestamp_ns: ts1,
            nonce: nonce1,
        };

        // Create a second input that is either identical (should admit) or mutated (should deny)
        let same = rng.gen_bool(0.5);
        let (agent2, session2, mem2, policy2, chain2, ext2, ts2, nonce2) = if same {
            (agent1.clone(), session1.clone(), mem1, policy1.clone(), chain1.clone(), ext1, ts1 + 1, rng.gen::<u64>())
        } else {
            let choice = rng.gen_range(0..5);
            match choice {
                0 => (format!("{}_mutated", agent1), session1.clone(), mem1, policy1.clone(), chain1.clone(), ext1, ts1 + 1, rng.gen::<u64>()),
                1 => (agent1.clone(), format!("{}_mutated", session1), mem1, policy1.clone(), chain1.clone(), ext1, ts1 + 1, rng.gen::<u64>()),
                2 => (agent1.clone(), session1.clone(), mem1.wrapping_add(1), policy1.clone(), chain1.clone(), ext1, ts1 + 1, rng.gen::<u64>()),
                3 => (agent1.clone(), session1.clone(), mem1, format!("{}_mutated", policy1), chain1.clone(), ext1, ts1 + 1, rng.gen::<u64>()),
                4 => (agent1.clone(), session1.clone(), mem1, policy1.clone(), chain1.clone(), ext1.wrapping_add(1), ts1 + 1, rng.gen::<u64>()),
                _ => unreachable!(),
            }
        };

        let input2 = GateInput {
            identity: IdentityContext { agent_id: agent2, session_id: session2, memory_state: mem2 },
            reference: ReferenceFrame { policy_version: policy2, delegation_chain: chain2, external_state_hash: ext2 },
            timestamp_ns: ts2,
            nonce: nonce2,
        };

        let mut gate = Nanogate::new();
        let out1 = gate.evaluate(input1.clone()); // clone for later use
        let out2 = gate.evaluate(input2.clone()); // clone for later use

        assert!(out1.admit, "Baseline should admit");

        if same {
            if !out2.admit {
                false_admits += 1;
                eprintln!("False deny when inputs identical: input1={:?}, input2={:?}", input1, input2);
            }
        } else {
            if out2.admit {
                false_admits += 1;
                eprintln!("False admit on mutation: input1={:?}, input2={:?}", input1, input2);
            }
        }
    }

    assert_eq!(false_admits, 0, "Expected 0 false admits, got {}", false_admits);
}
