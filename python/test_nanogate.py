import time
import json
from nanogate import PyNanogate

gate = PyNanogate()

# Simulate a DecisionAssure trace step
identity = {
    "agent_id": "agent_alice",
    "session_id": "session_live",
    "memory_state": 0  # simplified from JSON
}
reference = {
    "policy_version": "v1",
    "delegation_chain": ["root"],
    "external_state_hash": 123456
}
timestamp_ns = time.time_ns()
nonce = 42

# First call (genesis)
result = gate.evaluate(identity, reference, timestamp_ns, nonce)
print("First call:", result)

# Second call – same, should admit
timestamp_ns = time.time_ns()
result = gate.evaluate(identity, reference, timestamp_ns, nonce+1)
print("Second call (same):", result)

# Third call – change policy version (drift)
reference2 = reference.copy()
reference2["policy_version"] = "v2"
timestamp_ns = time.time_ns()
result = gate.evaluate(identity, reference2, timestamp_ns, nonce+2)
print("Third call (policy drift):", result)