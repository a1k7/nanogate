use nanogate::{Nanogate, GateInput, IdentityContext, ReferenceFrame};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let mut gate = Nanogate::new();
    let input1 = GateInput {
        identity: IdentityContext { agent_id: "agent_alice".to_string(), session_id: "session_1".to_string(), memory_state: 12345 },
        reference: ReferenceFrame { policy_version: "v1".to_string(), delegation_chain: vec!["root".to_string()], external_state_hash: 98765 },
        timestamp_ns: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64,
        nonce: 42,
    };
    let out1 = gate.evaluate(input1);
    println!("First call: admit={}, reason={}, elapsed_ns={}", out1.admit, out1.reason, out1.elapsed_ns);
    let input2 = GateInput {
        identity: IdentityContext { agent_id: "agent_alice".to_string(), session_id: "session_1".to_string(), memory_state: 12345 },
        reference: ReferenceFrame { policy_version: "v1".to_string(), delegation_chain: vec!["root".to_string()], external_state_hash: 98765 },
        timestamp_ns: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64,
        nonce: 43,
    };
    let out2 = gate.evaluate(input2);
    println!("Second call (same): admit={}, reason={}, elapsed_ns={}", out2.admit, out2.reason, out2.elapsed_ns);
    let input3 = GateInput {
        identity: IdentityContext { agent_id: "agent_alice".to_string(), session_id: "session_1".to_string(), memory_state: 12345 },
        reference: ReferenceFrame { policy_version: "v2".to_string(), delegation_chain: vec!["root".to_string()], external_state_hash: 98765 },
        timestamp_ns: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64,
        nonce: 44,
    };
    let out3 = gate.evaluate(input3);
    println!("Third call (policy drift): admit={}, reason={}, elapsed_ns={}", out3.admit, out3.reason, out3.elapsed_ns);
}
