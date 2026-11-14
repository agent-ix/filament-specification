---
id: US-010
title: "Author and maintain complete native Filament specifications"
type: US
relationships:
  - target: ix://agent-ix/filament-specification/StR-002
    type: traces_to
---

# US-010: Author and maintain complete native Filament specifications

## Story

**As a** specification author
**I want** one lossless, typed and versioned native language for the complete V1 capability set
**So that** I can evolve formal models without changing meaning across tools and backends.

## Context

Authors need complete values, models, expressions, time, protocols and linked
plans while retaining exact diagnostics and formatting.

## Acceptance Examples (Illustrative)

### US-010-EX-1: One complete package

- **Given** a package using every selected complete-V1 facet
- **When** it is parsed, checked and inspected by two conforming tools
- **Then** both tools report the same identities, capabilities and meanings

### US-010-EX-2: Unsupported tool remains honest

- **Given** a valid package containing a construct one tool cannot execute
- **When** that tool receives the package
- **Then** it preserves the package and reports the exact unsupported capability
