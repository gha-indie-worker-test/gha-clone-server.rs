#!/usr/bin/env python3
# Refinement check: concrete clone states must refine the abstract ready/not-ready model.
CONCRETE = {"requested": False, "prepared": False, "cloned": False, "verified": True}
for state, abstract_ready in CONCRETE.items():
    refined = state == "verified"
    assert refined == abstract_ready, f"refinement mismatch for {state}"
# Safety: no non-verified concrete state may refine to abstract ready.
assert all(not ready for state, ready in CONCRETE.items() if state != "verified")
print("clone refinement model: ok")
