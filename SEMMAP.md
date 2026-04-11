# project -- Semantic Map

**Purpose:** Controlled shell host for terminal-native agents (Claude Code, Codex). Owns command resolution, policy enforcement, workspace boundaries, and session/artifact ledger. Built in Rust + Dioxus desktop.

## Legend

`[ENTRY]` Application entry point

`[CORE]` Core business logic

`[TYPE]` Data structures and types

`[UTIL]` Utility functions

`[HOTSPOT]` High fan-in file imported by 4+ others - request this file early in any task

`[GLOBAL-UTIL]` High fan-in utility imported from 3+ distinct domains

`[DOMAIN-CONTRACT]` Shared contract imported mostly by one subsystem

`[ROLE:model]` Primary domain model or state-holding data structure.

`[ROLE:controller]` Coordinates commands, events, or request handling.

`[ROLE:rendering]` Produces visual output or drawing behavior.

`[ROLE:view]` Represents a reusable UI view or presentation component.

`[ROLE:dialog]` Implements dialog-oriented interaction flow.

`[ROLE:config]` Defines configuration loading or configuration schema behavior.

`[ROLE:os-integration]` Bridges the application to OS-specific APIs or services.

`[ROLE:utility]` Provides cross-cutting helper logic without owning core flow.

`[ROLE:bootstrap]` Initializes the application or wires subsystem startup.

`[ROLE:build-only]` Supports the build toolchain rather than runtime behavior.

`[COUPLING:pure]` Logic stays within the language/runtime without external surface coupling.

`[COUPLING:mixed]` Blends pure logic with side effects or boundary interactions.

`[COUPLING:ui-coupled]` Depends directly on UI framework, rendering, or windowing APIs.

`[COUPLING:os-coupled]` Depends directly on operating-system services or platform APIs.

`[COUPLING:build-only]` Only relevant during build, generation, or compilation steps.

`[BEHAVIOR:owns-state]` Maintains durable in-memory state for a subsystem.

`[BEHAVIOR:mutates]` Changes application or model state in response to work.

`[BEHAVIOR:renders]` Produces rendered output, drawing commands, or visual layout.

`[BEHAVIOR:dispatches]` Routes commands, events, or control flow to other units.

`[BEHAVIOR:observes]` Listens to callbacks, notifications, or external signals.

`[BEHAVIOR:persists]` Reads from or writes to durable storage.

`[BEHAVIOR:spawns-worker]` Creates background workers, threads, or async jobs.

`[BEHAVIOR:sync-primitives]` Coordinates execution with locks, channels, or wait primitives.

`[SURFACE:filesystem]` Touches filesystem paths, files, or directory traversal.

`[SURFACE:ntfs]` Uses NTFS-specific filesystem semantics or metadata.

`[SURFACE:win32]` Touches Win32 platform APIs or Windows-native handles.

`[SURFACE:shell]` Integrates with shell commands, shell UX, or command launch surfaces.

`[SURFACE:clipboard]` Reads from or writes to the system clipboard.

`[SURFACE:gdi]` Uses GDI drawing primitives or related graphics APIs.

`[SURFACE:control]` Represents or manipulates widget/control surfaces.

`[SURFACE:view]` Represents a view-level presentation surface.

`[SURFACE:dialog]` Represents a dialog/window interaction surface.

`[SURFACE:document]` Represents document-oriented editing or display surfaces.

`[SURFACE:frame]` Represents application frame/window chrome surfaces.

`[BEHAVIOR:async]` Uses async/await patterns for concurrent execution.

`[BEHAVIOR:panics-on-error]` Contains unwrap/expect/panic patterns that abort on failure.

`[BEHAVIOR:logs-and-continues]` Logs errors and continues without propagating or aborting.

`[BEHAVIOR:returns-nil-on-error]` Returns nil/null/None on error instead of propagating.

`[BEHAVIOR:swallows-errors]` Catches errors without re-raising or propagating them.

`[BEHAVIOR:propagates-errors]` Propagates errors to callers via Result, throw, or raise.

`[SURFACE:http-handler]` Implements HTTP request handling or web endpoint logic.

`[SURFACE:database]` Interacts with database services or ORMs.

`[SURFACE:external-api]` Makes outbound calls to external HTTP APIs or services.

`[SURFACE:template]` Uses template engines for rendering output.

`[QUALITY:undocumented]` Has public symbols without documentation.

`[QUALITY:complex-flow]` Contains functions with high cognitive complexity.

`[QUALITY:error-boundary]` Concentrated error handling — many panic, swallow, or propagation sites.

`[QUALITY:concurrency-heavy]` Uses multiple concurrency primitives (async, locks, spawn).

