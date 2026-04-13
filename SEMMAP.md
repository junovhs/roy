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

`neti.toml`
Configuration for neti.

## Layer 1 -- Domain (Engine)

`src/agents/adapter.rs`
Implements agent handle.events. [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:sync-primitives,panics-on-error,propagates-errors] [QUALITY:undocumented]
Exports: AgentHandle.send_raw_bytes, AgentHandle.set_pty_master, AgentAuthMethod, AgentErrorKind
Touch: Contains inline Rust tests alongside runtime code.
Semantic: synchronized side-effecting that panics on error

`src/agents/claude_code.rs`
Concrete adapter for hosting Claude Code inside the ROY shell. [HOTSPOT] [COUPLING:pure] [BEHAVIOR:propagates-errors]
Exports: ClaudeCodeAdapter.auth_method, ClaudeCodeAdapter.from_path, ClaudeCodeAdapter, ClaudeCodeAdapter.discover
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation that propagates errors

`src/agents/codex.rs`
Adapter for hosting the Codex CLI inside the ROY shell. [COUPLING:pure] [BEHAVIOR:propagates-errors]
Exports: CodexAdapter.auth_method, CodexAdapter.from_path, CodexAdapter, CodexAdapter.discover
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation that propagates errors

`src/agents/host.rs`
Terminal dimensions reported to the hosted agent. [COUPLING:mixed] [BEHAVIOR:owns-state,persists,sync-primitives,propagates-errors] [QUALITY:error-boundary]
Touch: Contains inline Rust tests alongside runtime code.
Semantic: synchronized side-effecting stateful adapter that propagates errors

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

`src/commands/ast.rs`
Typed AST for ROY v0.2 commands. [CORE] [HOTSPOT] [COUPLING:pure] [BEHAVIOR:propagates-errors]
Exports: Command.to_argv, LineRange, ParseError, ParseError.fmt
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation that propagates errors

`src/commands/ast_tokenise.rs`
Raw tokeniser for the ROY v0.2 command grammar. [CORE] [COUPLING:pure] [BEHAVIOR:propagates-errors]
Semantic: pure computation that propagates errors

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
Severity level for a diagnostics trace entry. [HOTSPOT] [COUPLING:pure]
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
High-value output classes that ROY promotes above transcript text. [HOTSPOT] [GLOBAL-UTIL] [COUPLING:pure] [QUALITY:undocumented]
Exports: SessionArtifact.denied_command, SessionArtifact.validation_run, ArtifactKind.as_str, SessionArtifact.diff
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation

`src/session/engine.rs`
A ROY shell session — owns the ordered event ledger. [HOTSPOT] [COUPLING:pure]
Exports: Session.events_of_kind, Session.is_empty, Session, Session.artifacts
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation

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

`src/shell/runtime_agent.rs`
Implements shellruntime.poll agent events. [HOTSPOT] [COUPLING:pure]
Exports: ShellRuntime.poll_agent_events, ShellRuntime.poll_agent_lines, ShellRuntime.send_agent_input, ShellRuntime.send_agent_raw
Semantic: pure computation

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
Error returned by storage operations. [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:persists,propagates-errors] [SURFACE:database] [QUALITY:undocumented,error-boundary]
Exports: StoredArtifactRef, RoyStore.load_artifact_refs, RoyStore.append_event, RoyStore.open_memory
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting adapter with database surface that propagates errors

`src/storage/sqlite_issues.rs`
Issue storage APIs. [HOTSPOT] [COUPLING:pure] [BEHAVIOR:propagates-errors] [SURFACE:database] [QUALITY:error-boundary]
Exports: RoyStore.list_open_issues, RoyStore.list_issues, IssueRecord, RoyStore.insert_issue
Semantic: pure computation with database surface that propagates errors

`src/storage/sqlite_refs.rs`
Named-ref storage APIs. [HOTSPOT] [COUPLING:pure] [BEHAVIOR:propagates-errors] [SURFACE:database] [QUALITY:error-boundary]
Exports: RoyStore.list_named_refs, NamedRefRecord, RoyStore.load_named_ref, RoyStore.upsert_named_ref
Semantic: pure computation with database surface that propagates errors

