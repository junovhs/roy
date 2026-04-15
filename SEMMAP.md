# neti -- Semantic Map

**Purpose:** Terminal

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

`README.md`
Project overview and usage guide.

`SEMMAP.md`
Generated semantic map.

`corpus/manifest.toml`
Configuration for manifest.

`corpus/results/run_1.md`
Support file for the results subsystem.

`neti.toml`
Configuration for neti.

`refactor.md`
Support file for refactor.

`src/config/io.rs`
utility for io via file I/O. [COUPLING:mixed] [BEHAVIOR:persists,propagates-errors] [QUALITY:undocumented]
Exports: apply_project_defaults, process_ignore_line, save_to_file, load_toml_config
Semantic: side-effecting adapter that propagates errors

`src/config/locality.rs`
Configuration for the Law of Locality enforcement. [HOTSPOT] [GLOBAL-UTIL] [COUPLING:pure]
Exports: LocalityConfig.is_error_mode, LocalityConfig.to_validator_config, LocalityConfig.default, LocalityConfig.is_enabled
Semantic: pure computation

`src/config/types.rs`
Implements safety config.default. [COUPLING:pure] [QUALITY:undocumented]
Exports: CommandEntry, CommandEntry.into_vec, NetiToml, RuleConfig.default
Semantic: pure computation

`src/graph/tsconfig.rs`
Parser for tsconfig.json / jsconfig.json path mappings. [HOTSPOT] [GLOBAL-UTIL] [COUPLING:mixed] [BEHAVIOR:persists,propagates-errors] [QUALITY:error-boundary]
Exports: TsConfig, TsConfig.load, TsConfig.resolve
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting adapter that propagates errors

## Layer 1 -- Domain (Engine)

`corpus/results/summary.json`
Implements summary functionality. data.

`src/analysis/ast.rs`
Implements analysis result. [COUPLING:pure] [QUALITY:undocumented]
Exports: AnalysisResult, Analyzer, Analyzer.analyze, Analyzer.default
Semantic: pure computation

`src/analysis/checks.rs`
AST-based complexity and style checks.
Exports: check_dead_params, CheckContext, check_banned, check_syntax

`src/analysis/checks/banned.rs`
Banned construct checks (Law of Paranoia). [COUPLING:mixed] [BEHAVIOR:persists,sync-primitives]
Exports: check_banned
Touch: Contains inline Rust tests alongside runtime code.
Semantic: synchronized side-effecting adapter

`src/analysis/checks/dead_params.rs`
Dead parameter detection (EPOCH-01f). [COUPLING:mixed] [BEHAVIOR:owns-state] [SURFACE:http-handler] [QUALITY:complex-flow]
Exports: check_dead_params
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting stateful module with HTTP handler surface

`src/analysis/checks/syntax.rs`
AST-level syntax error and malformed node detection. [HOTSPOT] [COUPLING:pure]
Exports: check_syntax
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation

`src/analysis/checks/syntax_rust.rs`
Implements syntax rust. [COUPLING:pure]
Semantic: pure computation

`src/analysis/checks/syntax_ts.rs`
Implements syntax ts. [COUPLING:pure]
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation

`src/analysis/engine.rs`
Main execution logic for the `Neti` analysis engine. [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:persists]
Exports: Engine.scan_with_progress, Engine, Engine.scan
Semantic: side-effecting adapter

`src/analysis/patterns/paranoia_universal.rs`
Universal cross-language `LAW OF PARANOIA` checks. [COUPLING:pure] [QUALITY:complex-flow]
Exports: check_paranoia
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation

`src/analysis/patterns/semantic.rs`
Actionable semantic checks: M03 and M05. [HOTSPOT] [COUPLING:pure]
Exports: detect_universal, detect
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation

`src/analysis/patterns/semantic_universal.rs`
Universal semantic detectors for M03 and M05. [COUPLING:pure]
Exports: detect_semantic_universal
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation

`src/analysis/safety.rs`
Validates safety source. [COUPLING:mixed] [BEHAVIOR:persists]
Exports: check_safety_source, check_safety
Semantic: side-effecting adapter

`src/analysis/suppress.rs`
Inline suppression parsing: `// neti:allow(CODE) reason`. [COUPLING:pure] [BEHAVIOR:propagates-errors] [QUALITY:complex-flow]
Exports: apply
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation that propagates errors

`src/analysis/worker.rs`
Worker module for file parsing and analysis. [COUPLING:mixed] [BEHAVIOR:persists]
Exports: is_ignored, scan_file
Semantic: side-effecting adapter