`[QUALITY:syntax-degraded]` Parse errors detected — semantic analysis may be incomplete.

## Layer 0 -- Config

`Cargo.toml`
Workspace configuration.

`SEMMAP.md`
Generated semantic map.

## Layer 1 -- Domain (Engine)

`src/commands/registry.rs`
All commands known to ROY — built-ins, ROY-native (pending), and compat traps. [CORE] [HOTSPOT] [GLOBAL-UTIL] [COUPLING:mixed] [BEHAVIOR:owns-state,panics-on-error] [QUALITY:error-boundary]
Exports: CommandRegistry.is_empty, CommandRegistry.public_commands, CommandRegistry, CommandRegistry.default
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting stateful module that panics on error

`src/commands/schema.rs`
How a command is executed once resolved. [CORE] [COUPLING:pure]
Exports: Backend.is_denied, CommandSchema, RiskLevel, Backend
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation

`src/policy/engine.rs`
Typed outcome of a policy evaluation. [COUPLING:mixed] [BEHAVIOR:persists]
Exports: PolicyEngine.profile_name, PolicyEngine.set_profile, PolicyEngine.is_allowed, PolicyOutcome.is_blocked
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting adapter

`src/policy/profile.rs`
What the policy engine decides about a particular command invocation. [COUPLING:pure]
Exports: PolicyProfile.is_explicitly_allowed, PolicyProfile.read_only, PolicyProfile.is_blocked, PolicyPermission
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation

`src/session/engine.rs`
A ROY shell session — owns the ordered event ledger. [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:panics-on-error]
Exports: Session.events_of_kind, Session.is_empty, Session, Session.end
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting that panics on error

`src/session/events.rs`
Milliseconds since UNIX epoch — simple, copy-friendly, SQLite-friendly. [COUPLING:pure]
Exports: SessionEvent.kind_str, SessionEvent, SessionEvent.timestamp
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation

`src/shell/env.rs`
Controlled environment for one ROY shell session. [HOTSPOT] [GLOBAL-UTIL] [COUPLING:mixed] [BEHAVIOR:panics-on-error,propagates-errors] [QUALITY:error-boundary]
Exports: ShellEnv.roy_path, ShellEnv, ShellEnv.chdir, ShellEnv.cwd
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting that panics on error

`src/shell/io.rs`
IO surface for a ROY shell session. [COUPLING:pure] [QUALITY:undocumented]
Exports: BufferedIo.prompt_str, BufferedIo.write_error, BufferedIo.write_line, ShellIo
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation

`src/shell/resolve.rs`
Outcome of resolving a command name through the ROY command registry. [COUPLING:mixed] [BEHAVIOR:panics-on-error]
Exports: ResolveOutcome, resolve_command
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting that panics on error

`src/shell/result.rs`
Outcome of dispatching a command through the shell runtime.
Exports: DispatchResult

`src/shell/traps.rs`
Compatibility traps — well-known commands ROY explicitly blocks. [COUPLING:mixed] [BEHAVIOR:owns-state]
Exports: COMPAT_TRAPS
Semantic: side-effecting stateful module

`src/ui/layout/atoms.rs`
Implements atoms functionality. [COUPLING:pure]
Semantic: pure computation

`src/ui/layout/chrome.rs`
Implements chrome functionality. [COUPLING:mixed] [BEHAVIOR:owns-state,persists,sync-primitives]
Semantic: synchronized side-effecting stateful adapter

`src/ui/layout/footer.rs`
Implements footer functionality. [COUPLING:mixed] [BEHAVIOR:persists,sync-primitives]
Semantic: synchronized side-effecting adapter

`src/ui/layout/panels/activity.rs`
Implements activity functionality. [COUPLING:mixed] [BEHAVIOR:persists,sync-primitives] [QUALITY:complex-flow]
Semantic: synchronized side-effecting adapter

`src/ui/layout/panels/shell.rs`
Implements shell functionality. [COUPLING:mixed] [BEHAVIOR:owns-state,persists,sync-primitives]
Semantic: synchronized side-effecting stateful adapter

`src/ui/layout/panels/workspace.rs`
Implements workspace functionality. [COUPLING:mixed] [BEHAVIOR:persists,sync-primitives]
Semantic: synchronized side-effecting adapter

`src/workspace/boundary.rs`
Error types for workspace boundary operations. [HOTSPOT] [GLOBAL-UTIL] [COUPLING:mixed] [BEHAVIOR:panics-on-error,propagates-errors] [QUALITY:undocumented,error-boundary]
Exports: WorkspaceBoundary.validate_cwd, WorkspaceBoundary, WorkspaceBoundary.contains, WorkspaceBoundary.new
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting that panics on error

