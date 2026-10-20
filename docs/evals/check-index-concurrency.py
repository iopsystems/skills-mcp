#!/usr/bin/env python3
"""Exercise concurrent first-use MCP calls against five fresh synthetic vaults."""
import concurrent.futures
import hashlib
import importlib.util
import json
from pathlib import Path
import sys
import tempfile
import threading

sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location(
    "driver", HERE / "reconcile-execution/driver.py"
)
driver = importlib.util.module_from_spec(spec)
spec.loader.exec_module(driver)


def main():
    binary, skill = (Path(arg).resolve() for arg in sys.argv[1:3])
    rounds = []
    with tempfile.TemporaryDirectory(prefix="index-concurrency-") as temporary:
        suite = Path(temporary) / "suite"
        driver.setup(suite, binary, skill)
        for case_name in driver.read(HERE / "reconcile-execution/fixture.json")["cases"]:
            case = suite / case_name
            barrier = threading.Barrier(8)

            def call(_):
                barrier.wait()
                try:
                    result = driver.rpc(case, "vault_reflect", {})
                    if result.get("isError"):
                        return {"error": result}
                    report = json.loads(result["content"][0]["text"])
                    return {"report": report}
                except Exception as error:
                    return {"error": str(error)}

            with concurrent.futures.ThreadPoolExecutor(max_workers=8) as pool:
                calls = list(pool.map(call, range(8)))
            # All successful callers should observe the same complete report.
            reports = [call["report"] for call in calls if "report" in call]
            consistent = bool(reports) and all(report == reports[0] for report in reports)
            rounds.append({
                "case": case_name,
                "successful_calls": len(reports),
                "consistent_reports": consistent,
                "sample_report": reports[0] if reports else None,
                "errors": [call["error"] for call in calls if "error" in call],
            })
    result = {
        "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
        "method": "Eight separate MCP processes start vault_reflect together; five fresh vaults.",
        "rounds": rounds,
    }
    print(json.dumps(result, indent=2))
    return int(any(round["errors"] or not round["consistent_reports"] for round in rounds))


if __name__ == "__main__":
    raise SystemExit(main())
