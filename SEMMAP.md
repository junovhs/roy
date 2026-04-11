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

`src/agents/adapter.rs`
Which terminal-native agent product is hosted. [HOTSPOT] [DOMAIN-CONTRACT] [COUPLING:mixed] [BEHAVIOR:sync-primitives,panics-on-error] [QUALITY:undocumented]
Exports: AgentAuthMethod, AgentErrorKind, AgentError.auth_required, AgentError.launch_failed
Touch: Contains inline Rust tests alongside runtime code.
Semantic: synchronized side-effecting that panics on error

`src/agents/claude_code.rs`
Adapter for hosting Claude Code inside the ROY shell. [COUPLING:mixed] [BEHAVIOR:persists,sync-primitives,propagates-errors] [QUALITY:error-boundary]
Exports: ClaudeCodeAdapter.auth_method, ClaudeCodeAdapter.from_path, ClaudeCodeAdapter, ClaudeCodeAdapter.discover
Touch: Contains inline Rust tests alongside runtime code.
Semantic: synchronized side-effecting adapter that propagates errors

`src/agents/session.rs`
Lifecycle state of an embedded-agent session. [COUPLING:mixed] [BEHAVIOR:panics-on-error]
Exports: AgentSessionState, AgentSession.agent_id, AgentSession.is_active, AgentSessionState.is_active
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting that panics on error

`src/capabilities/fs.rs`
Implements fs functionality. [COUPLING:mixed] [BEHAVIOR:persists,propagates-errors] [QUALITY:error-boundary]
Semantic: side-effecting adapter that propagates errors

`src/capabilities/validation.rs`
Implements validation functionality. [COUPLING:mixed] [BEHAVIOR:persists,propagates-errors]
Semantic: side-effecting adapter that propagates errors

`src/commands/fs.rs`
Implements fs functionality. [CORE] [COUPLING:mixed] [BEHAVIOR:owns-state]
Semantic: side-effecting stateful module

`src/commands/registry.rs`
ROY command registry — the explicit, data-driven substitution table. [CORE] [HOTSPOT] [GLOBAL-UTIL] [COUPLING:mixed] [BEHAVIOR:persists,panics-on-error] [QUALITY:error-boundary]
Exports: CommandRegistry.public_help_lines, CommandRegistry.is_empty, CommandRegistry.public_commands, CommandRegistry
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting adapter that panics on error

`src/commands/registry_data.rs`
Implements registry data. [CORE] [COUPLING:pure]
Semantic: pure computation

`src/commands/registry_data/builtins.rs`
Implements builtins functionality. [CORE] [COUPLING:mixed] [BEHAVIOR:owns-state]
Semantic: side-effecting stateful module

`src/commands/registry_data/compat.rs`
Implements compat functionality. [CORE] [COUPLING:mixed] [BEHAVIOR:owns-state]
Semantic: side-effecting stateful module

`src/commands/schema.rs`
How a command is executed once resolved. [CORE] [COUPLING:pure]
Exports: Backend.is_denied, CommandSchema, RiskLevel, Backend
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation

`src/commands/validation.rs`
Implements validation functionality. [CORE] [COUPLING:mixed] [BEHAVIOR:owns-state]
Semantic: side-effecting stateful module

`src/diagnostics/pane.rs`
Severity level for a diagnostics trace entry. [COUPLING:pure]
Exports: build_trace, DiagEntry, DiagSeverity, DiagSeverity.color
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation

`src/policy/engine.rs`
Typed outcome of a policy evaluation. [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:persists]
Exports: PolicyEngine.profile_name, PolicyEngine.set_profile, PolicyEngine.is_allowed, PolicyOutcome.is_blocked
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting adapter

`src/policy/profile.rs`
What the policy engine decides about a particular command invocation. [HOTSPOT] [COUPLING:pure]
Exports: PolicyProfile.is_explicitly_allowed, PolicyProfile.read_only, PolicyProfile.is_blocked, PolicyPermission
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation

