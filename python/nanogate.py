import json
import nanogate  # the compiled Rust extension

class PyNanogate:
    def __init__(self):
        self._gate = nanogate.PyNanogate()

    def evaluate(self, identity: dict, reference: dict, timestamp_ns: int, nonce: int) -> dict:
        input_json = json.dumps({
            "identity": identity,
            "reference": reference,
            "timestamp_ns": timestamp_ns,
            "nonce": nonce,
        })
        output_json = self._gate.evaluate(input_json)
        return json.loads(output_json)