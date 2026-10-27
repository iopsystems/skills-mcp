---
name: rust-source-layout
description: >-
  Keep Rust source files cohesive and navigable when adding substantial code,
  reviewing a growing module, or splitting a file that mixes declarations,
  runtime behavior, and large tests. Preserve public paths and behavior while
  choosing module boundaries. Do not trigger a repository-wide cleanup for a
  small unrelated edit or mechanically split generated code.
---

# Rust source layout

A reader should be able to find a declaration, understand the behavior that
implements it, and review its tests without searching through unrelated work.
Choose boundaries by responsibility and dependency direction; use size to
notice when those boundaries have become hard to see.

## Inspect before adding or moving code

Read the scoped repository instructions and neighboring modules. Inspect the
whole touched file, its child modules, callers, feature gates, and tests. Map
the responsibilities it currently holds: public API, schema, declarative data,
runtime policy, I/O, algorithms, and test fixtures. Identify which items share
an invariant or tend to change together.

Measure production code, declarative tables, and tests separately. A file around
500 lines of handwritten production code deserves a look before substantial
growth; this is a review prompt, not a cap or a requirement to split. A short
file can mix unrelated responsibilities, and a long coherent table can be easy
to maintain. Follow an explicit project limit when one exists.

During implementation, put new work in its natural home before appending it to
an already mixed file. During review, cite the concrete navigation or ownership
problem and propose a boundary. Do not flag a line count alone as a defect.
Keep a small unrelated fix scoped; record a larger layout concern separately
unless the requested work needs that reorganization.

## Choose cohesive boundaries

- Keep a module's entrypoint readable as its API and wiring. Put substantial
  implementation in named child modules when a caller no longer needs it to
  understand that entrypoint.
- Keep declarative registries and configuration tables visually declarative.
  Move substantial schema definitions, environment discovery, validation, and
  runtime policy into appropriate modules. Keep a small table-specific lookup
  beside the table when that is its clearest home.
- Group code by a concept it owns, such as host discovery, geometry limits,
  registry declarations, or validation. A schema shared across those concepts
  can have its own module. Avoid a generic `helpers`, `utils`, or `constants`
  file that merely collects leftovers from unrelated concepts.
- Move a large unit-test body to `tests.rs` or `tests/` under the module,
  loaded with `#[cfg(test)] mod tests;`. Keep small, local tests inline when
  they remain easy to read. Put bulky witnesses, fixtures, and reusable test
  support under that test module, so they do not obscure the production API.
  Use crate-level `tests/` for tests of the public API; do not expose internals
  just to move a unit test there.
- Keep an algorithm, ownership protocol, or unsafe invariant together when a
  split would hide the relationship needed to judge correctness. Split its
  independent adapters or test data first. Avoid one file per type, numbered
  chunks, and modules that exist only to satisfy a size target.
- For generated or vendored files, inspect their ownership and generator.
  Change generator structure only when authorized and useful; do not hand-edit
  or chop up generated output to meet a handwritten-code guideline.

For example, a stream registry might become:

```text
streams.rs            public facade, module declarations, selected re-exports
streams/schema.rs     stream and buffer schema types
streams/hosts.rs      host parsing and environment discovery
streams/geometry.rs   sizing constants and related rules
streams/registry.rs   stream and buffer declarations, table-specific lookups
streams/tests.rs      unit tests
streams/tests/witness.rs   large test-only reference data
```

This is an example of separable responsibilities, not a required directory
template. Use `streams/mod.rs` instead of `streams.rs` if that matches the
repository. Do not create both as competing module entrypoints.

## Preserve contracts during the move

Keep existing public paths with explicit facade re-exports when the task is
organizational: callers of `crate::streams::StreamSpec` should continue to
compile. Keep implementation modules private and use the narrowest internal
visibility that works. Do not make everything `pub`, duplicate type definitions,
or add glob re-exports simply to silence privacy errors.

Preserve declaration order where it is observable, values, feature/platform
gates, error behavior, and synchronization or initialization order. Check
relative `include!`/`include_str!` paths, macro scope, and rustdoc links after
moves. Put genuinely shared definitions below their users in the dependency
structure rather than hiding circular ownership behind cross-module imports.
Keep behavior changes separate so the reviewer can distinguish them from moves.

## Validate and report

Run the repository's format/check commands and affected crate tests, including
relevant features. Code gated to a target the host does not compile, such as
Linux-only code on a macOS host or Windows-only code on Linux, is not checked by
host runs. Require that target's CI job to pass, or
`cargo clippy --target <triple> --all-targets` to finish with no errors.
`--all-targets` compiles the gated test modules. Leave out `-D warnings`: it
turns a lint in the library into an error that stops the library's dependent
targets from compiling, which hides their errors. Run the same command on the
base commit, fix any warning the move introduced, and report warnings that were
already there without fixing them in the move. Install the target with
`rustup target add <triple>`. Report the gate as unverified when neither ran, or
when the cross check cannot build on this host for a reason unrelated to the
change, such as a build script that needs a cross C toolchain. When only the
cross check ran, report the target's tests as not run. Check downstream users of preserved public paths; a passing library
unit test alone does not verify its consumers. Use existing table-order or
fixture assertions to check declarative moves when available. For an
organizational change, add a test only for an uncovered contract at risk, not
to assert filenames or line counts.

Inspect the final diff for accidental value changes, widened visibility, lost
attributes, and duplicate definitions. Report the responsibilities moved, the
public paths preserved, validation results, and any remaining large cohesive
sections with the reason they stayed together. If the new modules still need
many cross-imports or a long explanation of where things live, reconsider the
boundary rather than adding more files.