`src/baseline.rs`
.neti-baseline.json` — violation snapshot for staged adoption. [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:owns-state,persists,propagates-errors]
Exports: Baseline.apply_to_files, Baseline.from_scan, BASELINE_FILE, BaselineEntry
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting stateful adapter that propagates errors

`src/cli/dispatch.rs`
Command dispatch logic extracted from binary to reduce main function size. [COUPLING:pure] [BEHAVIOR:propagates-errors]
Exports: execute
Semantic: pure computation that propagates errors

`src/cli/handlers/check_report.rs`
Report building and scorecard display for `neti check`. [COUPLING:mixed] [BEHAVIOR:persists,panics-on-error] [QUALITY:complex-flow,error-boundary]
Exports: build_full_detail_text, write_fix_packet, build_summary_text, print_commands_scorecard
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting adapter that panics on error

`src/cli/handlers/dead.rs`
Handler for `neti dead` — workspace-level dead code detection. [COUPLING:pure]
Exports: handle_dead
Semantic: pure computation

`src/cli/handlers/inspect.rs`
model for inspect via file I/O. [TYPE] [COUPLING:mixed] [BEHAVIOR:persists,propagates-errors] [QUALITY:undocumented]
Exports: DocCommentLink, InspectCapability, InspectReport, handle_inspect
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting adapter that propagates errors

`src/cli/handlers/scan_report.rs`
Scan report display formatting. [COUPLING:pure]
Exports: aggregate_by_law, build_summary_string, print
Semantic: pure computation

`src/cli/locality.rs`
Handler for locality scanning. [COUPLING:pure] [BEHAVIOR:propagates-errors] [QUALITY:error-boundary]
Exports: is_locality_blocking, check_locality_silent, run_locality_check, LocalityResult
Semantic: pure computation that propagates errors

`src/constants.rs`
Shared constants for file filtering and pattern matching. [COUPLING:mixed] [BEHAVIOR:owns-state] [QUALITY:undocumented]
Exports: BIN_EXT_PATTERN, CODE_BARE_PATTERN, CODE_EXT_PATTERN, SKIP_DIRS
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting stateful module

`src/corpus.rs`
model for corpus via file I/O. [TYPE] [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:persists,propagates-errors] [QUALITY:undocumented,error-boundary]
Exports: FileBaseline, CorpusRepo, load_manifest, run_manifest
Semantic: side-effecting adapter that propagates errors

`src/exit.rs`
Standardized process exit codes for `Neti`. [HOTSPOT] [GLOBAL-UTIL] [COUPLING:pure] [QUALITY:undocumented]
Exports: NetiExit, NetiExit.code, NetiExit.exit, NetiExit.from
Semantic: pure computation

`src/file_class.rs`
File classification: distinguishes source code from config, assets, and data. [HOTSPOT] [COUPLING:pure]
Exports: FileKind, FileKind.is_governed, FileKind.secrets_applicable, classify
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation

`src/graph/dead_code.rs`
Workspace-level dead code detection via zero-indegree reference analysis. [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:persists,panics-on-error,propagates-errors] [QUALITY:complex-flow,error-boundary]
Exports: DeadCodeReport, DeadCodeReport.build, DeadDefinition, DeadKind
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting adapter that panics on error

`src/graph/defs/extract.rs`
Implements def kind. [COUPLING:pure] [QUALITY:undocumented]
Exports: DefKind, Definition
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation

`src/graph/file_graph.rs`
FileGraph` — graph provider backed by static import analysis. [HOTSPOT] [COUPLING:pure] [BEHAVIOR:propagates-errors] [QUALITY:undocumented]
Exports: FileGraph.edge_count, FileGraph, FileGraph.build, FileGraph.edges
Semantic: pure computation that propagates errors

`src/graph/gravity.rs`
model for gravity via file I/O. [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:owns-state,persists] [QUALITY:undocumented]
Exports: GravityReport.is_accidental_hub, is_recognized_hub, compute_coupling_gravity, default_gravity_multiplier
Semantic: side-effecting stateful adapter

`src/graph/imports.rs`
Implements extract functionality. [HOTSPOT] [COUPLING:pure]
Exports: extract
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation

`src/graph/locality/analysis/metrics.rs`
Finds hub candidates. [COUPLING:pure] [QUALITY:undocumented]
Exports: compute_module_coupling, GodModuleInfo, find_god_modules, find_hub_candidates
Semantic: pure computation

`src/graph/locality/analysis/violations.rs`
Categories of locality violations. [HOTSPOT] [COUPLING:pure] [QUALITY:undocumented]
Exports: CategorizedViolation, ViolationKind, ViolationKind.description, ViolationKind.label
Semantic: pure computation

`src/graph/locality/classifier.rs`
Node classification based on coupling metrics. [HOTSPOT] [GLOBAL-UTIL] [COUPLING:pure]
Exports: ClassifierConfig, ClassifierConfig.default, classify
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation

`src/graph/locality/coupling.rs`
Afferent and Efferent coupling computation. [COUPLING:pure]
Exports: compute_coupling
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation

`src/graph/locality/cycles.rs`
Cycle detection for the Law of Locality. [COUPLING:pure]
Exports: detect_cycles_generic, detect_cycles
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation

`src/graph/locality/distance.rs`
Dependency Distance calculator via Lowest Common Ancestor (LCA). [COUPLING:pure]
Exports: compute_distance, find_lca
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation

`src/graph/locality/edges.rs`
Edge collection for locality analysis. [HOTSPOT] [GLOBAL-UTIL] [COUPLING:mixed] [BEHAVIOR:persists,propagates-errors]
Exports: normalize_path, collect
Semantic: side-effecting adapter that propagates errors

`src/graph/locality/exemptions.rs`
Smart structural exemptions for Rust module patterns. [COUPLING:pure] [BEHAVIOR:propagates-errors]
Exports: is_structural_pattern
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation that propagates errors

`src/graph/locality/layers.rs`
Layer inference for the Law of Locality. [HOTSPOT] [COUPLING:pure]
Exports: check_layer_violation, infer_layers
Semantic: pure computation

`src/graph/locality/report.rs`
Rich output formatting for locality analysis. [COUPLING:pure]
Exports: print_full_report
Semantic: pure computation