`src/session/artifacts.rs`
High-value output classes that ROY promotes above transcript text. [HOTSPOT] [GLOBAL-UTIL] [COUPLING:mixed] [BEHAVIOR:panics-on-error] [QUALITY:undocumented]
Exports: SessionArtifact.denied_command, SessionArtifact.validation_run, ArtifactKind.as_str, SessionArtifact.diff
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting that panics on error

`src/session/engine.rs`
A ROY shell session — owns the ordered event ledger. [COUPLING:mixed] [BEHAVIOR:panics-on-error]
Exports: Session.events_of_kind, Session.is_empty, Session, Session.artifacts
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting that panics on error

`src/session/events.rs`
Milliseconds since UNIX epoch — simple, copy-friendly, SQLite-friendly. [HOTSPOT] [COUPLING:pure]
Exports: SessionEvent.kind_str, SessionEvent, SessionEvent.timestamp
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation

`src/shell/env.rs`
Controlled environment for one ROY shell session. [HOTSPOT] [GLOBAL-UTIL] [COUPLING:mixed] [BEHAVIOR:panics-on-error,propagates-errors] [QUALITY:error-boundary]
Exports: ShellEnv.roy_path, ShellEnv, ShellEnv.chdir, ShellEnv.cwd
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting that panics on error

`src/shell/io.rs`
IO surface for a ROY shell session. [HOTSPOT] [COUPLING:pure] [QUALITY:undocumented]
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

`src/shell/runtime_builtins.rs`
Implements runtime builtins. [COUPLING:pure]
Semantic: pure computation

`src/shell/runtime_native.rs`
Implements runtime native. [COUPLING:pure]
Semantic: pure computation

`src/shell/traps.rs`
Compatibility traps — resolved from the command registry instead of being duplicated in a second static table. [COUPLING:mixed] [BEHAVIOR:panics-on-error]
Exports: compat_trap_message
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting that panics on error

`src/storage/sqlite.rs`
Error returned by storage operations. [COUPLING:mixed] [BEHAVIOR:persists,propagates-errors] [SURFACE:database] [QUALITY:undocumented,error-boundary]
Exports: StoredArtifactRef, RoyStore.load_artifact_refs, RoyStore.append_event, RoyStore.open_memory
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting adapter with database surface that propagates errors

`src/ui/artifacts.rs`
Implements artifacts functionality. [COUPLING:mixed] [BEHAVIOR:owns-state]
Semantic: side-effecting stateful module

`src/ui/layout/artifacts_row.rs`
Implements artifacts row. [COUPLING:mixed] [BEHAVIOR:persists,sync-primitives]
Semantic: synchronized side-effecting adapter

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

`src/ui/layout/panels/command_line.rs`
Minimal shell-like command line parser. [CORE] [COUPLING:mixed] [BEHAVIOR:panics-on-error] [QUALITY:complex-flow,error-boundary]
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting that panics on error

`src/ui/layout/panels/terminal.rs`
Implements terminal functionality. [COUPLING:mixed] [BEHAVIOR:persists,sync-primitives]
Semantic: synchronized side-effecting adapter

`src/ui/layout/panels/terminal_actions.rs`
Implements terminal actions. [COUPLING:mixed] [BEHAVIOR:persists,sync-primitives] [QUALITY:syntax-degraded]
Semantic: synchronized side-effecting adapter

`src/ui/layout/panels/terminal_model.rs`
Module definitions for terminal_model. [TYPE] [COUPLING:mixed] [BEHAVIOR:owns-state]
Semantic: side-effecting stateful module

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
Embedded-agent adapter layer. [ENTRY] [HOTSPOT]
Exports: claude_code, adapter, session

`src/app/mod.rs`
Root application component. [ENTRY] [COUPLING:pure]
Exports: App
Semantic: pure computation

`src/capabilities/mod.rs`
Typed capability runtime for ROY-native commands. [ENTRY] [HOTSPOT] [GLOBAL-UTIL] [COUPLING:pure] [QUALITY:undocumented]
Exports: CapabilityOutput.exit_code, WorkspaceEntry, CapabilityOutput.error_text, CapabilityOutput.primary_text
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation

