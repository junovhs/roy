# ROY Current Capabilities

Version: `v0.1.4`

This document is the current-state snapshot of what ROY actually ships today. It is intentionally narrower than the concept and architecture docs. If this file and the code disagree, the code wins and this file should be updated.

## Product Status

ROY is currently a controlled shell host with a desktop cockpit, a small ROY-native command world, deterministic shell-fallback denials, typed session/artifact tracking, and an implemented Claude Code adapter layer.

ROY is not yet a full general-purpose shell, and it is not yet exposing every planned agent-hosting flow directly through the live cockpit UI.

## What Works Today

### 1. Controlled shell cockpit

- Desktop app built with Rust + Dioxus.
- Main cockpit layout includes:
  - workspace pane
  - shell pane
  - activity pane
  - artifact row
  - diagnostics pane
- Shell sessions track typed events instead of only raw transcript text.

### 2. Public ROY command surface

The public command list currently includes:

- `cd`
- `pwd`
- `env`
- `exit`
- `help`
- `commands`
- `ls`
- `read`
- `write`
- `check`

Current semantics:

- `ls [path]` lists workspace entries
- `read <path>` prints a workspace file
- `write <path> <text>` overwrites or creates a workspace file
- `check` runs trusted workspace validation (`cargo check`)

### 3. Deterministic compatibility traps

ROY explicitly denies common shell fallback surfaces and redirects the user or embedded agent toward the ROY world instead.

Representative blocked commands:

- shells: `bash`, `sh`, `zsh`, `fish`, `csh`
- search/read fallbacks: `grep`, `rg`, `find`, `cat`, `head`, `tail`
- destructive/file ops: `rm`, `mv`, `cp`
- network/package/admin: `curl`, `wget`, `sudo`, `apt`, `apt-get`, `pip`, `npm`
- interpreters: `python`, `python3`, `node`

These are blocked by registry/policy behavior, not by vague guidance.

### 4. Workspace and policy boundaries

- Commands resolve through a ROY-owned registry, not arbitrary shell delegation.
- Current working directory is workspace-scoped.
- ROY-native command help and discoverability surfaces are available inside the shell.
- Denied commands produce structured session events and user-visible diagnostics.

### 5. Typed session and artifact ledger

ROY has typed session events for shell activity and promoted artifacts for important outputs.

Artifact model currently includes:

- diffs
- validation runs
- denied-command traces
- issue drafts
- notes

Promotions currently implemented in runtime flow:

- file writes can produce diff artifacts
- validation runs can produce validation artifacts
- denied commands can produce denied-command-trace artifacts

### 6. Artifact and diagnostics UI

The cockpit can render promoted artifacts with dedicated viewers instead of leaving them buried in transcript text.

Current viewer support includes:

- diff artifacts
- validation run artifacts
- denied-command-trace artifacts
- note / issue-draft display shapes

Diagnostics surfaces also expose command, artifact, and trace state for developers.

### 7. Local persistence foundation

SQLite-backed session persistence exists and includes artifact references.

What is implemented:

- migrations
- session/event persistence
- artifact reference persistence
- replay/load helpers
- idempotent save behavior

Current limitation:

- the SQLite store exists and is tested, but full live session-manager wiring into the running desktop loop is still partial rather than product-complete.

### 8. Embedded-agent hosting status

Implemented now:

- embedded-agent adapter contract
- agent session model
- Claude Code adapter
- Claude binary discovery
- controlled PATH shaping for Claude launch
- stdout/stderr/exit supervision
- buffered pending-event drain model

Important current limitation:

- the shipped cockpit still presents itself as a local shell session by default
- there is not yet a first-class in-app launcher/selector flow for switching the live cockpit into a Claude-hosted session
- Codex hosting is not yet implemented as a concrete adapter

## What ROY Does Not Yet Claim

ROY should not currently claim all of the following as finished product behavior:

- full multi-agent hosting parity across Claude Code and Codex
- a fully surfaced embedded-agent control flow in the main cockpit
- general text search beyond the current `ls`/`read`/`write` world
- arbitrary subprocess execution as a supported user-facing capability
- network/package-manager/script-interpreter access inside the ROY command world

## Why `v0.1.4`

`v0.1.4` is a reasonable current version marker because the repo now includes:

- the controlled shell cockpit
- public ROY-native filesystem and validation commands
- deterministic denial/redirect behavior
- an implemented Claude Code adapter layer
- typed session artifacts with dedicated UI viewers
- SQLite artifact/session persistence foundations

That is materially beyond the earlier `0.1.1` metadata and better matches the actual state of the codebase.
