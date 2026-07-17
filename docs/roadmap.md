# Roadmap

This roadmap describes intended outcomes. [The backlog](backlog.md) owns concrete
work items, and [assumptions and limitations](assumptions-and-limitations.md)
records the boundaries under which these stages are expected to work.

## Stage 1: Skill templates and dogfood (in implementation)

The repository separates invocable skills from inert templates, provides read-only
catalog and template retrieval, and ships the first engineering-journal and
feature-documentation templates with read-only recommendations and
provenance-aware seeding. Seeder mutations coordinate cooperative project writers
and safe-stop on detected identity or digest changes; an uncooperative local writer
remains outside that guarantee.

The feature-documentation template is installed in this repository and produced
the README as an internal entry point for installation, skill discovery, and
contribution from development experience.

The implementation is ready for pull-request review, but this stage remains open
until live Codex and Claude Code discovery is verified from the same canonical
installed instance or the repository owner explicitly revises that criterion.

## Stage 2: Low-friction distribution (delivered in v0.1.0)

The v0.1.0 release publishes versioned, checksummed binaries for Apple Silicon
and Intel macOS and for x86_64 and aarch64 Linux, built by
`.github/workflows/release.yml`. The hosted `install.sh` fetches and verifies the
binary matching the caller's platform. The `iopsystems/homebrew-iop` tap carries
a `skills-mcp` formula with bottles for Apple Silicon macOS (Sonoma and Sequoia)
and x86_64 Linux, produced by the tap's `brew test-bot` pipeline. Source builds
remain the contributor path and the fallback for platforms without a bottle or a
prebuilt binary.

## Stage 3: Adoption and observation

Use explicit, authorized surveys to find installed instances across selected
projects. Compare each instance with its recorded base, distinguish deliberate
customization from accidental drift, and report recurring customizations without
changing either downstream projects or base templates automatically.

## Stage 4: Evidence-based template evolution

Promote broadly useful patterns into base templates through reviewed changes and
provide three-way upgrades to downstream instances. Add template-catalog or
retrieval capabilities beyond the initial read-only tools only when observed usage
shows a clear need.

## Stage 5: Broader distribution

Evaluate coding-agent plugin or marketplace packaging after local installation,
customization, upgrade, and review workflows are stable. Treat each platform as a
distribution adapter rather than a new source of truth.