`src/commands/mod.rs`
Command resolution and substitution registry. [CORE] [HOTSPOT] [GLOBAL-UTIL] [COUPLING:mixed] [BEHAVIOR:persists]
Exports: CommandRegistry, CommandSchema, validation, registry
Semantic: side-effecting adapter

`src/diagnostics/mod.rs`
Developer diagnostics — trace computation and display data for the developer pane. [ENTRY] [HOTSPOT]
Exports: build_trace, pane

`src/main.rs`
Application entry point. [ENTRY] [COUPLING:pure]
Semantic: pure computation

`src/policy/mod.rs`
Policy engine for command and capability execution. [ENTRY] [HOTSPOT] [GLOBAL-UTIL]
Exports: engine, profile

`src/session/mod.rs`
Session and transcript event ledger. [ENTRY] [HOTSPOT] [GLOBAL-UTIL]
Exports: Session, artifacts, engine, events

`src/shell/mod.rs`
Shell host runtime. [ENTRY] [HOTSPOT] [GLOBAL-UTIL] [COUPLING:pure] [QUALITY:undocumented]
Exports: DispatchResult, ShellEnv, ShellError, ShellError.fmt
Semantic: pure computation

`src/storage/mod.rs`
SQLite persistence layer. [ENTRY]
Exports: sqlite

`src/ui/layout/mod.rs`
Root shell cockpit. [ENTRY] [COUPLING:mixed] [BEHAVIOR:owns-state]
Exports: Cockpit
Semantic: side-effecting stateful module

`src/ui/layout/panels/mod.rs`
Module definitions for mod. [ENTRY]

`src/ui/mod.rs`
Provides shared mod used across multiple domains. [ENTRY] [HOTSPOT] [GLOBAL-UTIL]
Exports: layout

`src/workspace/mod.rs`
Workspace management and boundary enforcement. [ENTRY] [HOTSPOT] [GLOBAL-UTIL]
Exports: WorkspaceCwd, boundary

## Layer 4 -- Tests

`src/agents/adapter_tests.rs`
Tests for the embedded-agent adapter contract: AgentKind, AgentHandle lifecycle, and AgentError classifications. [COUPLING:pure]
Semantic: pure computation

`src/agents/claude_code_tests.rs`
Tests for ClaudeCodeAdapter — meta, auth, binary discovery, and PATH isolation. [COUPLING:mixed] [BEHAVIOR:persists]
Semantic: side-effecting adapter

`src/capabilities/capability_tests.rs`
Tests for super. [COUPLING:mixed] [BEHAVIOR:persists,panics-on-error] [QUALITY:error-boundary]
Semantic: side-effecting adapter that panics on error

`src/diagnostics/pane_tests.rs`
Tests for diagnostics::pane — build_trace and DiagSeverity. [COUPLING:pure]
Semantic: pure computation

`src/policy/engine_tests.rs`
Tests for crate. [COUPLING:mixed] [BEHAVIOR:persists]
Semantic: side-effecting adapter

`src/shell/runtime_tests_builtins.rs`
Tests for ShellRuntime built-in command handlers: pwd, cd, env, exit, help, and the transcript drain. [COUPLING:mixed] [BEHAVIOR:panics-on-error] [QUALITY:error-boundary]
Semantic: side-effecting that panics on error

`src/shell/runtime_tests_discoverability.rs`
Tests for ShellRuntime help and discoverability surfaces: help sections, commands builtin, and not_found hint. [COUPLING:mixed] [BEHAVIOR:panics-on-error] [QUALITY:error-boundary]
Semantic: side-effecting that panics on error

`src/shell/runtime_tests_native.rs`
Tests for ShellRuntime ROY-native command dispatch. [COUPLING:mixed] [BEHAVIOR:persists,panics-on-error] [QUALITY:error-boundary]
Semantic: side-effecting adapter that panics on error