`src/workspace/cwd.rs`
Workspace-scoped current working directory. [COUPLING:mixed] [BEHAVIOR:panics-on-error,propagates-errors] [QUALITY:error-boundary]
Exports: WorkspaceCwd.display_path, WorkspaceCwd, WorkspaceCwd.boundary, WorkspaceCwd.chdir
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting that panics on error

## Layer 2 -- Adapters / Infra

`src/shell/runtime.rs`
ROY shell runtime. [CORE] [HOTSPOT] [COUPLING:pure]
Exports: ShellRuntime.public_command_count, ShellRuntime.last_exit_status, ShellRuntime.set_exit_status, ShellRuntime.env_mut
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation

## Layer 3 -- App / Entrypoints

`src/agents/mod.rs`
Placeholder file. [ENTRY]

`src/app/mod.rs`
Root application component. [ENTRY] [COUPLING:pure]
Exports: App
Semantic: pure computation

`src/commands/mod.rs`
Command resolution and substitution registry. [CORE] [HOTSPOT] [GLOBAL-UTIL]
Exports: CommandRegistry, CommandSchema, registry, schema

`src/diagnostics/mod.rs`
Placeholder file. [ENTRY]

`src/main.rs`
Application entry point. [ENTRY] [COUPLING:pure]
Semantic: pure computation

`src/policy/mod.rs`
Policy engine for command and capability execution. [ENTRY] [HOTSPOT] [GLOBAL-UTIL]
Exports: engine, profile

`src/session/mod.rs`
Session and transcript event ledger. [ENTRY] [HOTSPOT] [GLOBAL-UTIL]
Exports: Session, engine, events

`src/shell/mod.rs`
Shell host runtime. [ENTRY] [HOTSPOT] [GLOBAL-UTIL] [COUPLING:pure] [QUALITY:undocumented]
Exports: DispatchResult, ShellEnv, ShellError, ShellError.fmt
Semantic: pure computation

`src/storage/mod.rs`
Placeholder file. [ENTRY]

`src/ui/layout/mod.rs`
Root shell cockpit. [ENTRY] [COUPLING:mixed] [BEHAVIOR:owns-state]
Exports: Cockpit
Semantic: side-effecting stateful module

`src/ui/layout/panels/mod.rs`
Module definitions for mod. [ENTRY]

`src/ui/mod.rs`
Re-exports the public API surface. [ENTRY]
Exports: layout

`src/workspace/mod.rs`
Workspace management and boundary enforcement. [ENTRY] [HOTSPOT] [GLOBAL-UTIL]
Exports: WorkspaceCwd, boundary

## Layer 4 -- Tests

`src/shell/runtime_tests_builtins.rs`
Tests for ShellRuntime built-in command handlers: pwd, cd, env, exit, help — and the transcript drain. [COUPLING:mixed] [BEHAVIOR:panics-on-error] [QUALITY:error-boundary]
Semantic: side-effecting that panics on error

`src/shell/runtime_tests_policy.rs`
Tests for ShellRuntime dispatch policy: compatibility traps, NotFound, prompt indicator, transcript errors, and policy engine integration. [COUPLING:mixed] [BEHAVIOR:panics-on-error] [QUALITY:error-boundary]
Semantic: side-effecting that panics on error


## DependencyGraph

