#!/usr/bin/env python3
"""Check proposed call arguments against the captured reconciliation tool schemas.

This validates the schemas' object/type/required/enum/range constraints, not
execution. The schemas contain no references or combinators. It rejects a
response with no recognized calls rather than reporting vacuous success.
"""
import json
import sys
from pathlib import Path


def errors(value, schema, path="$args"):
    kind = schema.get("type")
    if kind == "object":
        if not isinstance(value, dict):
            return [f"{path}: expected object"]
        out = [f"{path}.{key}: required" for key in schema.get("required", []) if key not in value]
        properties = schema.get("properties", {})
        for key, item in value.items():
            if key in properties:
                out.extend(errors(item, properties[key], f"{path}.{key}"))
            elif schema.get("additionalProperties") is False:
                out.append(f"{path}.{key}: undeclared argument")
        return out
    if kind == "string" and not isinstance(value, str):
        return [f"{path}: expected string"]
    if kind == "integer":
        if type(value) is not int:
            return [f"{path}: expected integer"]
        if "minimum" in schema and value < schema["minimum"]:
            return [f"{path}: below minimum"]
        if "maximum" in schema and value > schema["maximum"]:
            return [f"{path}: above maximum"]
    if "enum" in schema and value not in schema["enum"]:
        return [f"{path}: outside enum"]
    return []


def calls(value, path="$"):
    if isinstance(value, dict):
        if "tool" in value:
            yield path, value
        for key, item in value.items():
            yield from calls(item, f"{path}.{key}")
    elif isinstance(value, list):
        for index, item in enumerate(value):
            yield from calls(item, f"{path}[{index}]")


def main():
    schemas = json.loads(Path(sys.argv[1]).read_text())
    reports = {}
    for name in sys.argv[2:]:
        report = {"checked_calls": 0, "errors": []}
        for path, call in calls(json.loads(Path(name).read_text())):
            if call["tool"] not in schemas:
                continue
            report["checked_calls"] += 1
            for error in errors(call.get("arguments"), schemas[call["tool"]]):
                report["errors"].append({"path": path, "tool": call["tool"], "error": error})
        if not report["checked_calls"]:
            report["errors"].append({"error": "No recognized calls found"})
        reports[name] = report
    print(json.dumps(reports, indent=2))
    return int(not reports or any(report["errors"] for report in reports.values()))


if __name__ == "__main__":
    raise SystemExit(main())
