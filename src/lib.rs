use blake3::Hasher;
use serde::{Serialize, Deserialize};
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityContext {
    pub agent_id: String,
    pub session_id: String,
    pub memory_state: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceFrame {
    pub policy_version: String,
    pub delegation_chain: Vec<String>,
    pub external_state_hash: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GateInput {
    pub identity: IdentityContext,
    pub reference: ReferenceFrame,
    pub timestamp_ns: u64,
    pub nonce: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GateOutput {
    pub admit: bool,
    pub reason: String,
    pub proof_hash: String,
    pub elapsed_ns: u64,
}

pub struct Nanogate {
    last_identity_hash: u64,
    last_ref_hash: u64,
    last_timestamp_ns: u64,
}

impl Nanogate {
    pub fn new() -> Self {
        Nanogate {
            last_identity_hash: 0,
            last_ref_hash: 0,
            last_timestamp_ns: 0,
        }
    }

    pub fn hash_identity(ctx: &IdentityContext) -> u64 {
        let mut hasher = xxhash_rust::xxh64::Xxh64::new(0);
        hasher.update(ctx.agent_id.as_bytes());
        hasher.update(&[0]);
        hasher.update(ctx.session_id.as_bytes());
        hasher.update(&[0]);
        hasher.update(&ctx.memory_state.to_le_bytes());
        hasher.digest()
    }

    pub fn hash_reference(frame: &ReferenceFrame) -> u64 {
        let mut hasher = xxhash_rust::xxh64::Xxh64::new(0);
        hasher.update(frame.policy_version.as_bytes());
        hasher.update(&[0]);
        for chain in &frame.delegation_chain {
            hasher.update(chain.as_bytes());
            hasher.update(&[0]);
        }
        hasher.update(&frame.external_state_hash.to_le_bytes());
        hasher.digest()
    }

    pub fn evaluate(&mut self, input: GateInput) -> GateOutput {
        let start = Instant::now();
        let identity_hash = Self::hash_identity(&input.identity);
        let ref_hash = Self::hash_reference(&input.reference);
        let is_first = self.last_identity_hash == 0 && self.last_ref_hash == 0;
        let identity_ok = is_first || identity_hash == self.last_identity_hash;
        let ref_ok = is_first || ref_hash == self.last_ref_hash;
        let time_ok = is_first || input.timestamp_ns > self.last_timestamp_ns;
        let admit = identity_ok && ref_ok && time_ok;
        let reason = if !identity_ok {
            "Identity drift detected".to_string()
        } else if !ref_ok {
            "Reference frame drift detected".to_string()
        } else if !time_ok {
            "Timestamp non‑monotonic".to_string()
        } else {
            "Continuity intact".to_string()
        };
        let mut hasher = Hasher::new();
        hasher.update(&identity_hash.to_le_bytes());
        hasher.update(&ref_hash.to_le_bytes());
        hasher.update(&input.timestamp_ns.to_le_bytes());
        hasher.update(&input.nonce.to_le_bytes());
        let proof_hash = hasher.finalize().to_hex().to_string();
        if admit {
            self.last_identity_hash = identity_hash;
            self.last_ref_hash = ref_hash;
            self.last_timestamp_ns = input.timestamp_ns;
        }
        let elapsed = start.elapsed().as_nanos() as u64;
        GateOutput {
            admit,
            reason,
            proof_hash,
            elapsed_ns: elapsed,
        }
    }
}