```yaml
DependencyGraph:
  # --- Entrypoints ---
  main.rs:
    Imports: [agents/mod.rs, app/mod.rs, boundary.rs, commands/mod.rs, diagnostics/mod.rs, policy/mod.rs, session/mod.rs, shell/mod.rs, storage/mod.rs, ui/mod.rs, workspace/mod.rs]
    ImportedBy: []
  # --- High Fan-In Hotspots ---
  boundary.rs:
    Imports: [workspace/mod.rs]
    ImportedBy: [cwd.rs, env.rs, io.rs, layout/mod.rs, main.rs, policy/engine.rs, profile.rs, registry.rs, resolve.rs, runtime.rs, runtime_tests_builtins.rs, runtime_tests_policy.rs, session/engine.rs, shell.rs, workspace/mod.rs]
  commands/mod.rs:
    Imports: [registry.rs, schema.rs]
    ImportedBy: [main.rs, policy/engine.rs, profile.rs, resolve.rs, runtime.rs]
  env.rs:
    Imports: [boundary.rs]
    ImportedBy: [cwd.rs, runtime.rs, shell/mod.rs, workspace.rs]
  policy/mod.rs:
    Imports: [policy/engine.rs, profile.rs]
    ImportedBy: [main.rs, policy/engine.rs, runtime.rs, runtime_tests_policy.rs]
  registry.rs:
    Imports: [boundary.rs]
    ImportedBy: [activity.rs, commands/mod.rs, io.rs, layout/mod.rs, policy/engine.rs, runtime.rs, session/engine.rs]
  session/mod.rs:
    Imports: [events.rs, session/engine.rs]
    ImportedBy: [activity.rs, chrome.rs, footer.rs, layout/mod.rs, main.rs, session/engine.rs, shell.rs, workspace.rs]
  shell/mod.rs:
    Imports: [env.rs, io.rs, resolve.rs, result.rs, runtime.rs, traps.rs]
    ImportedBy: [chrome.rs, footer.rs, layout/mod.rs, main.rs, runtime_tests_builtins.rs, runtime_tests_policy.rs, shell.rs, workspace.rs]
  workspace/mod.rs:
    Imports: [boundary.rs, cwd.rs]
    ImportedBy: [boundary.rs, cwd.rs, main.rs, runtime.rs]
  # --- Layer 0 -- Config ---
  Cargo.toml, SEMMAP.md:
    Imports: []
    ImportedBy: []
  # --- Layer 1 -- Domain (Engine) ---
  activity.rs:
    Imports: [registry.rs, session/mod.rs]
    ImportedBy: [panels/mod.rs]
  atoms.rs:
    Imports: []
    ImportedBy: [layout/mod.rs]
  chrome.rs:
    Imports: [session/mod.rs, shell/mod.rs]
    ImportedBy: [layout/mod.rs]
  cwd.rs:
    Imports: [boundary.rs, env.rs, workspace/mod.rs]
    ImportedBy: [workspace/mod.rs]
  events.rs:
    Imports: []
    ImportedBy: [session/engine.rs, session/mod.rs]
  footer.rs:
    Imports: [runtime.rs, session/engine.rs, session/mod.rs, shell/mod.rs]
    ImportedBy: [layout/mod.rs]
  io.rs:
    Imports: [boundary.rs, registry.rs]
    ImportedBy: [runtime.rs, shell/mod.rs]
  policy/engine.rs:
    Imports: [boundary.rs, commands/mod.rs, policy/mod.rs, profile.rs, registry.rs]
    ImportedBy: [policy/mod.rs, runtime.rs]
  profile.rs:
    Imports: [boundary.rs, commands/mod.rs]
    ImportedBy: [policy/engine.rs, policy/mod.rs]
  resolve.rs:
    Imports: [boundary.rs, commands/mod.rs]
    ImportedBy: [runtime.rs, shell/mod.rs]
  result.rs, traps.rs:
    Imports: []
    ImportedBy: [shell/mod.rs]
  schema.rs:
    Imports: []
    ImportedBy: [commands/mod.rs]
  session/engine.rs:
    Imports: [boundary.rs, events.rs, registry.rs, session/mod.rs]
    ImportedBy: [footer.rs, session/mod.rs, shell.rs]
  shell.rs:
    Imports: [boundary.rs, session/engine.rs, session/mod.rs, shell/mod.rs]
    ImportedBy: [panels/mod.rs]
  workspace.rs:
    Imports: [env.rs, session/mod.rs, shell/mod.rs]
    ImportedBy: [panels/mod.rs]
  # --- Layer 2 -- Adapters / Infra ---
  runtime.rs:
    Imports: [boundary.rs, commands/mod.rs, env.rs, io.rs, policy/engine.rs, policy/mod.rs, registry.rs, resolve.rs, workspace/mod.rs]
    ImportedBy: [footer.rs, runtime_tests_policy.rs, shell/mod.rs]
  # --- Layer 3 -- App / Entrypoints ---
  agents/mod.rs, diagnostics/mod.rs, storage/mod.rs:
    Imports: []
    ImportedBy: [main.rs]
  app/mod.rs:
    Imports: [ui/mod.rs]
    ImportedBy: [main.rs]
  layout/mod.rs:
    Imports: [atoms.rs, boundary.rs, chrome.rs, footer.rs, panels/mod.rs, registry.rs, session/mod.rs, shell/mod.rs]
    ImportedBy: [ui/mod.rs]
  panels/mod.rs:
    Imports: [activity.rs, shell.rs, workspace.rs]
    ImportedBy: [layout/mod.rs]
  ui/mod.rs:
    Imports: [layout/mod.rs]
    ImportedBy: [app/mod.rs, main.rs]
  # --- Tests ---
  runtime_tests_builtins.rs:
    Imports: [boundary.rs, shell/mod.rs]
    ImportedBy: []
  runtime_tests_policy.rs:
    Imports: [boundary.rs, policy/mod.rs, runtime.rs, shell/mod.rs]
    ImportedBy: []
```