`src/ui/artifacts.rs`
Implements artifacts functionality. [COUPLING:mixed] [BEHAVIOR:owns-state]
Semantic: side-effecting stateful module

`src/ui/layout/cockpit.rs`
Implements Cockpit functionality. [COUPLING:mixed] [BEHAVIOR:persists,sync-primitives]
Exports: Cockpit
Semantic: synchronized side-effecting adapter

`src/ui/layout/` (8 files: 8 .rs)
Representative: src/ui/layout/activity_drawer.rs, src/ui/layout/artifacts_row.rs

`src/ui/layout/panels/command_line.rs`
Implements command line. [CORE] [COUPLING:mixed] [BEHAVIOR:panics-on-error,propagates-errors] [QUALITY:error-boundary]
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting that panics on error

`src/ui/layout/panels/terminal_grid.rs`
alacritty_terminal state helpers for the agent terminal view. [COUPLING:mixed] [BEHAVIOR:owns-state,sync-primitives,panics-on-error] [QUALITY:undocumented]
Exports: TermDims.columns, TermListener.send_event, TermDims.screen_lines, TermDims.total_lines
Touch: Contains inline Rust tests alongside runtime code.
Semantic: synchronized side-effecting stateful module that panics on error

`src/ui/layout/panels/terminal_model.rs`
Visual classification of a terminal line, driving rendering in terminal.rs. [TYPE] [COUPLING:mixed] [BEHAVIOR:owns-state]
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting stateful module

`src/ui/layout/panels/` (12 files: 12 .rs)
Representative: src/ui/layout/panels/activity.rs, src/ui/layout/panels/terminal.rs

`src/ui/layout/resize.rs`
Implements resize functionality. [COUPLING:pure]
Semantic: pure computation

`src/ui/layout/review_drawer.rs`
Implements review drawer. [COUPLING:mixed] [BEHAVIOR:persists,sync-primitives]
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

`src/commands/ast_parse.rs`
Parser for the ROY v0.2 command grammar. [UTIL] [HOTSPOT] [GLOBAL-UTIL] [COUPLING:pure] [BEHAVIOR:propagates-errors] [QUALITY:error-boundary]
Exports: parse
Semantic: pure computation that propagates errors

`src/commands/ast_parse_helpers.rs`
Implements ast parse helpers. [UTIL] [COUPLING:pure] [BEHAVIOR:propagates-errors] [QUALITY:error-boundary]
Semantic: pure computation that propagates errors

`src/shell/runtime.rs`
ROY shell runtime. [CORE] [HOTSPOT] [COUPLING:pure]
Exports: ShellRuntime.public_command_count, ShellRuntime.last_exit_status, ShellRuntime.set_exit_status, ShellRuntime.agent_active
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation

`src/storage/sqlite_lang.rs`
Denial and approval storage APIs. [HOTSPOT] [COUPLING:pure] [BEHAVIOR:propagates-errors] [SURFACE:database] [QUALITY:error-boundary]
Exports: RoyStore.list_all_approvals, RoyStore.list_pending_approvals, RoyStore.insert_pending_approval, RoyStore.list_denials
Semantic: pure computation with database surface that propagates errors

`src/ui/layout/helpers.rs`
Implements helpers functionality. [UTIL] [COUPLING:mixed] [BEHAVIOR:panics-on-error]
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting that panics on error

`src/ui/layout/panels/terminal_grid_render.rs`
Implements terminal grid render. [UTIL] [COUPLING:pure]
Semantic: pure computation

`src/ui/layout/panels/terminal_submit_parse.rs`
Implements terminal submit parse. [UTIL] [COUPLING:mixed] [BEHAVIOR:persists,sync-primitives]
Semantic: synchronized side-effecting adapter

## Layer 3 -- App / Entrypoints