`src/graph/locality/types.rs`
Core types for the Law of Locality enforcement system. [TYPE] [COUPLING:pure] [QUALITY:undocumented]
Exports: LocalityEdge.routes_to_hub, NodeIdentity.allows_far_deps, LocalityEdge.is_local, PassReason
Semantic: pure computation

`src/graph/locality/validator.rs`
The Universal Locality Algorithm - Judgment Pass. [HOTSPOT] [GLOBAL-UTIL] [COUPLING:pure] [QUALITY:undocumented]
Exports: validate_from_provider, ValidationReport.check_cohesion, ValidationReport.is_clean, ValidationReport.total_edges
Semantic: pure computation

`src/graph/provider.rs`
GraphProvider` trait — pluggable graph topology for locality analysis. [COUPLING:pure]
Exports: GraphProvider, edge_count
Semantic: pure computation

`src/graph/resolver.rs`
Implements resolve functionality. [COUPLING:mixed] [BEHAVIOR:persists,propagates-errors] [QUALITY:error-boundary]
Exports: resolve
Semantic: side-effecting adapter that propagates errors

`src/project.rs`
Detects project type from current directory. [HOTSPOT] [COUPLING:pure] [QUALITY:undocumented]
Exports: ProjectType.detect_in, ProjectType.is_typescript, generate_toml, npx_cmd
Semantic: pure computation

`src/reporting.rs`
Console output formatting for scan results. [HOTSPOT] [GLOBAL-UTIL] [COUPLING:pure] [BEHAVIOR:propagates-errors]
Exports: build_rich_report, format_report_string, print_json, print_report
Semantic: pure computation that propagates errors

`src/reporting/console.rs`
Prints a formatted scan report to stdout with severity tiers and deduplication. [COUPLING:mixed] [BEHAVIOR:persists] [QUALITY:complex-flow]
Exports: print_report
Semantic: side-effecting adapter

`src/reporting/coverage.rs`
Per-language reporting for the retained Neti core checks. [COUPLING:pure]
Exports: append_coverage_report, print_coverage_scorecard, LangCoverageEntry, lang_name
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation

`src/reporting/guidance.rs`
Static educational guidance for retained rule codes. [COUPLING:pure]
Semantic: pure computation

`src/reporting/rich.rs`
utility for rich via file I/O. [COUPLING:mixed] [BEHAVIOR:persists,propagates-errors] [QUALITY:error-boundary]
Exports: build_rich_report, format_report_string
Semantic: side-effecting adapter that propagates errors

`src/reporting/shared.rs`
Implements shared functionality. [COUPLING:pure]
Semantic: pure computation

`src/reporting/summary.rs`
Punch-list report builder. [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:persists,panics-on-error,propagates-errors] [QUALITY:complex-flow,error-boundary]
Exports: build_console_punch_list, build_summary_report
Semantic: side-effecting adapter that panics on error

`src/spinner/client.rs`
Client for sending updates to the spinner. [COUPLING:pure] [QUALITY:undocumented]
Exports: SpinnerClient.set_macro_step, SpinnerClient.step_micro_progress, SpinnerClient.set_micro_status, SpinnerClient.push_log
Semantic: pure computation

`src/spinner/controller.rs`
Lifecycle controller for the spinner thread. [HOTSPOT] [GLOBAL-UTIL] [COUPLING:pure]
Exports: SpinnerController, SpinnerController.new, SpinnerController.stop
Semantic: pure computation

`src/spinner/handle.rs`
Thread management for the spinner. [COUPLING:pure]
Exports: SpinnerHandle, SpinnerHandle.spawn, SpinnerHandle.stop
Semantic: pure computation

`src/spinner/safe_hud.rs`
Thread-safe wrapper for HUD state. [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:sync-primitives] [QUALITY:undocumented]
Exports: SafeHud.completion_info, SafeHud, SafeHud.modify, SafeHud.new
Semantic: synchronized side-effecting

`src/spinner/state.rs`
HUD state management. [COUPLING:mixed] [BEHAVIOR:owns-state] [QUALITY:undocumented]
Exports: HudState.step_micro_progress, HudState.set_macro_step, HudState.set_micro_status, ATOMIC_LINES
Semantic: side-effecting stateful module

`src/tokens.rs`
Counts the approximate number of tokens in the given text. [HOTSPOT] [GLOBAL-UTIL] [COUPLING:pure]
Exports: Tokenizer.exceeds_limit, Tokenizer, Tokenizer.count
Semantic: pure computation

`src/types/command.rs`
Result of an external command execution. [TYPE] [HOTSPOT] [DOMAIN-CONTRACT] [COUPLING:pure]
Exports: CommandResult.error_count, CommandResult.warning_count, CommandResult, CommandResult.new
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation

`src/types/locality.rs`
Types for locality (Law of Locality) reporting. [TYPE]
Exports: LocalityReport, LocalityViolation

`src/verification/runner.rs`
Command execution and output capture. [COUPLING:mixed] [BEHAVIOR:persists,panics-on-error]
Exports: run_commands
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting adapter that panics on error

## Layer 2 -- Adapters / Infra

`src/discovery.rs`
utility for discovery via file I/O. [UTIL] [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:owns-state,persists,sync-primitives,panics-on-error] [QUALITY:error-boundary]
Exports: group_by_directory, discover
Touch: Contains inline Rust tests alongside runtime code.
Semantic: synchronized side-effecting stateful adapter that panics on error

`src/spinner/render.rs`
HUD rendering logic. [UTIL] [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:owns-state,propagates-errors] [QUALITY:undocumented]
Exports: run_hud_loop, str.blue, str.bold, str.cyan
Semantic: side-effecting stateful module that propagates errors

## Layer 3 -- App / Entrypoints

`src/analysis/mod.rs`
Core analysis logic exposed to the CLI. [ENTRY] [HOTSPOT] [GLOBAL-UTIL] [COUPLING:pure] [QUALITY:undocumented]
Exports: run_analysis_with_progress, AnalysisReport.into_scan_report, AnalysisReport.has_any_findings, AnalysisReport.has_blocking_errors
Semantic: pure computation

`src/analysis/patterns/mod.rs`
Retained semantic and paranoia checks. [ENTRY] [COUPLING:pure]
Exports: detect_all, semantic
Touch: Contains inline Rust tests alongside runtime code.
Semantic: pure computation

`src/bin/corpus-harness.rs`
Implements corpus-harness functionality. [COUPLING:pure]
Semantic: pure computation

`src/bin/neti.rs`
Implements neti functionality. [COUPLING:pure]
Semantic: pure computation

`src/cli/args.rs`
Implements Cli functionality. [COUPLING:mixed] [BEHAVIOR:panics-on-error] [QUALITY:error-boundary]
Exports: Cli, Commands
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting that panics on error

`src/cli/handlers/mod.rs`
Core analysis command handlers. [ENTRY] [HOTSPOT] [COUPLING:mixed] [BEHAVIOR:persists,panics-on-error,propagates-errors] [QUALITY:error-boundary]
Exports: get_repo_root, scan_report, handle_baseline, handle_check
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting adapter that panics on error

`src/cli/mod.rs`
CLI command handlers. [ENTRY] [HOTSPOT] [GLOBAL-UTIL]
Exports: args, dispatch, handlers, locality

`src/config/mod.rs`
Provides shared mod used across multiple domains. [ENTRY] [HOTSPOT] [GLOBAL-UTIL] [COUPLING:mixed] [BEHAVIOR:persists] [QUALITY:undocumented]
Exports: Config.process_ignore_line, save_to_file, Config.load_local_config, Config.parse_toml
Semantic: side-effecting adapter

`src/graph/defs/mod.rs`
Extracts symbol definitions from source files via omni_ast primitives. [ENTRY]

`src/graph/locality/analysis/mod.rs`
Deep topology analysis: categorize violations, find patterns, suggest fixes. [ENTRY] [HOTSPOT] [COUPLING:pure]
Exports: TopologyAnalysis, analyze, metrics, violations
Semantic: pure computation

`src/graph/locality/mod.rs`
Law of Locality enforcement for topological integrity. [ENTRY] [HOTSPOT] [GLOBAL-UTIL]
Exports: is_structural_pattern, normalize_path, collect_edges, compute_coupling
Touch: Contains inline Rust tests alongside runtime code.

`src/graph/mod.rs`
Re-exports the public API surface. [ENTRY] [HOTSPOT] [GLOBAL-UTIL]
Exports: DeadCodeReport, dead_code, FileGraph, GraphProvider

`src/lib.rs`
Re-exports the public API surface. [ENTRY]
Exports: file_class, omni_ast, analysis, baseline

`src/main.rs`
Placeholder file. [ENTRY]

`src/spinner/mod.rs`
Triptych HUD (Head-Up Display) for process execution feedback. [ENTRY] [COUPLING:pure]
Exports: safe_hud, SpinnerClient, SpinnerController, render
Semantic: pure computation

`src/types/mod.rs`
Severity tier for a violation — determines gate behavior. [TYPE] [HOTSPOT] [GLOBAL-UTIL] [COUPLING:mixed] [BEHAVIOR:panics-on-error] [QUALITY:undocumented]
Exports: ScanReport.has_blocking_errors, ScanReport.clean_file_count, CommandResult, FileReport.is_clean
Touch: Contains inline Rust tests alongside runtime code.
Semantic: side-effecting that panics on error

`src/verification/mod.rs`
External command verification pipeline. [ENTRY] [COUPLING:pure]
Exports: CommandResult, VerificationReport, VerificationReport.failed_count, VerificationReport.new
Semantic: pure computation

## Layer 4 -- Tests

`src/analysis/checks/syntax_test.rs`
Tests for crate. [COUPLING:pure]
Semantic: pure computation

`src/analysis/checks/test_scope.rs`
Utilities for detecting `#[cfg(test)]` module boundaries in Rust source. [COUPLING:pure]
Exports: is_in_test_block, cfg_test_ranges
Semantic: pure computation