`src/shell/runtime_tests_policy.rs`
Tests for ShellRuntime dispatch policy: compatibility traps, NotFound, prompt indicator, transcript errors, and policy engine integration. [COUPLING:mixed] [BEHAVIOR:panics-on-error] [QUALITY:error-boundary]
Semantic: side-effecting that panics on error

`src/storage/store_tests.rs`
Integration tests for RoyStore — save/load roundtrip through SQLite. [COUPLING:mixed] [BEHAVIOR:panics-on-error] [QUALITY:error-boundary]
Semantic: side-effecting that panics on error


## DependencyGraph

```yaml
DependencyGraph:
  # --- Entrypoints ---
  main.rs:
    Imports: [agents/mod.rs, app/mod.rs, boundary.rs, capabilities/mod.rs, commands/mod.rs, diagnostics/mod.rs, policy/mod.rs, session/mod.rs, shell/mod.rs, storage/mod.rs, ui/mod.rs, workspace/mod.rs]
    ImportedBy: []
  # --- High Fan-In Hotspots ---
  boundary.rs:
    Imports: [workspace/mod.rs]
    ImportedBy: [adapter.rs, adapter_tests.rs, capabilities/fs.rs, capabilities/validation.rs, capability_tests.rs, claude_code.rs, command_line.rs, cwd.rs, engine_tests.rs, env.rs, io.rs, layout/mod.rs, main.rs, pane.rs, pane_tests.rs, policy/engine.rs, profile.rs, registry.rs, resolve.rs, runtime.rs, runtime_builtins.rs, runtime_native.rs, runtime_tests_builtins.rs, runtime_tests_discoverability.rs, runtime_tests_native.rs, runtime_tests_policy.rs, session.rs, session/artifacts.rs, session/engine.rs, sqlite.rs, store_tests.rs, terminal_actions.rs, terminal_model.rs, traps.rs, workspace/mod.rs]
  capabilities/mod.rs:
    Imports: [capabilities/fs.rs, capabilities/validation.rs, registry.rs, workspace/mod.rs]
    ImportedBy: [commands/fs.rs, commands/mod.rs, commands/validation.rs, main.rs, runtime.rs, runtime_native.rs]
  commands/mod.rs:
    Imports: [capabilities/mod.rs, commands/fs.rs, commands/validation.rs, registry.rs, schema.rs]
    ImportedBy: [builtins.rs, compat.rs, footer.rs, main.rs, pane.rs, pane_tests.rs, policy/engine.rs, profile.rs, registry.rs, registry_data.rs, resolve.rs, runtime.rs, traps.rs]
  diagnostics/mod.rs:
    Imports: [pane.rs]
    ImportedBy: [footer.rs, main.rs, pane_tests.rs]
  env.rs:
    Imports: [boundary.rs]
    ImportedBy: [claude_code.rs, claude_code_tests.rs, cwd.rs, runtime_builtins.rs, runtime_native.rs, shell/mod.rs, workspace.rs]
  io.rs:
    Imports: [boundary.rs]
    ImportedBy: [runtime.rs, runtime_builtins.rs, runtime_native.rs, shell/mod.rs]
  policy/mod.rs:
    Imports: [policy/engine.rs, profile.rs]
    ImportedBy: [engine_tests.rs, main.rs, runtime.rs, runtime_tests_policy.rs]
  registry.rs:
    Imports: [boundary.rs, commands/mod.rs]
    ImportedBy: [activity.rs, artifacts_row.rs, capabilities/mod.rs, claude_code_tests.rs, command_line.rs, commands/mod.rs, commands/validation.rs, layout/mod.rs, pane.rs, runtime.rs, runtime_builtins.rs, runtime_native.rs, session/artifacts.rs, session/engine.rs, ui/artifacts.rs]
  session/artifacts.rs:
    Imports: [boundary.rs, registry.rs]
    ImportedBy: [adapter.rs, runtime.rs, runtime_native.rs, session/mod.rs]
  session/mod.rs:
    Imports: [events.rs, session/artifacts.rs, session/engine.rs]
    ImportedBy: [activity.rs, artifacts_row.rs, chrome.rs, events.rs, footer.rs, layout/mod.rs, main.rs, pane.rs, pane_tests.rs, result.rs, runtime.rs, runtime_native.rs, session/engine.rs, sqlite.rs, store_tests.rs, terminal.rs, terminal_actions.rs, terminal_model.rs, ui/artifacts.rs, workspace.rs]
  shell/mod.rs:
    Imports: [env.rs, io.rs, resolve.rs, result.rs, runtime.rs, traps.rs]
    ImportedBy: [chrome.rs, footer.rs, layout/mod.rs, main.rs, runtime_builtins.rs, runtime_tests_builtins.rs, runtime_tests_discoverability.rs, runtime_tests_native.rs, runtime_tests_policy.rs, terminal.rs, terminal_actions.rs, terminal_model.rs, workspace.rs]
  ui/mod.rs:
    Imports: [layout/mod.rs, ui/artifacts.rs]
    ImportedBy: [activity.rs, app/mod.rs, artifacts_row.rs, main.rs]
  workspace/mod.rs:
    Imports: [boundary.rs, cwd.rs]
    ImportedBy: [boundary.rs, capabilities/mod.rs, cwd.rs, main.rs, runtime.rs]
  # --- Layer 0 -- Config ---
  Cargo.toml, SEMMAP.md:
    Imports: []
    ImportedBy: []
  # --- Layer 1 -- Domain (Engine) ---
  activity.rs:
    Imports: [registry.rs, session/mod.rs, ui/mod.rs]
    ImportedBy: [panels/mod.rs]
  adapter.rs:
    Imports: [boundary.rs, session/artifacts.rs]
    ImportedBy: [adapter_tests.rs, agents/mod.rs, claude_code.rs]
  artifacts_row.rs:
    Imports: [registry.rs, session/mod.rs, ui/mod.rs]
    ImportedBy: [layout/mod.rs]
  atoms.rs:
    Imports: []
    ImportedBy: [layout/mod.rs]
  builtins.rs, compat.rs, registry_data.rs:
    Imports: [commands/mod.rs]
    ImportedBy: []
  capabilities/fs.rs, capabilities/validation.rs:
    Imports: [boundary.rs]
    ImportedBy: [capabilities/mod.rs]
  chrome.rs:
    Imports: [session/mod.rs, shell/mod.rs]
    ImportedBy: [layout/mod.rs]
  claude_code.rs:
    Imports: [adapter.rs, boundary.rs, env.rs]
    ImportedBy: [agents/mod.rs, claude_code_tests.rs]
  command_line.rs:
    Imports: [boundary.rs, registry.rs]
    ImportedBy: []
  commands/fs.rs:
    Imports: [capabilities/mod.rs]
    ImportedBy: [commands/mod.rs]
  commands/validation.rs:
    Imports: [capabilities/mod.rs, registry.rs]
    ImportedBy: [commands/mod.rs]
  cwd.rs:
    Imports: [boundary.rs, env.rs, workspace/mod.rs]
    ImportedBy: [workspace/mod.rs]
  events.rs:
    Imports: [session/mod.rs]
    ImportedBy: [session/engine.rs, session/mod.rs, store_tests.rs]
  footer.rs:
    Imports: [commands/mod.rs, diagnostics/mod.rs, runtime.rs, session/engine.rs, session/mod.rs, shell/mod.rs]
    ImportedBy: [layout/mod.rs]
  pane.rs:
    Imports: [boundary.rs, commands/mod.rs, registry.rs, session/mod.rs]
    ImportedBy: [diagnostics/mod.rs]
  policy/engine.rs:
    Imports: [boundary.rs, commands/mod.rs, profile.rs]
    ImportedBy: [engine_tests.rs, policy/mod.rs, runtime.rs]
  profile.rs:
    Imports: [boundary.rs, commands/mod.rs]
    ImportedBy: [engine_tests.rs, policy/engine.rs, policy/mod.rs]
  resolve.rs, traps.rs:
    Imports: [boundary.rs, commands/mod.rs]
    ImportedBy: [shell/mod.rs]
  result.rs:
    Imports: [session/mod.rs]
    ImportedBy: [shell/mod.rs]
  runtime_builtins.rs:
    Imports: [boundary.rs, env.rs, io.rs, registry.rs, shell/mod.rs]
    ImportedBy: []
  runtime_native.rs:
    Imports: [boundary.rs, capabilities/mod.rs, env.rs, io.rs, registry.rs, session/artifacts.rs, session/mod.rs]
    ImportedBy: []
  schema.rs:
    Imports: []
    ImportedBy: [commands/mod.rs]
  session.rs:
    Imports: [agents/mod.rs, boundary.rs]
    ImportedBy: [agents/mod.rs]
  session/engine.rs:
    Imports: [agents/mod.rs, boundary.rs, events.rs, registry.rs, session/mod.rs]
    ImportedBy: [footer.rs, session/mod.rs]
  sqlite.rs:
    Imports: [boundary.rs, session/mod.rs]
    ImportedBy: [storage/mod.rs, store_tests.rs]
  terminal.rs:
    Imports: [session/mod.rs, shell/mod.rs]
    ImportedBy: [panels/mod.rs]
  terminal_actions.rs:
    Imports: [boundary.rs, session/mod.rs, shell/mod.rs]
    ImportedBy: []
  terminal_model.rs:
    Imports: [boundary.rs, session/mod.rs, shell/mod.rs]
    ImportedBy: [panels/mod.rs]
  ui/artifacts.rs:
    Imports: [registry.rs, session/mod.rs]
    ImportedBy: [ui/mod.rs]
  workspace.rs:
    Imports: [env.rs, session/mod.rs, shell/mod.rs]
    ImportedBy: [panels/mod.rs]
  # --- Layer 2 -- Adapters / Infra ---
  runtime.rs:
    Imports: [boundary.rs, capabilities/mod.rs, commands/mod.rs, io.rs, policy/engine.rs, policy/mod.rs, registry.rs, session/artifacts.rs, session/mod.rs, workspace/mod.rs]
    ImportedBy: [footer.rs, runtime_tests_policy.rs, shell/mod.rs]
  # --- Layer 3 -- App / Entrypoints ---
  agents/mod.rs:
    Imports: [adapter.rs, claude_code.rs, session.rs]
    ImportedBy: [main.rs, session.rs, session/engine.rs]
  app/mod.rs:
    Imports: [ui/mod.rs]
    ImportedBy: [main.rs]
  layout/mod.rs:
    Imports: [artifacts_row.rs, atoms.rs, boundary.rs, chrome.rs, footer.rs, panels/mod.rs, registry.rs, session/mod.rs, shell/mod.rs]
    ImportedBy: [ui/mod.rs]
  panels/mod.rs:
    Imports: [activity.rs, terminal.rs, terminal_model.rs, workspace.rs]
    ImportedBy: [layout/mod.rs]
  storage/mod.rs:
    Imports: [sqlite.rs]
    ImportedBy: [main.rs]
  # --- Tests ---
  adapter_tests.rs:
    Imports: [adapter.rs, boundary.rs]
    ImportedBy: []
  capability_tests.rs:
    Imports: [boundary.rs]
    ImportedBy: []
  claude_code_tests.rs:
    Imports: [claude_code.rs, env.rs, registry.rs]
    ImportedBy: []
  engine_tests.rs:
    Imports: [boundary.rs, policy/engine.rs, policy/mod.rs, profile.rs]
    ImportedBy: []
  pane_tests.rs:
    Imports: [boundary.rs, commands/mod.rs, diagnostics/mod.rs, session/mod.rs]
    ImportedBy: []
  runtime_tests_builtins.rs, runtime_tests_discoverability.rs, runtime_tests_native.rs:
    Imports: [boundary.rs, shell/mod.rs]
    ImportedBy: []
  runtime_tests_policy.rs:
    Imports: [boundary.rs, policy/mod.rs, runtime.rs, shell/mod.rs]
    ImportedBy: []
  store_tests.rs:
    Imports: [boundary.rs, events.rs, session/mod.rs, sqlite.rs]
    ImportedBy: []
```