`src/agents/mod.rs`
Embedded-agent adapter layer. [ENTRY] [HOTSPOT] [GLOBAL-UTIL]
Exports: claude_code, adapter, codex, session
Touch: Contains inline Rust tests alongside runtime code.

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
Developer diagnostics — trace computation and display data for the developer pane. [ENTRY]
Exports: pane

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
Implements Cockpit functionality. [ENTRY] [COUPLING:mixed] [BEHAVIOR:owns-state]
Exports: Cockpit
Semantic: side-effecting stateful module

`src/ui/layout/panels/mod.rs`
Module definitions for mod. [ENTRY]

`src/ui/mod.rs`
Re-exports the public API surface. [ENTRY] [HOTSPOT] [GLOBAL-UTIL]
Exports: layout

`src/workspace/mod.rs`
Workspace management and boundary enforcement. [ENTRY] [HOTSPOT] [GLOBAL-UTIL] [COUPLING:pure]
Exports: normalize_host_path, WorkspaceCwd, boundary
Semantic: pure computation

## Layer 4 -- Tests

`src/agents/adapter_tests.rs`
Tests for the embedded-agent adapter contract: AgentKind, AgentHandle lifecycle, and AgentError classifications. [HOTSPOT] [GLOBAL-UTIL] [COUPLING:mixed] [BEHAVIOR:sync-primitives,panics-on-error]
Exports: SharedWriter.flush, SharedWriter.write
Semantic: synchronized side-effecting that panics on error

`src/agents/agent_contract_tests.rs`
Contract-comparison tests for the generic AgentAdapter layer. [COUPLING:mixed] [BEHAVIOR:panics-on-error]
Semantic: side-effecting that panics on error

`src/agents/claude_code_tests.rs`
Tests for ClaudeCodeAdapter — meta, auth, binary discovery, and PATH isolation. [COUPLING:pure]
Semantic: pure computation

`src/agents/codex_tests.rs`
Tests for CodexAdapter — meta, auth, binary discovery, and PATH isolation. [COUPLING:pure]
Semantic: pure computation

`src/capabilities/capability_tests.rs`
Tests for super. [COUPLING:mixed] [BEHAVIOR:persists,panics-on-error] [QUALITY:error-boundary]
Semantic: side-effecting adapter that panics on error

`src/commands/ast_tests.rs`
Tests for super. [COUPLING:mixed] [BEHAVIOR:panics-on-error]
Semantic: side-effecting that panics on error

`src/commands/ast_tests_basic.rs`
Tests for super. [COUPLING:pure]
Semantic: pure computation

`src/commands/ast_tests_filters.rs`
Tests for super. [COUPLING:mixed] [BEHAVIOR:panics-on-error]
Semantic: side-effecting that panics on error

`src/commands/ast_tests_refiners.rs`
Tests for super. [COUPLING:pure]
Semantic: pure computation

`src/diagnostics/pane_tests.rs`
Tests for diagnostics::pane — build_trace and DiagSeverity. [COUPLING:pure]
Semantic: pure computation

`src/policy/engine_tests.rs`
Tests for crate. [COUPLING:mixed] [BEHAVIOR:persists]
Semantic: side-effecting adapter

`src/session/artifacts_tests.rs`
Tests for super. [COUPLING:mixed] [BEHAVIOR:panics-on-error]
Semantic: side-effecting that panics on error

`src/session/engine_tests.rs`
Tests for crate. [COUPLING:mixed] [BEHAVIOR:panics-on-error]
Semantic: side-effecting that panics on error

`src/shell/runtime_tests_agent.rs`
Tests for agent launch dispatch: already-running guard, not-installed path, and auth-gate regressions. [COUPLING:mixed] [BEHAVIOR:persists,sync-primitives,panics-on-error] [QUALITY:error-boundary]
Exports: SharedWriter.flush, SharedWriter.write
Semantic: synchronized side-effecting adapter that panics on error