`src/cli/handlers/inspect_tests.rs`
Tests for omni_ast. [COUPLING:mixed] [BEHAVIOR:persists,panics-on-error] [QUALITY:error-boundary]
Semantic: side-effecting adapter that panics on error

`src/graph/locality/tests.rs`
Integration tests for locality analysis — part 1. [COUPLING:pure]
Exports: MemGraph.edges, MemGraph.name
Semantic: pure computation

`src/graph/locality/tests/part2.rs`
Integration tests for locality analysis — part 2. [COUPLING:pure]
Semantic: pure computation

`tests/check_cross_language_test.rs`
Integration tests for the retained cross-language Neti surface. [COUPLING:mixed]
Semantic: side-effecting

`tests/check_json_test.rs`
Integration test: `neti check --json` must emit valid JSON to stdout. [COUPLING:mixed]
Semantic: side-effecting

`tests/check_locality_test.rs`
Integration test: locality integration in `neti check` pipeline. [COUPLING:mixed]
Semantic: side-effecting

`tests/command_parsing_test.rs`
Integration test: command parsing with shell-words. [COUPLING:mixed]
Semantic: side-effecting

`tests/corpus/README.md`
Project overview and usage guide.

`tests/corpus/badge_expectations.toml`
Support file for the corpus subsystem.

