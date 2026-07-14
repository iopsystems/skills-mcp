# Roadmap

This roadmap describes intended outcomes. [The backlog](backlog.md) owns concrete
work items, and [assumptions and limitations](assumptions-and-limitations.md)
records the boundaries under which these stages are expected to work.

## Stage 1: Skill templates and dogfood

Separate invocable skills from inert templates, add read-only catalog and template
retrieval, and ship the first engineering-journal and feature-documentation
templates. Provide read-only recommendations and provenance-aware seeding.

Install the feature-documentation template in this repository and use it to make
the README a strong internal entry point for installation, skill discovery, and
contribution from development experience.

## Stage 2: Low-friction distribution

Publish versioned, checksummed artifacts for Apple Silicon macOS and the selected
Linux targets. Add an organizational Homebrew tap after the release artifacts and
platform smoke tests are trustworthy. Preserve source builds as the contributor
path and fallback.

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