`src/shell/` (4 files: 4 .rs)
Representative: src/shell/runtime_tests_builtins.rs, src/shell/runtime_tests_discoverability.rs

`src/storage/` (6 files: 6 .rs)
Representative: src/storage/store_tests.rs, src/storage/store_tests_lang.rs

`src/ui/layout/panels/terminal_model_tests.rs`
Tests for crate. [COUPLING:pure]
Semantic: pure computation

`src/ui/layout/panels/terminal_model_tests_parse.rs`
Tests for dioxus. [COUPLING:mixed] [BEHAVIOR:persists,sync-primitives]
Semantic: synchronized side-effecting adapter

`src/ui/layout/panels/terminal_model_tests_submit.rs`
Implements shared writer.write. [COUPLING:mixed] [BEHAVIOR:persists,sync-primitives,panics-on-error]
Exports: SharedWriter.flush, SharedWriter.write
Semantic: synchronized side-effecting adapter that panics on error


## DependencyGraph

```yaml
DependencyGraph:
  # --- Entrypoints ---
  main.rs:
    Imports: [agents/mod.rs, app/mod.rs, ast.rs, boundary.rs, capabilities/mod.rs, commands/mod.rs, diagnostics/mod.rs, policy/mod.rs, session/mod.rs, shell/mod.rs, storage/mod.rs, ui/mod.rs, workspace/mod.rs]
    ImportedBy: []
  # --- High Fan-In Hotspots ---
  agents/mod.rs:
    Imports: [adapter.rs, claude_code.rs, codex.rs, host.rs, session.rs]
    ImportedBy: [helpers.rs, main.rs, runtime.rs, runtime_agent.rs, runtime_tests_agent.rs, session.rs, session/engine_tests.rs, terminal_view.rs]
  ast_parse.rs:
    Imports: [boundary.rs, registry.rs]
    ImportedBy: [ast_parse_helpers.rs, ast_tests.rs, command_line.rs, runtime_builtins.rs]
  boundary.rs:
    Imports: [workspace/mod.rs]
    ImportedBy: [adapter.rs, adapter_tests.rs, artifacts_tests.rs, ast.rs, ast_parse.rs, ast_parse_helpers.rs, ast_tokenise.rs, capabilities/fs.rs, capabilities/validation.rs, capability_tests.rs, cockpit.rs, cwd.rs, env.rs, helpers.rs, host.rs, io.rs, main.rs, pane.rs, pane_tests.rs, policy/engine.rs, policy/engine_tests.rs, profile.rs, registry.rs, resolve.rs, runtime.rs, runtime_agent.rs, runtime_builtins.rs, runtime_native.rs, runtime_tests_agent.rs, runtime_tests_builtins.rs, runtime_tests_discoverability.rs, runtime_tests_native.rs, runtime_tests_policy.rs, session.rs, session/artifacts.rs, session/engine.rs, session/engine_tests.rs, sqlite.rs, store_tests.rs, store_tests_lang.rs, store_tests_lang_migrations.rs, store_tests_lang_refs.rs, terminal_grid.rs, terminal_model.rs, terminal_model_tests.rs, terminal_model_tests_submit.rs, terminal_spans.rs, terminal_submit_handle.rs, terminal_submit_parse.rs, terminal_view.rs, traps.rs, workspace/mod.rs]
  capabilities/mod.rs:
    Imports: [capabilities/fs.rs, capabilities/validation.rs, registry.rs, workspace/mod.rs]
    ImportedBy: [capability_tests.rs, commands/fs.rs, commands/mod.rs, commands/validation.rs, host.rs, main.rs, runtime.rs, runtime_native.rs]
  claude_code.rs:
    Imports: []
    ImportedBy: [agent_contract_tests.rs, agents/mod.rs, claude_code_tests.rs, codex_tests.rs, runtime_agent.rs]
  commands/mod.rs:
    Imports: [ast.rs, capabilities/mod.rs, commands/fs.rs, commands/validation.rs, registry.rs, schema.rs]
    ImportedBy: [builtins.rs, capability_tests.rs, compat.rs, footer.rs, main.rs, pane.rs, pane_tests.rs, policy/engine.rs, profile.rs, registry.rs, registry_data.rs, resolve.rs, runtime.rs, traps.rs]
  env.rs:
    Imports: [boundary.rs, workspace/mod.rs]
    ImportedBy: [claude_code_tests.rs, codex_tests.rs, cwd.rs, diag_drawer.rs, host.rs, runtime_builtins.rs, runtime_native.rs, runtime_tests_builtins.rs, runtime_tests_native.rs, runtime_tests_policy.rs, shell/mod.rs, workspace.rs]
  events.rs:
    Imports: [session/mod.rs]
    ImportedBy: [session/engine.rs, session/engine_tests.rs, session/mod.rs, store_tests.rs]
  io.rs:
    Imports: [boundary.rs]
    ImportedBy: [runtime.rs, runtime_agent.rs, runtime_builtins.rs, runtime_native.rs, shell/mod.rs]
  policy/mod.rs:
    Imports: [policy/engine.rs, profile.rs]
    ImportedBy: [main.rs, policy/engine_tests.rs, runtime.rs, runtime_tests_policy.rs]
  registry.rs:
    Imports: [boundary.rs, commands/mod.rs]
    ImportedBy: [activity.rs, activity_drawer.rs, artifacts_row.rs, ast_parse.rs, ast_parse_helpers.rs, capabilities/mod.rs, claude_code_tests.rs, codex_tests.rs, commands/mod.rs, commands/validation.rs, helpers.rs, pane.rs, runtime.rs, runtime_agent.rs, runtime_builtins.rs, runtime_native.rs, session/artifacts.rs, session/engine.rs, terminal_spans.rs, terminal_submit_lines.rs, terminal_view.rs, ui/artifacts.rs]
  runtime.rs:
    Imports: [agents/mod.rs, boundary.rs, capabilities/mod.rs, commands/mod.rs, io.rs, policy/engine.rs, policy/mod.rs, registry.rs, session/artifacts.rs, session/mod.rs, workspace/mod.rs]
    ImportedBy: [diag_drawer.rs, footer.rs, runtime_tests_policy.rs, shell/mod.rs, terminal_submit_handle.rs, terminal_view.rs]
  session/artifacts.rs:
    Imports: [boundary.rs, registry.rs]
    ImportedBy: [adapter.rs, artifacts_tests.rs, ast_parse_helpers.rs, runtime.rs, runtime_native.rs, session/mod.rs, terminal_model_tests_submit.rs]
  session/mod.rs:
    Imports: [events.rs, session/artifacts.rs, session/engine.rs]
    ImportedBy: [activity.rs, activity_drawer.rs, artifacts_row.rs, attention_drawer.rs, chrome.rs, diag_drawer.rs, events.rs, footer.rs, helpers.rs, main.rs, pane.rs, pane_tests.rs, result.rs, review_drawer.rs, runtime.rs, runtime_agent.rs, runtime_native.rs, session/engine_tests.rs, sqlite.rs, store_tests.rs, store_tests_lang.rs, terminal.rs, terminal_model_tests.rs, terminal_model_tests_submit.rs, terminal_submit_handle.rs, terminal_submit_parse.rs, terminal_submit_record.rs, terminal_view.rs, ui/artifacts.rs, workspace.rs]
  shell/mod.rs:
    Imports: [env.rs, io.rs, resolve.rs, result.rs, runtime.rs, traps.rs]
    ImportedBy: [chrome.rs, cockpit.rs, diag_drawer.rs, footer.rs, main.rs, runtime_builtins.rs, runtime_tests_agent.rs, runtime_tests_builtins.rs, runtime_tests_discoverability.rs, runtime_tests_native.rs, runtime_tests_policy.rs, terminal.rs, terminal_model_tests.rs, terminal_submit_record.rs, terminal_view.rs, workspace.rs]
  sqlite.rs:
    Imports: [boundary.rs, session/mod.rs]
    ImportedBy: [storage/mod.rs, store_tests.rs, store_tests_lang.rs, store_tests_lang_migrations.rs]
  ui/mod.rs:
    Imports: [layout/mod.rs]
    ImportedBy: [activity.rs, app/mod.rs, artifacts_row.rs, main.rs]
  workspace/mod.rs:
    Imports: [boundary.rs, cwd.rs]
    ImportedBy: [boundary.rs, capabilities/mod.rs, cwd.rs, env.rs, main.rs, runtime.rs, runtime_tests_builtins.rs, runtime_tests_policy.rs]
  # --- Layer 0 -- Config ---
  Cargo.toml, SEMMAP.md, neti.toml:
    Imports: []
    ImportedBy: []
  # --- Layer 1 -- Domain (Engine) ---
  activity.rs, artifacts_row.rs:
    Imports: [registry.rs, session/mod.rs, ui/mod.rs]
    ImportedBy: []
  activity_drawer.rs:
    Imports: [registry.rs, session/mod.rs]
    ImportedBy: [layout/mod.rs]
  adapter.rs:
    Imports: [adapter_tests.rs, boundary.rs, session/artifacts.rs]
    ImportedBy: [agents/mod.rs, host.rs, runtime_agent.rs]
  ast.rs:
    Imports: [boundary.rs]
    ImportedBy: [command_line.rs, commands/mod.rs, main.rs]
  ast_tokenise.rs, terminal_grid.rs:
    Imports: [boundary.rs]
    ImportedBy: []
  atoms.rs, terminal_colors.rs, terminal_composer.rs, terminal_line.rs, terminal_submit.rs:
    Imports: []
    ImportedBy: []
  attention_drawer.rs, review_drawer.rs:
    Imports: [session/mod.rs]
    ImportedBy: [layout/mod.rs]
  builtins.rs, compat.rs, registry_data.rs:
    Imports: [commands/mod.rs]
    ImportedBy: []
  capabilities/fs.rs:
    Imports: [adapter_tests.rs, boundary.rs]
    ImportedBy: [capabilities/mod.rs]
  capabilities/validation.rs:
    Imports: [boundary.rs]
    ImportedBy: [capabilities/mod.rs]
  chrome.rs:
    Imports: [session/mod.rs, shell/mod.rs]
    ImportedBy: [layout/mod.rs]
  cockpit.rs:
    Imports: [boundary.rs, shell/mod.rs]
    ImportedBy: [layout/mod.rs]
  codex.rs:
    Imports: []
    ImportedBy: [agents/mod.rs]
  command_line.rs:
    Imports: [ast.rs, ast_parse.rs]
    ImportedBy: [panels/mod.rs]
  commands/fs.rs:
    Imports: [capabilities/mod.rs]
    ImportedBy: [commands/mod.rs]
  commands/validation.rs:
    Imports: [capabilities/mod.rs, registry.rs]
    ImportedBy: [commands/mod.rs]
  cwd.rs:
    Imports: [boundary.rs, env.rs, workspace/mod.rs]
    ImportedBy: [workspace/mod.rs]
  diag_drawer.rs:
    Imports: [env.rs, runtime.rs, session/mod.rs, shell/mod.rs]
    ImportedBy: [layout/mod.rs]
  drawer_shell.rs, resize.rs:
    Imports: []
    ImportedBy: [layout/mod.rs]
  footer.rs:
    Imports: [commands/mod.rs, diagnostics/mod.rs, pane.rs, runtime.rs, session/engine.rs, session/mod.rs, shell/mod.rs]
    ImportedBy: []
  host.rs:
    Imports: [adapter.rs, boundary.rs, capabilities/mod.rs, env.rs]
    ImportedBy: [agents/mod.rs]
  pane.rs:
    Imports: [boundary.rs, commands/mod.rs, registry.rs, session/mod.rs]
    ImportedBy: [diagnostics/mod.rs, footer.rs, pane_tests.rs]
  policy/engine.rs:
    Imports: [boundary.rs, commands/mod.rs, profile.rs]
    ImportedBy: [policy/engine_tests.rs, policy/mod.rs, runtime.rs]
  profile.rs:
    Imports: [boundary.rs, commands/mod.rs]
    ImportedBy: [policy/engine.rs, policy/engine_tests.rs, policy/mod.rs]
  resolve.rs, traps.rs:
    Imports: [boundary.rs, commands/mod.rs]
    ImportedBy: [shell/mod.rs]
  result.rs:
    Imports: [session/mod.rs]
    ImportedBy: [shell/mod.rs]
  runtime_agent.rs:
    Imports: [adapter.rs, agents/mod.rs, boundary.rs, claude_code.rs, io.rs, registry.rs, session/mod.rs]
    ImportedBy: [runtime_tests_agent.rs, terminal_submit_handle.rs, terminal_view.rs]
  runtime_builtins.rs:
    Imports: [ast_parse.rs, boundary.rs, env.rs, io.rs, registry.rs, shell/mod.rs]
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
    Imports: [boundary.rs, events.rs, registry.rs]
    ImportedBy: [footer.rs, session/engine_tests.rs, session/mod.rs]
  sqlite_issues.rs:
    Imports: []
    ImportedBy: [store_tests_lang_issues.rs, store_tests_lang_lang.rs, store_tests_lang_migrations.rs]
  sqlite_refs.rs:
    Imports: []
    ImportedBy: [store_tests_lang_lang.rs, store_tests_lang_migrations.rs, store_tests_lang_refs.rs]
  terminal.rs:
    Imports: [session/mod.rs, shell/mod.rs]
    ImportedBy: [panels/mod.rs]
  terminal_model.rs:
    Imports: [boundary.rs]
    ImportedBy: [panels/mod.rs]
  terminal_spans.rs:
    Imports: [boundary.rs, registry.rs]
    ImportedBy: []
  terminal_submit_handle.rs:
    Imports: [adapter_tests.rs, boundary.rs, runtime.rs, runtime_agent.rs, session/mod.rs]
    ImportedBy: []
  terminal_submit_lines.rs:
    Imports: [registry.rs]
    ImportedBy: []
  terminal_submit_record.rs:
    Imports: [session/mod.rs, shell/mod.rs]
    ImportedBy: []
  terminal_view.rs:
    Imports: [adapter_tests.rs, agents/mod.rs, boundary.rs, registry.rs, runtime.rs, runtime_agent.rs, session/mod.rs, shell/mod.rs]
    ImportedBy: []
  ui/artifacts.rs:
    Imports: [registry.rs, session/mod.rs]
    ImportedBy: []
  workspace.rs:
    Imports: [env.rs, session/mod.rs, shell/mod.rs]
    ImportedBy: []
  # --- Layer 2 -- Adapters / Infra ---
  ast_parse_helpers.rs:
    Imports: [ast_parse.rs, boundary.rs, registry.rs, session/artifacts.rs]
    ImportedBy: []
  helpers.rs:
    Imports: [agents/mod.rs, boundary.rs, registry.rs, session/mod.rs]
    ImportedBy: [layout/mod.rs]
  sqlite_lang.rs:
    Imports: []
    ImportedBy: [store_tests_lang_lang.rs, store_tests_lang_migrations.rs]
  terminal_grid_render.rs:
    Imports: []
    ImportedBy: []
  terminal_submit_parse.rs:
    Imports: [adapter_tests.rs, boundary.rs, session/mod.rs]
    ImportedBy: []
  # --- Layer 3 -- App / Entrypoints ---
  app/mod.rs:
    Imports: [ui/mod.rs]
    ImportedBy: [main.rs]
  diagnostics/mod.rs:
    Imports: [pane.rs]
    ImportedBy: [footer.rs, main.rs]
  layout/mod.rs:
    Imports: [activity_drawer.rs, attention_drawer.rs, chrome.rs, cockpit.rs, diag_drawer.rs, drawer_shell.rs, helpers.rs, panels/mod.rs, resize.rs, review_drawer.rs]
    ImportedBy: [ui/mod.rs]
  panels/mod.rs:
    Imports: [command_line.rs, terminal.rs, terminal_model.rs]
    ImportedBy: [layout/mod.rs]
  storage/mod.rs:
    Imports: [sqlite.rs]
    ImportedBy: [main.rs]
  # --- Tests ---
  adapter_tests.rs:
    Imports: [boundary.rs]
    ImportedBy: [adapter.rs, capabilities/fs.rs, capability_tests.rs, runtime_tests_native.rs, terminal_model_tests_submit.rs, terminal_submit_handle.rs, terminal_submit_parse.rs, terminal_view.rs]
  agent_contract_tests.rs:
    Imports: [claude_code.rs]
    ImportedBy: []
  artifacts_tests.rs:
    Imports: [boundary.rs, session/artifacts.rs]
    ImportedBy: []
  ast_tests.rs:
    Imports: [ast_parse.rs]
    ImportedBy: []
  ast_tests_basic.rs, ast_tests_filters.rs, ast_tests_refiners.rs, terminal_model_tests_parse.rs:
    Imports: []
    ImportedBy: []
  capability_tests.rs:
    Imports: [adapter_tests.rs, boundary.rs, capabilities/mod.rs, commands/mod.rs]
    ImportedBy: []
  claude_code_tests.rs, codex_tests.rs:
    Imports: [claude_code.rs, env.rs, registry.rs]
    ImportedBy: []
  pane_tests.rs:
    Imports: [boundary.rs, commands/mod.rs, pane.rs, session/mod.rs]
    ImportedBy: []
  policy/engine_tests.rs:
    Imports: [boundary.rs, policy/engine.rs, policy/mod.rs, profile.rs]
    ImportedBy: []
  runtime_tests_agent.rs:
    Imports: [agents/mod.rs, boundary.rs, runtime_agent.rs, shell/mod.rs]
    ImportedBy: []
  runtime_tests_builtins.rs:
    Imports: [boundary.rs, env.rs, shell/mod.rs, workspace/mod.rs]
    ImportedBy: []
  runtime_tests_discoverability.rs:
    Imports: [boundary.rs, shell/mod.rs]
    ImportedBy: []
  runtime_tests_native.rs:
    Imports: [adapter_tests.rs, boundary.rs, env.rs, shell/mod.rs]
    ImportedBy: []
  runtime_tests_policy.rs:
    Imports: [boundary.rs, env.rs, policy/mod.rs, runtime.rs, shell/mod.rs, workspace/mod.rs]
    ImportedBy: []
  session/engine_tests.rs:
    Imports: [agents/mod.rs, boundary.rs, events.rs, session/engine.rs, session/mod.rs]
    ImportedBy: []
  store_tests.rs:
    Imports: [boundary.rs, events.rs, session/mod.rs, sqlite.rs]
    ImportedBy: []
  store_tests_lang.rs:
    Imports: [boundary.rs, session/mod.rs, sqlite.rs]
    ImportedBy: []
  store_tests_lang_issues.rs:
    Imports: [sqlite_issues.rs]
    ImportedBy: []
  store_tests_lang_lang.rs:
    Imports: [sqlite_issues.rs, sqlite_lang.rs, sqlite_refs.rs]
    ImportedBy: []
  store_tests_lang_migrations.rs:
    Imports: [boundary.rs, sqlite.rs, sqlite_issues.rs, sqlite_lang.rs, sqlite_refs.rs]
    ImportedBy: []
  store_tests_lang_refs.rs:
    Imports: [boundary.rs, sqlite_refs.rs]
    ImportedBy: []
  terminal_model_tests.rs:
    Imports: [boundary.rs, session/mod.rs, shell/mod.rs]
    ImportedBy: []
  terminal_model_tests_submit.rs:
    Imports: [adapter_tests.rs, boundary.rs, session/artifacts.rs, session/mod.rs]
    ImportedBy: []
```