`tests/corpus/baselines-summary.md`
Support file for the corpus subsystem.

`tests/corpus/repos.toml`
Support file for the corpus subsystem.

`tests/corpus_harness_test.rs`
Tests for neti_core. [COUPLING:mixed]
Semantic: side-effecting


## DependencyGraph

```yaml
DependencyGraph:
  # --- Entrypoints ---
  corpus-harness.rs:
    Imports: [corpus.rs, exit.rs]
    ImportedBy: []
  lib.rs:
    Imports: [baseline.rs, cli/mod.rs, config/mod.rs, constants.rs, corpus.rs, discovery.rs, exit.rs, file_class.rs, graph/mod.rs, project.rs, reporting.rs, spinner/mod.rs, src/analysis/mod.rs, tokens.rs, types/mod.rs, verification/mod.rs]
    ImportedBy: []
  main.rs:
    Imports: []
    ImportedBy: []
  neti.rs:
    Imports: [cli/mod.rs, dispatch.rs, exit.rs]
    ImportedBy: []
  # --- High Fan-In Hotspots ---
  classifier.rs:
    Imports: [config/locality.rs, controller.rs, types/mod.rs]
    ImportedBy: [file_class.rs, locality/mod.rs, validator.rs, worker.rs]
  cli/mod.rs:
    Imports: [args.rs, cli/locality.rs, dispatch.rs, handlers/mod.rs]
    ImportedBy: [check_cross_language_test.rs, check_json_test.rs, check_locality_test.rs, command_parsing_test.rs, corpus.rs, corpus_harness_test.rs, lib.rs, neti.rs, runner.rs]
  command.rs:
    Imports: [controller.rs, tokens.rs]
    ImportedBy: [console.rs, rich.rs, summary.rs, types/mod.rs]
  config/locality.rs:
    Imports: [controller.rs, graph/mod.rs]
    ImportedBy: [classifier.rs, cli/locality.rs, config/mod.rs, config/types.rs, locality/analysis/mod.rs, part2.rs, safe_hud.rs, syntax_test.rs, tests.rs, types/mod.rs, validator.rs]
  config/mod.rs:
    Imports: [config/locality.rs, config/types.rs, constants.rs, controller.rs, io.rs, types/mod.rs]
    ImportedBy: [ast.rs, checks.rs, cli/locality.rs, discovery.rs, engine.rs, handlers/mod.rs, imports.rs, io.rs, lib.rs, safety.rs, src/analysis/mod.rs, syntax_test.rs, verification/mod.rs, worker.rs]
  controller.rs:
    Imports: [safe_hud.rs]
    ImportedBy: [ast.rs, banned.rs, baseline.rs, check_cross_language_test.rs, check_json_test.rs, check_locality_test.rs, check_report.rs, classifier.rs, cli/locality.rs, command.rs, command_parsing_test.rs, config/locality.rs, config/mod.rs, console.rs, corpus.rs, corpus_harness_test.rs, coupling.rs, coverage.rs, cycles.rs, dead.rs, dead_code.rs, dead_params.rs, discovery.rs, distance.rs, edges.rs, exemptions.rs, extract.rs, file_class.rs, gravity.rs, handle.rs, handlers/mod.rs, imports.rs, inspect.rs, inspect_tests.rs, io.rs, layers.rs, metrics.rs, paranoia_universal.rs, part2.rs, patterns/mod.rs, project.rs, report.rs, rich.rs, runner.rs, safe_hud.rs, safety.rs, scan_report.rs, semantic.rs, semantic_universal.rs, shared.rs, spinner/mod.rs, state.rs, summary.rs, suppress.rs, syntax.rs, syntax_test.rs, test_scope.rs, validator.rs, worker.rs]
  corpus.rs:
    Imports: [cli/mod.rs, controller.rs, edges.rs, exit.rs, tokens.rs]
    ImportedBy: [corpus-harness.rs, corpus_harness_test.rs, lib.rs]
  discovery.rs:
    Imports: [config/mod.rs, constants.rs, controller.rs, edges.rs, locality/mod.rs]
    ImportedBy: [cli/locality.rs, handlers/mod.rs, lib.rs]
  edges.rs:
    Imports: [controller.rs, graph/mod.rs, imports.rs, locality/mod.rs, resolver.rs]
    ImportedBy: [banned.rs, baseline.rs, check_cross_language_test.rs, check_report.rs, cli/locality.rs, console.rs, corpus.rs, coupling.rs, coverage.rs, cycles.rs, dead_code.rs, dead_params.rs, discovery.rs, distance.rs, engine.rs, exemptions.rs, extract.rs, gravity.rs, inspect.rs, io.rs, layers.rs, locality/mod.rs, metrics.rs, paranoia_universal.rs, patterns/mod.rs, report.rs, resolver.rs, rich.rs, scan_report.rs, semantic.rs, semantic_universal.rs, src/analysis/mod.rs, summary.rs, suppress.rs, syntax_rust.rs, syntax_ts.rs, test_scope.rs, tests.rs, tsconfig.rs, validator.rs, violations.rs]
  exit.rs:
    Imports: []
    ImportedBy: [baseline.rs, check_report.rs, cli/locality.rs, command_parsing_test.rs, corpus-harness.rs, corpus.rs, coverage.rs, cycles.rs, dispatch.rs, handlers/mod.rs, inspect.rs, inspect_tests.rs, lib.rs, neti.rs, part2.rs, runner.rs, tests.rs, tsconfig.rs]
  file_graph.rs:
    Imports: []
    ImportedBy: [cli/locality.rs, cycles.rs, graph/mod.rs, provider.rs, validator.rs]
  graph/mod.rs:
    Imports: [dead_code.rs, defs/mod.rs, file_graph.rs, gravity.rs, imports.rs, locality/mod.rs, provider.rs, resolver.rs, tsconfig.rs]
    ImportedBy: [cli/locality.rs, config/locality.rs, dead.rs, edges.rs, gravity.rs, lib.rs, locality/analysis/mod.rs, metrics.rs, resolver.rs, tests.rs, validator.rs, violations.rs]
  gravity.rs:
    Imports: [controller.rs, edges.rs, graph/mod.rs]
    ImportedBy: [graph/mod.rs, locality/analysis/mod.rs, metrics.rs, tests.rs, validator.rs, violations.rs]
  locality/mod.rs:
    Imports: [classifier.rs, coupling.rs, cycles.rs, distance.rs, edges.rs, exemptions.rs, layers.rs, locality/analysis/mod.rs, locality/types.rs, report.rs, tests.rs, validator.rs]
    ImportedBy: [cli/locality.rs, coupling.rs, discovery.rs, edges.rs, graph/mod.rs, summary.rs, tests.rs, validator.rs]
  project.rs:
    Imports: [controller.rs]
    ImportedBy: [handlers/mod.rs, io.rs, lib.rs]
  reporting.rs:
    Imports: [console.rs, coverage.rs, guidance.rs, rich.rs, shared.rs, summary.rs]
    ImportedBy: [check_report.rs, console.rs, dead.rs, handlers/mod.rs, inspect.rs, lib.rs, rich.rs, scan_report.rs]
  safe_hud.rs:
    Imports: [config/locality.rs, controller.rs]
    ImportedBy: [client.rs, controller.rs, render.rs, spinner/mod.rs]
  src/analysis/mod.rs:
    Imports: [ast.rs, checks.rs, config/mod.rs, edges.rs, engine.rs, patterns/mod.rs, safety.rs, suppress.rs, types/mod.rs, worker.rs]
    ImportedBy: [check_report.rs, handlers/mod.rs, layers.rs, lib.rs, report.rs]
  tokens.rs:
    Imports: []
    ImportedBy: [command.rs, corpus.rs, lib.rs, resolver.rs, scan_report.rs, summary.rs, syntax_rust.rs, test_scope.rs, types/mod.rs, verification/mod.rs, violations.rs, worker.rs]
  tsconfig.rs:
    Imports: [edges.rs, exit.rs]
    ImportedBy: [cli/locality.rs, graph/mod.rs, handlers/mod.rs, render.rs, resolver.rs, verification/mod.rs]
  types/mod.rs:
    Imports: [command.rs, config/locality.rs, tokens.rs, types/locality.rs, validator.rs]
    ImportedBy: [ast.rs, banned.rs, baseline.rs, check_report.rs, classifier.rs, cli/locality.rs, config/mod.rs, console.rs, coupling.rs, coverage.rs, dead_params.rs, engine.rs, handlers/mod.rs, io.rs, layers.rs, lib.rs, paranoia_universal.rs, patterns/mod.rs, rich.rs, runner.rs, safety.rs, scan_report.rs, semantic.rs, semantic_universal.rs, shared.rs, src/analysis/mod.rs, summary.rs, suppress.rs, syntax.rs, validator.rs, verification/mod.rs, worker.rs]
  validator.rs:
    Imports: [classifier.rs, config/locality.rs, controller.rs, cycles.rs, distance.rs, edges.rs, file_graph.rs, graph/mod.rs, gravity.rs, layers.rs, locality/mod.rs, types/mod.rs]
    ImportedBy: [cli/locality.rs, locality/mod.rs, report.rs, rich.rs, tests.rs, types/mod.rs]
  # --- Layer 0 -- Config ---
  Cargo.toml, README.md, SEMMAP.md, corpus/manifest.toml, corpus/results/run_1.md, neti.toml, refactor.md:
    Imports: []
    ImportedBy: []
  config/types.rs:
    Imports: [config/locality.rs]
    ImportedBy: [config/mod.rs, io.rs]
  io.rs:
    Imports: [config/mod.rs, config/types.rs, controller.rs, edges.rs, project.rs, semantic.rs, types/mod.rs]
    ImportedBy: [config/mod.rs]
  # --- Layer 1 -- Domain (Engine) ---
  ast.rs:
    Imports: [banned.rs, config/mod.rs, controller.rs, dead_params.rs, safety.rs, syntax.rs, types/mod.rs]
    ImportedBy: [src/analysis/mod.rs]
  banned.rs:
    Imports: [controller.rs, edges.rs, test_scope.rs, types/mod.rs]
    ImportedBy: [ast.rs, checks.rs]
  baseline.rs:
    Imports: [controller.rs, edges.rs, exit.rs, types/mod.rs]
    ImportedBy: [handlers/mod.rs, lib.rs]
  check_report.rs:
    Imports: [controller.rs, edges.rs, exit.rs, reporting.rs, rich.rs, src/analysis/mod.rs, summary.rs, types/mod.rs, verification/mod.rs]
    ImportedBy: [handlers/mod.rs]
  checks.rs:
    Imports: [banned.rs, config/mod.rs, dead_params.rs, syntax.rs, test_scope.rs]
    ImportedBy: [src/analysis/mod.rs]
  cli/locality.rs:
    Imports: [config/locality.rs, config/mod.rs, controller.rs, dead_code.rs, discovery.rs, edges.rs, exit.rs, file_graph.rs, graph/mod.rs, locality/analysis/mod.rs, locality/mod.rs, report.rs, tsconfig.rs, types/mod.rs, validator.rs, violations.rs]
    ImportedBy: [cli/mod.rs, handlers/mod.rs]
  client.rs:
    Imports: [safe_hud.rs]
    ImportedBy: [handlers/mod.rs, spinner/mod.rs]
  console.rs:
    Imports: [command.rs, controller.rs, edges.rs, reporting.rs, types/mod.rs]
    ImportedBy: [handlers/mod.rs, reporting.rs]
  constants.rs:
    Imports: []
    ImportedBy: [config/mod.rs, discovery.rs, lib.rs]
  coupling.rs:
    Imports: [controller.rs, edges.rs, locality/mod.rs, types/mod.rs]
    ImportedBy: [locality/mod.rs]
  coverage.rs:
    Imports: [controller.rs, edges.rs, exit.rs, types/mod.rs]
    ImportedBy: [reporting.rs]
  cycles.rs:
    Imports: [controller.rs, edges.rs, exit.rs, file_graph.rs]
    ImportedBy: [locality/mod.rs, validator.rs]
  dead.rs:
    Imports: [controller.rs, dead_code.rs, graph/mod.rs, reporting.rs]
    ImportedBy: [dispatch.rs, handlers/mod.rs]
  dead_code.rs:
    Imports: [controller.rs, edges.rs]
    ImportedBy: [cli/locality.rs, dead.rs, graph/mod.rs]
  dead_params.rs:
    Imports: [controller.rs, edges.rs, engine.rs, types/mod.rs]
    ImportedBy: [ast.rs, checks.rs]
  dispatch.rs:
    Imports: [dead.rs, exit.rs, handlers/mod.rs, inspect.rs]
    ImportedBy: [cli/mod.rs, neti.rs]
  distance.rs:
    Imports: [controller.rs, edges.rs]
    ImportedBy: [locality/mod.rs, validator.rs]
  engine.rs:
    Imports: [config/mod.rs, edges.rs, types/mod.rs, worker.rs]
    ImportedBy: [dead_params.rs, src/analysis/mod.rs]
  exemptions.rs:
    Imports: [controller.rs, edges.rs]
    ImportedBy: [locality/mod.rs]
  extract.rs:
    Imports: [controller.rs, edges.rs, imports.rs]
    ImportedBy: [defs/mod.rs]
  file_class.rs:
    Imports: [classifier.rs, controller.rs]
    ImportedBy: [lib.rs, worker.rs]
  guidance.rs:
    Imports: []
    ImportedBy: [reporting.rs]
  handle.rs:
    Imports: [controller.rs, render.rs]
    ImportedBy: [spinner/mod.rs]
  imports.rs:
    Imports: [config/mod.rs, controller.rs]
    ImportedBy: [edges.rs, extract.rs, graph/mod.rs]
  inspect.rs:
    Imports: [controller.rs, edges.rs, exit.rs, handlers/mod.rs, reporting.rs]
    ImportedBy: [dispatch.rs, handlers/mod.rs]
  layers.rs:
    Imports: [controller.rs, edges.rs, src/analysis/mod.rs, types/mod.rs]
    ImportedBy: [locality/mod.rs, tests.rs, validator.rs]
  locality/types.rs:
    Imports: []
    ImportedBy: [locality/mod.rs]
  metrics.rs:
    Imports: [controller.rs, edges.rs, graph/mod.rs, gravity.rs]
    ImportedBy: [locality/analysis/mod.rs]
  paranoia_universal.rs:
    Imports: [controller.rs, edges.rs, types/mod.rs]
    ImportedBy: [patterns/mod.rs]
  provider.rs:
    Imports: [file_graph.rs]
    ImportedBy: [graph/mod.rs]
  report.rs:
    Imports: [controller.rs, edges.rs, src/analysis/mod.rs, validator.rs]
    ImportedBy: [cli/locality.rs, locality/mod.rs]
  resolver.rs:
    Imports: [edges.rs, graph/mod.rs, tokens.rs, tsconfig.rs]
    ImportedBy: [edges.rs, graph/mod.rs]
  rich.rs:
    Imports: [command.rs, controller.rs, edges.rs, reporting.rs, types/mod.rs, validator.rs]
    ImportedBy: [check_report.rs, reporting.rs]
  runner.rs:
    Imports: [cli/mod.rs, controller.rs, exit.rs, types/mod.rs]
    ImportedBy: [verification/mod.rs]
  safety.rs:
    Imports: [config/mod.rs, controller.rs, types/mod.rs]
    ImportedBy: [ast.rs, src/analysis/mod.rs]
  scan_report.rs:
    Imports: [controller.rs, edges.rs, render.rs, reporting.rs, summary.rs, tokens.rs, types/mod.rs]
    ImportedBy: [handlers/mod.rs]
  semantic.rs:
    Imports: [controller.rs, edges.rs, semantic_universal.rs, types/mod.rs]
    ImportedBy: [io.rs, patterns/mod.rs]
  semantic_universal.rs:
    Imports: [controller.rs, edges.rs, types/mod.rs]
    ImportedBy: [semantic.rs]
  shared.rs:
    Imports: [controller.rs, types/mod.rs, violations.rs]
    ImportedBy: [reporting.rs]
  state.rs:
    Imports: [controller.rs]
    ImportedBy: [spinner/mod.rs]
  summary.json:
    Imports: []
    ImportedBy: []
  summary.rs:
    Imports: [command.rs, controller.rs, edges.rs, locality/mod.rs, tokens.rs, types/mod.rs]
    ImportedBy: [check_report.rs, reporting.rs, scan_report.rs]
  suppress.rs:
    Imports: [controller.rs, edges.rs, types/mod.rs]
    ImportedBy: [src/analysis/mod.rs, worker.rs]
  syntax.rs:
    Imports: [controller.rs, types/mod.rs]
    ImportedBy: [ast.rs, checks.rs, syntax_test.rs]
  syntax_rust.rs:
    Imports: [edges.rs, tokens.rs]
    ImportedBy: []
  syntax_ts.rs:
    Imports: [edges.rs]
    ImportedBy: []
  types/locality.rs:
    Imports: []
    ImportedBy: [types/mod.rs]
  violations.rs:
    Imports: [edges.rs, graph/mod.rs, gravity.rs, tokens.rs]
    ImportedBy: [cli/locality.rs, locality/analysis/mod.rs, shared.rs]
  worker.rs:
    Imports: [classifier.rs, config/mod.rs, controller.rs, file_class.rs, locality/analysis/mod.rs, patterns/mod.rs, suppress.rs, tokens.rs, types/mod.rs]
    ImportedBy: [engine.rs, src/analysis/mod.rs]
  # --- Layer 2 -- Adapters / Infra ---
  render.rs:
    Imports: [safe_hud.rs, tsconfig.rs]
    ImportedBy: [handle.rs, scan_report.rs, spinner/mod.rs]
  # --- Layer 3 -- App / Entrypoints ---
  args.rs:
    Imports: []
    ImportedBy: [cli/mod.rs]
  defs/mod.rs:
    Imports: [extract.rs]
    ImportedBy: [graph/mod.rs]
  handlers/mod.rs:
    Imports: [baseline.rs, check_report.rs, cli/locality.rs, client.rs, config/mod.rs, console.rs, controller.rs, dead.rs, discovery.rs, exit.rs, inspect.rs, project.rs, reporting.rs, scan_report.rs, spinner/mod.rs, src/analysis/mod.rs, tsconfig.rs, types/mod.rs, verification/mod.rs]
    ImportedBy: [cli/mod.rs, dispatch.rs, inspect.rs]
  locality/analysis/mod.rs:
    Imports: [config/locality.rs, graph/mod.rs, gravity.rs, metrics.rs, violations.rs]
    ImportedBy: [cli/locality.rs, locality/mod.rs, worker.rs]
  patterns/mod.rs:
    Imports: [controller.rs, edges.rs, paranoia_universal.rs, semantic.rs, types/mod.rs]
    ImportedBy: [src/analysis/mod.rs, worker.rs]
  spinner/mod.rs:
    Imports: [client.rs, controller.rs, handle.rs, render.rs, safe_hud.rs, state.rs]
    ImportedBy: [handlers/mod.rs, lib.rs]
  verification/mod.rs:
    Imports: [config/mod.rs, runner.rs, tokens.rs, tsconfig.rs, types/mod.rs]
    ImportedBy: [check_report.rs, handlers/mod.rs, lib.rs]
  # --- Tests ---
  badge_expectations.toml, repos.toml, tests/corpus/README.md, tests/corpus/baselines-summary.md:
    Imports: []
    ImportedBy: []
  check_cross_language_test.rs:
    Imports: [cli/mod.rs, controller.rs, edges.rs]
    ImportedBy: []
  check_json_test.rs, check_locality_test.rs:
    Imports: [cli/mod.rs, controller.rs]
    ImportedBy: []
  command_parsing_test.rs:
    Imports: [cli/mod.rs, controller.rs, exit.rs]
    ImportedBy: []
  corpus_harness_test.rs:
    Imports: [cli/mod.rs, controller.rs, corpus.rs]
    ImportedBy: []
  inspect_tests.rs:
    Imports: [controller.rs, exit.rs]
    ImportedBy: []
  part2.rs:
    Imports: [config/locality.rs, controller.rs, exit.rs]
    ImportedBy: [tests.rs]
  syntax_test.rs:
    Imports: [config/locality.rs, config/mod.rs, controller.rs, syntax.rs]
    ImportedBy: []
  test_scope.rs:
    Imports: [controller.rs, edges.rs, tokens.rs]
    ImportedBy: [banned.rs, checks.rs]
  tests.rs:
    Imports: [config/locality.rs, edges.rs, exit.rs, graph/mod.rs, gravity.rs, layers.rs, locality/mod.rs, part2.rs, validator.rs]
    ImportedBy: [locality/mod.rs]
```
