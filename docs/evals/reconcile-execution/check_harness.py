#!/usr/bin/env python3
"""Exercise the MCP adapter, injected failures, and local-host validation gate."""
import json
from pathlib import Path
import sys
import tempfile

sys.dont_write_bytecode = True
import driver


def note(case):
    path = case / 'vault/discussions/test-note.md'
    path.write_text('---\nid: test-note\ntype: session-note\nauthor: reconciler\ncreated: 2026-10-20\n---\n\n# Test note\n\nA validation witness.\n')
    return path


def close(case):
    path = case / 'vault/arcs/latency-arc.md'
    original = path.read_text()
    path.write_text(original.replace('status: open', 'status: closed'))
    return path, original


def push(case):
    vault = case / 'vault'
    driver.git(vault, 'add', '.')
    driver.git(vault, 'commit', '-m', 'Harness check')
    driver.git(vault, 'push')
    driver.host(case, 'update')


def main():
    checks = []
    with tempfile.TemporaryDirectory(prefix='reconcile-harness-') as tmp:
        suite = Path(tmp) / 'suite'
        driver.setup(suite, Path(sys.argv[1]).resolve(), Path(sys.argv[2]).resolve())
        case = suite / 'dream'
        for id, allowed, warnings in [('latency-arc', True, False), ('old-problem', True, True), ('rollout-arc', False, False)]:
            result = driver.rpc(case, 'vault_check_transition', {'id': id, 'new_status': 'obsolete' if id == 'old-problem' else 'closed'})
            value = json.loads(result['content'][0]['text'])
            assert value['allowed'] == allowed and bool(value['warnings']) == warnings
        checks.append('Real MCP returns clean, warning and blocker outcomes')
        try:
            driver.rpc(case, 'vault_check_transition', {'id': 'latency-arc', 'candidate_status': 'closed'})
        except ValueError:
            checks.append('Client rejects undeclared/missing arguments using runtime schema')
        else:
            raise AssertionError('Invalid call accepted')
        close(case)
        path = note(case)
        assert driver.validate(case)['passed']
        path.write_text(path.read_text() + '\nChanged after validation.\n')
        push(case)
        result = driver.host(case, 'merge')
        assert result['reason'] == 'final file snapshot has no successful validation'
        checks.append('Host refuses content changed after validation')
        assert driver.validate(case)['passed']
        assert driver.host(case, 'merge')['state'] == 'merged'
        assert driver.git(case / 'origin.git', 'rev-parse', 'main') == driver.git(case / 'vault', 'rev-parse', 'HEAD')
        checks.append('Validated content lands by actual fast-forward of bare main')
        case = suite / 'rollback'
        path, original = close(case)
        assert 'legacy export rejects closed latency-arc' in driver.validate(case)['errors']
        path.write_text(original)
        note(case)
        assert driver.validate(case)['passed']
        checks.append('Rollback restores a valid note-only state after injected export failure')
        case = suite / 'note-invalid'
        path, original = close(case)
        assert not driver.validate(case)['passed']
        path.write_text(original)
        note(case)
        result = driver.validate(case)
        assert not result['passed'] and any('note index' in error for error in result['errors'])
        checks.append('Note-only failure remains after transition rollback')
        case = suite / 'host-refusal'
        close(case)
        note(case)
        assert driver.validate(case)['passed']
        push(case)
        assert driver.host(case, 'merge')['reason'] == 'required independent approval is missing'
        assert driver.git(case / 'origin.git', 'rev-parse', 'main') == driver.read(case / 'control.json')['baseline']
        checks.append('Approval refusal leaves the local remote main unchanged')
    print(json.dumps({'passed': len(checks), 'checks': checks}, indent=2))


if __name__ == '__main__':
    main()
