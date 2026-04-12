# ROY v0.2 Backlog Seed — Tracker Issues

AGENT - PLEASE ADAPT THESE INTO THE CANONICAL STORE VIA ISHOO CLI


The backlog is biased toward turning the current ROY implementation into the language surface described in your draft spec. It mixes foundation work, product-surface work, and the glue needed so AI agents can execute the issues without guessing.


---

## [LANG-01] Implement top-level ROY parser for verb noun filter pipe grammar

**Status:** TODO

**Files:** `src/ui/layout/panels/command_line.rs`, `src/shell/resolve.rs`, `src/commands/mod.rs`, `src/commands/schema.rs`, `src/commands/ast.rs`

**Labels:** v0-2, language, parser, grammar, core



**Description:** Replace the current shell-like token splitter with a real ROY command parser that understands the v0.2 surface shape: `<verb> <noun> [filters...] [| <refiner>...]`. The parser should produce a typed AST rather than an argv-style list, because ROY’s semantics depend on distinguishing verbs, nouns, singular-vs-plural targets, filters, spans, and pipeline stages before dispatch. Preserve the current lightweight feel in the UI, but move the real logic into a reusable parser module that both the shell runtime and tests can call.

Handle quoted tokens, spans like `lines 1..40`, `key:value` filters, negation (`!role:test`), ranges, and pipelines. The output should make room for later additions such as `--dry`, `--json`, and `--ref` without smuggling generic shell semantics back in. Error messages must be specific and corrective: point at the failing token, explain what the parser expected, and suggest a valid next form. Add golden tests for every example in the language spec and ensure malformed inputs fail deterministically with stable messages.


---

## [LANG-02] Introduce typed command AST and dispatch plan layer

**Status:** TODO

**Files:** `src/commands/schema.rs`, `src/commands/mod.rs`, `src/shell/runtime.rs`, `src/shell/result.rs`, `src/commands/ast.rs`, `src/commands/plan.rs`

**Labels:** v0-2, language, ast, dispatch, core



**Description:** Add a first-class AST and planning layer between parsing and execution. The current runtime dispatches by string command name; that is insufficient for ROY’s future because verbs, nouns, filters, projections, and refiners all carry distinct types and cost implications. Define AST types for `Verb`, `Noun`, `Target`, `FilterExpr`, `RefinerExpr`, `Pipeline`, and `RenderMode`, then add a planner that validates a parsed command against the registry and resolves it into an executable plan.

This plan layer should answer questions the runtime currently cannot: what noun schema applies, whether the noun is singular or plural, what result type each pipeline stage produces, which filters are source-pushdown vs post-materialization refiners, and whether the command is read-only or mutating. Keep the planner pure and side-effect-free so it becomes the foundation for `--dry`, help suggestions, and policy preflight. Add tests for valid and invalid plans, especially type errors like applying string refiners to a `FileSet`.


---

## [LANG-03] Add singular vs plural noun semantics to command validation

**Status:** TODO

**Files:** `src/commands/schema.rs`, `src/commands/registry.rs`, `src/commands/plan.rs`, `src/nouns/mod.rs`, `src/ui/layout/panels/terminal_model.rs`

**Labels:** v0-2, language, nouns, validation, semantics



**Description:** Implement the v0.2 rule that singular and plural nouns are different surfaces, not aliases. `file` should mean a point lookup returning a `File`; `files` should mean discovery returning a `FileSet`. The same pattern must apply to `symbol/symbols`, `module/modules`, `schema/schemas`, and other paired nouns. This distinction belongs in validation, not in ad hoc runtime branching, so the planner can reject invalid combinations early.

The critical behavior is user-facing correctness. Commands like `find file ...` should fail with a helpful message explaining that `file` takes an identifier and `files` takes filters. Likewise, list refiners such as `top`, `sort`, and `where` must reject singular objects unless they are explicitly projected to list-like values. Update help surfaces and examples so the agent sees the distinction immediately. Add exhaustive tests for parse-time and plan-time errors around singular/plural misuse, because this is one of the main guardrails keeping ROY from drifting back toward loose shell behavior.


---

## [LANG-04] Implement context-sensitive `?` help for partial commands and pipeline stages

**Status:** TODO

**Files:** `src/ui/layout/panels/command_line.rs`, `src/commands/plan.rs`, `src/commands/help.rs`, `src/ui/layout/panels/terminal.rs`

**Labels:** v0-2, language, help, discoverability, ux



**Description:** Support the v0.2 discovery affordance where appending `?` to an incomplete command shows what is valid next without executing anything. This must work at multiple positions: after a verb (`find ?`), after a noun (`find files ?`), after a pipe (`find files | ?`), and after a partially supplied target or filter. The result should be derived from the same registry, noun schema, and refiner typing rules used by the planner, so help cannot drift out of sync with the executable surface.

Do not bolt this onto output rendering as a special case string. Treat help as a structured response with variants such as `VerbHelp`, `NounHelp`, `FilterHelp`, and `RefinerHelp`, then render those nicely in the terminal. The implementation should be explicit about the current parse context and return only suggestions that are legal in that position. Add tests for all documented help flows, plus negative tests showing that `?` does not execute a partial command or mutate session state.


---

## [LANG-05] Add `--dry` planning mode with cost class and result estimate

**Status:** TODO

**Files:** `src/commands/plan.rs`, `src/commands/schema.rs`, `src/nouns/mod.rs`, `src/shell/runtime.rs`, `src/shell/result.rs`

**Labels:** v0-2, language, dry-run, cost, planning



**Description:** Implement the `--dry` flag exactly as described in the v0.2 spec: it should return the execution plan, cost class, and a rough estimate of the result set without performing the substantive query or mutation. The dry planner must use the same AST and schema system as real execution, otherwise it will become a misleading separate surface. For mutating commands, `--dry` should show the change target, policy class, approval expectation, and validation consequences without touching disk.

Define a small `CostClass` model (`instant`, `quick`, `slow`, `expensive`) and make it visible both in command schemas and in planner output. Estimation can be approximate, but it must be grounded: file counts from directory walks, symbol counts from indices, validation cost from known workspace shape, and so on. Add tests for representative noun families and ensure `--dry` never mutates session state, writes artifacts, or increments issue/session history beyond an optional read-only planning event.


---

## [LANG-06] Implement `--json` and `--ref` render modes with stable schema envelope

**Status:** TODO

**Files:** `src/commands/schema.rs`, `src/shell/result.rs`, `src/render/mod.rs`, `src/render/json.rs`, `src/render/ref.rs`, `src/ui/layout/panels/terminal_model.rs`

**Labels:** v0-2, rendering, json, refs, schemas



**Description:** Add the two machine-oriented render modes from the spec: `--json` for structured output and `--ref` for reference-only output. `--json` must emit a stable envelope containing `_schema` metadata with noun, projection, and version, plus the result payload. `--ref` must return a session-scoped reference id that points at the full typed result for downstream commands. Both modes need to be implemented at the plan/result layer, not as post-hoc string munging in the UI.

The hard part is preserving type information across the shell boundary. Result values should carry schema identity and renderers should switch on the result type rather than re-inferring structure from terminal text. This issue should also introduce versioned result schemas and ensure default human rendering remains distinct from the machine contract. Add end-to-end tests confirming that the same command can render in human, JSON, and ref modes without changing the underlying result semantics.


---

## [LANG-07] Add session-scoped named refs and `last`/`save as`/`show ref` operations

**Status:** TODO

**Files:** `src/session/engine.rs`, `src/session/events.rs`, `src/storage/sqlite.rs`, `src/commands/registry.rs`, `src/shell/runtime.rs`, `src/refs/mod.rs`

**Labels:** v0-2, refs, session, storage, language



**Description:** Implement named references as the typed replacement for shell variables. The session should always retain a `last` value representing the most recent command result, and users should be able to `save as <name>`, `list refs`, `show ref <name>`, and `drop ref <name>`. Refs must preserve the underlying result type, schema version, and summary so later commands can consume them safely without flattening to strings.

Treat refs as first-class session state rather than UI-only convenience. They should survive across terminal renders, appear in session review, and be persisted if the session itself is persisted. Make the planner aware of ref types so commands like `read file topfive.first` are validated correctly. Add guardrails against name collisions, stale refs to deleted artifacts, and attempts to use list-only selectors on singular refs. Tests should cover both parse-level syntax and replay/storage behavior.


---

## [LANG-08] Build internal schema registry and expose `schema`/`schemas` nouns

**Status:** TODO

**Files:** `src/commands/schema.rs`, `src/schema_registry/mod.rs`, `src/nouns/schema.rs`, `src/render/json.rs`, `src/commands/help.rs`

**Labels:** v0-2, schemas, introspection, language, meta



**Description:** Create a machine-readable schema registry describing every noun, projection, and result envelope that ROY exposes. This is the substrate for the new `schema` and `schemas` nouns and also the thing that keeps `--json` stable over time. Each schema entry should include the noun/projection name, version, field definitions, examples, compatibility notes, and the singular/plural behavior where applicable.

Expose two user-facing paths: `show schemas` for summary listing and `read schema <name>` for the full contract. Keep the registry close to the executable type definitions so it does not become stale documentation. This issue should also define how version bumps are represented and how minor vs major compatibility changes are declared. Add tests asserting that every noun and projection reachable through the planner has a schema entry and that missing schema coverage fails loudly in CI.


---

## [LANG-09] Reserve `task` and `tasks` nouns and emit explicit future-surface denial

**Status:** TODO

**Files:** `src/commands/registry.rs`, `src/commands/registry_data.rs`, `src/policy/engine.rs`, `src/session/artifacts.rs`, `src/shell/runtime.rs`

**Labels:** v0-2, language, reserved, denials, future-proofing



**Description:** Implement the spec rule that `task` and `tasks` are reserved future nouns and are not valid in v0.2. This should not look like a generic not-found error. Add them to the language surface as reserved entries that parse and validate far enough to produce a precise denial message: the nouns are intentionally held for future long-running/background work and are unavailable in this version.

Because denials are first-class in ROY, invoking a reserved noun should create a normal denial artifact with a stable rule id and reviewable explanation. That way an agent learns that the surface exists conceptually but is not yet shipped, rather than concluding the command was misspelled. Add tests for direct invocation, help behavior, and JSON/ref rendering of the denial.


---

## [LANG-10] Standardize parser and planner diagnostics with suggestions and stable error codes

**Status:** TODO

**Files:** `src/commands/diagnostics.rs`, `src/commands/plan.rs`, `src/ui/layout/panels/terminal_model.rs`, `src/shell/result.rs`

**Labels:** v0-2, language, diagnostics, errors, ux



**Description:** Add a unified diagnostics model for parse errors, planning/type errors, and invalid verb–noun combinations. Errors should carry a short stable code, a human-readable message, the token span or stage that failed, and one or more suggestions that move the actor toward a valid ROY command. This is especially important for AI operation because vague shell-like errors cause bad retries.

Make diagnostics structured internally and keep the terminal renderer responsible only for presentation. Unknown filters should suggest valid keys; invalid refiners should suggest compatible projections; singular/plural mistakes should explain the distinction rather than merely rejecting the command. Build regression tests around exact messages for the most important cases so future refactors do not degrade the learning surface.


---

## [FIND-01] Implement semantic `about <topic>` retrieval for `find files`

**Status:** TODO

**Files:** `src/nouns/files.rs`, `src/search/about.rs`, `src/workspace/mod.rs`, `src/render/json.rs`, `src/commands/help.rs`

**Labels:** v0-2, nouns, files, retrieval, semantic-search



**Description:** Ship the core `find files about <topic>` capability described in the spec. The implementation should rank files by semantic relevance rather than raw lexical match, with clear precedence among signal sources: semantic descriptions and summaries first, documentation next, identifiers after that, and lexical content as a weaker boost. Even if the initial implementation uses a simplified signal stack, the code structure must preserve this ordering so the behavior can improve without changing the command contract.

The result type should expose hidden relevance scores in JSON mode and concise human summaries in default mode. Empty results must be legitimate, not treated as an implementation failure. Also provide explainability: there should be enough metadata on each match for future `explain file` to say why a file surfaced. Add tests for ranking stability, empty-result behavior, and lexical-only false-positive resistance.


---

## [FIND-02] Add full `files` noun filter surface and source-pushdown planning

**Status:** TODO

**Files:** `src/nouns/files.rs`, `src/commands/plan.rs`, `src/schema_registry/mod.rs`, `src/workspace/boundary.rs`

**Labels:** v0-2, nouns, files, filters, planning



**Description:** Implement the `files` noun as a rich discovery surface with the filters promised in the spec: `lang`, `path`, `role`, `surface`, `quality`, `coupling`, `behavior`, `in`, `lines`, `fan_in`, `fan_out`, `changed`, and `staged`. The important design constraint is that filters remain typed and planner-visible so ROY can distinguish source-pushdown constraints from expensive post-materialization work. That distinction feeds both performance and truthful `--dry` output.

Start with the filters that can be grounded directly from the current workspace and semantic metadata, then stub the rest explicitly if a backing index is still missing. Do not silently treat unsupported filters as no-ops. The command should either execute correctly or return an honest “filter not yet implemented” error with a stable diagnostic code. Add schema entries, help docs, and tests for every supported filter and for combinations that should be rejected.


---

## [SYM-01] Create `symbols` noun backed by semantic index and omni-ast metadata

**Status:** TODO

**Files:** `src/nouns/symbols.rs`, `src/nouns/mod.rs`, `src/schema_registry/mod.rs`, `src/trace/mod.rs`

**Labels:** v0-2, nouns, symbols, semantic-map, ast



**Description:** Add the `symbols` noun as a first-class discovery surface for functions, methods, types, constants, statics, and fields. The result type should include qualified name, kind, file/span, visibility, and enough semantic attributes to support later refinement (`fan_in`, `fan_out`, `complexity`, `role`, `behavior`). This should not be a thin grep wrapper over source text; it needs to be backed by the best semantic index ROY has available.

Design the module so unsupported languages degrade honestly instead of pretending to be exact. For languages with partial support, return the fields you can guarantee and surface the limitations in help/schema output. Add discovery filters from the spec (`name`, `kind`, `lang`, `in`, `visibility`, etc.) and ensure the type integrates with `read symbol`, `trace callers/callees`, and JSON rendering. Tests should cover qualified-name disambiguation and stable ordering for repeated queries.


---

## [SYM-02] Implement `read symbol` with signature, docs, span, and body extraction

**Status:** TODO

**Files:** `src/nouns/symbols.rs`, `src/read/mod.rs`, `src/render/human.rs`, `src/render/json.rs`

**Labels:** v0-2, read, symbols, content, ux



**Description:** Build the singular symbol read path so `read symbol <target>` is the primary alternative to dumping whole files. The result should include the symbol’s signature, documentation if present, file path and span, and the body text scoped to just that symbol. This capability is central to the ROY promise that an agent should not need Bash to orient inside a codebase.

Handle ambiguity explicitly. If a short name resolves to multiple symbols, return a disambiguation result rather than picking arbitrarily. If the symbol cannot be read exactly due to language support limits, say so and degrade to the best available span or summary. Add targeted tests for methods with the same name in different impls/modules, doc-comment preservation, and JSON envelopes for symbol readouts.


---

## [MOD-01] Implement `module` and `modules` nouns with structural summaries

**Status:** TODO

**Files:** `src/nouns/modules.rs`, `src/nouns/mod.rs`, `src/schema_registry/mod.rs`, `src/render/human.rs`

**Labels:** v0-2, nouns, modules, structure, semmap



**Description:** Add module-level objects representing coherent directories or packages rather than raw filesystem paths. `show module <path>` should return a structural summary—entry points, notable files, role/surface badges, and a concise description—while `find modules` should support discovery by name, path, layer, and containment. The goal is to make module navigation feel like looking at a semantic map rather than running `ls`.

Keep the module boundary logic explicit and testable. Start with the best practical heuristic for the current codebase (directory/package boundaries plus semantic metadata) and make it possible to swap in stronger SEMMAP-backed grouping later without changing the command interface. Add tests covering root module behavior, nested module summaries, and the singular/plural distinction between `module` and `modules`.


---

## [HOT-01] Implement `hotspots` noun using dependency centrality and semantic badges

**Status:** TODO

**Files:** `src/nouns/hotspots.rs`, `src/trace/graph.rs`, `src/schema_registry/mod.rs`, `src/render/human.rs`

**Labels:** v0-2, nouns, hotspots, graph, navigation



**Description:** Create the `hotspots` noun so ROY can quickly surface high-centrality files and symbols. The implementation should combine graph-derived measures such as fan-in/fan-out with any semantic hotspot markers already available in the code intelligence layer. Output should be useful immediately for onboarding: name, kind, path, centrality metrics, and a short explanation of why the item is considered hot.

Support both discovery (`find hotspots`) and structural summary (`show hotspots`) and ensure the noun composes with list refiners like `top`, `sort by fan_in`, and `count by kind`. Do not bury the metric definitions in code comments—surface them in help/schema output so agents can reason about what a hotspot means. Add tests for ranking order, filter behavior, and deterministic output in small fixture graphs.


---

## [TAX-01] Add `roles` noun and role-badge membership queries

**Status:** TODO

**Files:** `src/nouns/roles.rs`, `src/nouns/files.rs`, `src/nouns/modules.rs`, `src/schema_registry/mod.rs`

**Labels:** v0-2, nouns, taxonomy, roles, semantic-map



**Description:** Expose semantic roles such as `controller`, `model`, `view`, `config`, `utility`, and `bootstrap` as a proper noun surface. The noun should support two complementary workflows: discovering the role vocabulary itself (`show roles`) and discovering which files/modules/symbols belong to a role (`find files role:controller`, `show role controller`). This makes semantic categorization an inspectable part of the language rather than a hidden internal badge system.

The implementation should define a single role source of truth and let other nouns reference it, rather than copying string lists across modules. If role inference is incomplete, surface that fact explicitly in help text and schemas. Add tests for membership queries, role summaries, and consistent rendering between human and JSON modes.


---

## [TAX-02] Add `surfaces` noun and external-coupling membership queries

**Status:** TODO

**Files:** `src/nouns/surfaces.rs`, `src/nouns/files.rs`, `src/nouns/modules.rs`, `src/schema_registry/mod.rs`

**Labels:** v0-2, nouns, taxonomy, surfaces, integration



**Description:** Implement the `surfaces` noun for external-coupling categories such as `filesystem`, `database`, `http-handler`, `shell`, and similar badges. This should work like the `roles` noun but answer a different question: what interfaces with the outside world, and where? The point is to let an agent ask “show me everything touching the filesystem” as a semantic query instead of a text search.

Make surface definitions centralized and reusable by file, symbol, and module summaries. Provide projections for names, definitions, and members, plus filters on other nouns using `surface:<name>`. Add tests confirming that surfaces are discoverable, attach cleanly to other noun results, and degrade honestly if the underlying classifier cannot yet populate them for a language or file type.


---

## [DEP-01] Implement `deps` noun and dependency relation queries

**Status:** TODO

**Files:** `src/nouns/deps.rs`, `src/trace/graph.rs`, `src/schema_registry/mod.rs`, `src/render/json.rs`

**Labels:** v0-2, nouns, deps, graph, tracing



**Description:** Add the `deps` noun as a relation set rather than a point object. It should support queries like `show deps from:src/shell/runtime.rs depth:1`, `trace dependents of <target>`, and later graph projections. The result model needs to capture relation kind, direction, source, target, and optionally depth/edge path metadata for multi-hop traces.

Keep the dependency engine generic enough to handle multiple relation kinds (`import`, `call`, `field-access`) even if the initial release only has one or two with strong support. This issue should define the typed relation record, the planner validation for `from`/`to`/`direction` filters, and a stable JSON representation that can later back graph artifacts. Add tests around directionality, depth handling, and flattening/tree view compatibility.


---

## [DIFF-01] Implement `diff` and `diffs` nouns for staged, unstaged, and workspace change sets

**Status:** TODO

**Files:** `src/nouns/diffs.rs`, `src/session/artifacts.rs`, `src/render/human.rs`, `src/render/json.rs`

**Labels:** v0-2, nouns, diffs, git, review



**Description:** Create a first-class diff surface that unifies Git-backed diffs with ROY-generated change artifacts. `show diff staged`, `read diff staged`, and `find diffs scope:workspace` should all feel like native ROY commands rather than shell pass-through. The noun should expose summary, files, hunks, and text projections as specified, and it should integrate tightly with the mutation/review pipeline.

Be explicit about source provenance. A diff may come from Git state, a staged ROY patch artifact, or a recorded past mutation, and that distinction should be present in metadata. Also define what happens when no Git repository exists: return a clear capability limitation rather than a confusing shell error. Add tests for staged/unstaged selection, artifact-backed diff reads, and JSON envelope consistency.


---

## [ART-01] Expose `artifacts` noun with read/show/review support across artifact kinds

**Status:** TODO

**Files:** `src/nouns/artifacts.rs`, `src/session/artifacts.rs`, `src/storage/sqlite.rs`, `src/render/human.rs`

**Labels:** v0-2, nouns, artifacts, session, review



**Description:** Turn the existing artifact machinery into a full noun surface. Users should be able to `find artifacts kind:validation`, `show artifact <id>`, `read artifact <id>`, and `review artifact <id>` without special-case UI wiring. The implementation should index artifact metadata, preserve typed bodies, and provide stable summaries even when the detailed renderer differs by artifact kind.

Unify the current in-memory artifact handling with storage-backed lookup so historical artifacts remain inspectable after session replay or restart. Keep artifact kind renderers modular—diffs, validation runs, denials, notes, and future graph artifacts should each have a renderer and JSON serializer. Add tests covering artifact lookup by id, filtering by kind/since/actor, and replay from persisted store.


---

## [DEN-01] Implement `denial` and `denials` nouns with structured review output

**Status:** TODO

**Files:** `src/nouns/denials.rs`, `src/policy/engine.rs`, `src/session/artifacts.rs`, `src/storage/sqlite.rs`

**Labels:** v0-2, nouns, denials, policy, review



**Description:** Expose policy and compatibility denials as a dedicated noun family rather than burying them in session text. A denial should carry the full structure from the language spec: original command, reason, redirect, policy rule, and agent hint. `review denial last` and `find denials since:1h` should both operate on this typed structure, with human output optimized for recovery and JSON output preserving every field.

This issue should also unify all denial sources—compat traps, reserved-future nouns, unsupported operations, policy rejections—under one common model. The session ledger and storage layer should record denials in a way that makes them queryable by command, reason, actor, and time. Add tests for denial creation, persistence, review rendering, and redirect integrity.


---

## [SES-01] Add `session` and `sessions` noun surfaces over the event ledger

**Status:** TODO

**Files:** `src/nouns/sessions.rs`, `src/session/engine.rs`, `src/session/events.rs`, `src/storage/sqlite.rs`

**Labels:** v0-2, nouns, sessions, storage, review



**Description:** Implement the session noun so ROY can introspect its own history with the same semantic shape it uses for workspace objects. `show session` should summarize the current session—workspace, start time, active refs, artifact counts, denials, last result type, and other useful context—while `review session since:1h` should expose historical events in a structured, filterable way. `sessions` should enable multi-session history once persisted storage is in place.

Avoid making the noun a lossy wrapper over freeform transcript lines. It should be built from the typed event ledger and stay stable even as UI renderings change. Add filtering by id, actor, and time, plus projections like `events`, `artifacts`, and `denials`. Tests should cover current-session summaries, replay from storage, and bounded review windows.


---

## [ISSUE-01] Build local issue store and `issue`/`issues` noun surfaces

**Status:** TODO

**Files:** `src/issues/mod.rs`, `src/nouns/issues.rs`, `src/storage/sqlite.rs`, `migrations/002_issues.sql`

**Labels:** v0-2, issues, tracker, storage, nouns



**Description:** Create a local issue model and storage layer so ROY can manage work items even when no external tracker is configured. The noun surface should match the spec: filters by state/id/tag/assignee/since, projections for ids/titles/bodies/summary, and singular mutation targets for `change issue`. Keep the issue schema intentionally small but complete enough to back your tracker population workflow immediately.

This issue should include a migration, data model, CRUD APIs, and session integration so issue events can be reviewed historically. IDs should be human-readable and stable, and issue text should be stored as structured fields rather than one markdown blob when possible. Add tests for create/read/update/query behavior and for linking issues to artifacts or validation runs later.


---

## [ISSUE-02] Add external tracker adapter boundary so ROY can proxy issue operations

**Status:** TODO

**Files:** `src/issues/mod.rs`, `src/issues/adapter.rs`, `src/nouns/issues.rs`, `src/commands/help.rs`

**Labels:** v0-2, issues, tracker, adapters, integration



**Description:** Define the adapter boundary for external issue trackers so the language surface stays stable whether ROY is backed by the local store or a tool like Ishoo later. The adapter should cover listing, lookup, status updates, tagging, assignment, and note append operations. The user-facing command language must not fork by backend; the adapter translates ROY issue operations into the underlying system.

Build this boundary now even if the first concrete backend is the local store. Doing so prevents issue commands from hard-coding local assumptions and makes later external integration a contained piece of work. Also surface backend identity in `show issues`/`show issue` metadata so an actor can tell whether an issue is local or proxied. Add interface tests using a fake adapter plus compatibility tests against the local implementation.


---

## [REFN-01] Create typed refiner engine and pipeline result typing

**Status:** TODO

**Files:** `src/refiners/mod.rs`, `src/refiners/types.rs`, `src/commands/plan.rs`, `src/shell/runtime.rs`

**Labels:** v0-2, refiners, pipeline, typing, core



**Description:** Implement the generic refiner engine that consumes typed results and produces new typed results along a pipeline. This engine is one of the central ROY differentiators: `|` is structural, not a byte stream. Define a result-type system with enough fidelity to distinguish singular objects, sets, grouped lists, key-count lists, string lists, and readouts. The planner should validate the whole pipeline using these types before execution begins.

Keep refiners as explicit operations, not embedded closures or generic callbacks. Each refiner should declare what input types it accepts and what output type it returns so error messages and help suggestions remain precise. Add tests for legal and illegal refiner chains, especially around projection to strings and the boundary between structural vs string-level refinement.


---

## [REFN-02] Implement list slicing refiners: `top`, `last`, `first`, `nth`, `skip`, `range`

**Status:** TODO

**Files:** `src/refiners/slicing.rs`, `src/refiners/mod.rs`, `src/commands/help.rs`

**Labels:** v0-2, refiners, slicing, pipeline



**Description:** Add the list-slicing family of refiners exactly as specified: `top N`, `last N`, `nth N`, `first`, `skip N`, and `range A..B`. These should operate on list-like result types only and preserve item types while narrowing cardinality. Negative or out-of-range indexing should fail predictably with good diagnostics, not panic or silently clamp.

Take care with singularization. `first` and `nth` transform a list into one item, which changes which later refiners are legal. That type transition needs to be reflected in both planner validation and runtime result metadata. Add targeted tests for boundary cases, empty lists, and human/JSON rendering of sliced outputs.


---

## [REFN-03] Implement filtering, sorting, grouping, and counting refiners

**Status:** TODO

**Files:** `src/refiners/filtering.rs`, `src/refiners/sorting.rs`, `src/refiners/aggregate.rs`, `src/schema_registry/mod.rs`

**Labels:** v0-2, refiners, filtering, sorting, aggregation



**Description:** Implement the structural refinement operators `where`, `unique`, `unique by`, `sort`, `sort by`, `reverse`, `count`, `count by`, and `group by`. These should work over typed result fields declared in noun schemas, not arbitrary runtime reflection. For example, `sort by fan_in desc` should be legal only when the current result type exposes a sortable `fan_in` field.

The implementation must preserve predictable semantics around stability, grouping keys, and field existence. Unknown keys should be rejected at plan time with suggestions drawn from the schema registry. For group/count output, define dedicated result types rather than stuffing aggregates into loosely typed maps. Add tests for representative noun families and for cross-type consistency between human and JSON output.


---

## [REFN-04] Implement projection refiners and `as <projection>` aliases

**Status:** TODO

**Files:** `src/refiners/projection.rs`, `src/schema_registry/mod.rs`, `src/nouns/mod.rs`

**Labels:** v0-2, refiners, projections, schemas



**Description:** Add projection-based refinement so commands can turn typed objects into other typed views such as `names`, `paths`, `roles`, `surfaces`, `summary`, `files`, and similar noun-defined projections. The projection system should be schema-driven: each noun declares which projections it supports and what result type each projection returns. `| names` and `| as names` must be equivalent in behavior and validation.

Projection is the hinge between structural and string-level work, so get the typing right. Projecting a `FileSet` to `names` should yield a `StringList`, enabling later string refiners, while projecting to `summary` might yield a record list. Add tests showing that unsupported projections fail clearly and that projected outputs advertise the correct schema/version in JSON mode.


---

## [REFN-05] Implement string-level fallback refiners on `StringList` only

**Status:** TODO

**Files:** `src/refiners/strings.rs`, `src/refiners/mod.rs`, `src/commands/plan.rs`, `src/commands/help.rs`

**Labels:** v0-2, refiners, strings, fallback, guardrails



**Description:** Implement the v0.2 string-level refiners—`chars`, `prefix`, `suffix`, `contains`, `split on`, and `join with`—but enforce that they apply only to `StringList` outputs, never to structural sets like files or symbols directly. This restriction is important to ROY’s identity: string munging should be the final stage after an intentional projection, not the default substrate.

Document and test the fallback nature of these refiners. `help refiners` should mark them as last-resort operations and the planner should suggest projecting to strings first when a user tries to apply them to a structural type. Add tests for Unicode-safe slicing behavior, split/join round-tripping, and the exact diagnostic path from illegal structural use to corrected projected use.


---

## [TRACE-01] Build typed trace engine for callers, callees, dependents, imports, and fields

**Status:** TODO

**Files:** `src/trace/mod.rs`, `src/trace/graph.rs`, `src/nouns/deps.rs`, `src/nouns/symbols.rs`

**Labels:** v0-2, trace, graph, relations, core



**Description:** Implement the `trace` verb over a typed relation engine rather than bespoke per-command walkers. The engine should support relations such as callers, callees, dependents, imports, and fields, accept an explicit target plus optional depth, and return either tree-shaped or flat relation results. This is the structural graph-navigation counterpart to `read` and needs to feel precise, not heuristic.

Keep relation definitions explicit so help/schema output can say exactly what each trace means and what graph it traverses. Multi-hop traces should preserve path information and depth metadata for later flattening or graph rendering. Add tests for one-hop and multi-hop traces, directionality, and tree-to-flat projection compatibility.


---

## [EXPL-01] Implement `explain` verb with noun-specific natural-language summaries

**Status:** TODO

**Files:** `src/explain/mod.rs`, `src/nouns/files.rs`, `src/nouns/modules.rs`, `src/nouns/denials.rs`, `src/render/human.rs`

**Labels:** v0-2, explain, ux, semantic-map, language



**Description:** Add the `explain` verb as a dedicated natural-language layer over typed noun results. `explain file`, `explain module`, `explain hotspot`, `explain symbol`, and `explain denial` should each produce concise, grounded prose answering “what is this” or “why did this happen” using the best available semantic metadata. This should not be implemented as a generic debug dump; each noun deserves a tailored explainer.

The implementation should be deterministic enough for tests. That means assembling prose from explicit fields and templates rather than an unconstrained freeform generator. Also make sure explainers are honest about confidence and data gaps—if a role/surface badge is missing, the explainer should say less, not invent. Add golden tests for representative explain commands and for JSON mode behavior if explain results are later structured.


---

## [CHG-01] Implement mutation pipeline with draft, review, apply, and validation linkage

**Status:** TODO

**Files:** `src/change/mod.rs`, `src/policy/engine.rs`, `src/session/artifacts.rs`, `src/storage/sqlite.rs`, `src/shell/runtime.rs`

**Labels:** v0-2, change, mutations, policy, artifacts



**Description:** Build the single mutation path described in the spec: propose (`draft`), inspect, apply, validate, and record. This issue should define the overarching change pipeline types and state transitions rather than the noun-specific operations themselves. A proposed change must create a diff artifact without mutating the workspace; an applied change must link back to that artifact, flow through policy, and record enough metadata for later `review`.

Do not allow mutating shortcuts that bypass artifacts or ledger recording. The whole point of ROY’s mutation surface is that every write is inspectable and attributable. This issue should also define how the runtime returns approval-pending vs auto-applied results and how validation artifacts are attached post-apply. Add end-to-end tests using small fixture workspaces.


---

## [CHG-02] Implement file mutation operations: create, replace lines, apply patch, move, copy, delete

**Status:** TODO

**Files:** `src/change/file.rs`, `src/capabilities/fs.rs`, `src/workspace/boundary.rs`, `src/session/artifacts.rs`

**Labels:** v0-2, change, files, mutations, fs



**Description:** Implement the closed set of file operations from the v0.2 spec: `create with:<ref>`, `replace lines A..B with:<ref>`, `apply patch:<ref>`, `move to <path>`, `copy to <path>`, and `delete`. Each operation should be explicit, workspace-bounded, and reviewable. In particular, delete must be modeled as a high-risk operation that always flows through policy rather than a direct filesystem call.

Keep operation parsing separate from execution so the planner can validate signatures and required refs before touching disk. Reuse the existing diff artifact machinery where possible, but extend it to cover move/copy/delete semantics cleanly. Add tests for happy-path behavior, patch application conflicts, line-range replacement accuracy, and workspace-boundary enforcement.


---

## [CHG-03] Implement symbol mutation operations: rename, move, and patch-by-span

**Status:** TODO

**Files:** `src/change/symbol.rs`, `src/nouns/symbols.rs`, `src/trace/graph.rs`, `src/session/artifacts.rs`

**Labels:** v0-2, change, symbols, refactor, ast



**Description:** Implement the symbol-level mutation operations `rename to <name>`, `move to <module>`, and `apply patch:<ref>` scoped to a symbol span. These are higher-level than file edits and should preserve semantic intent where possible—for example, rename should update references within the workspace rather than acting like a dumb text replacement. If the language/index support is insufficient to do that safely, the operation must be denied or limited explicitly.

This issue should define the execution contract, safety checks, and fallback behavior for symbol mutations. Keep symbol changes tied back to file-level diff artifacts so review and validation remain consistent with the rest of the pipeline. Add tests for local rename on fixture code, disambiguation of symbol targets, and refusal modes when safe refactoring cannot be guaranteed.


---

## [CHG-04] Implement issue mutation operations: state, tag, assign, and note append

**Status:** TODO

**Files:** `src/change/issue.rs`, `src/issues/mod.rs`, `src/nouns/issues.rs`, `src/storage/sqlite.rs`

**Labels:** v0-2, change, issues, tracker, workflow



**Description:** Implement the closed set of `change issue` operations from the spec: `state:<value>`, `tag <name>`, `untag <name>`, `assign <actor>`, `unassign`, and `note with:<ref>`. These operations should work against both the local issue store and future external adapters, so they need a backend-neutral command and result model. Unlike file changes, issue mutations do not produce file diffs, but they still need ledger entries and reviewable change records.

Make operation validation strict: unknown states, duplicate tags, or invalid assignees should yield structured errors rather than silent coercion. Also preserve issue history so `review issue <id>` can later explain how the item evolved. Add tests for each operation, local-store persistence, and adapter pass-through semantics.


---

## [VAL-01] Extend `validate` verb to support file, module, changed, and workspace scopes

**Status:** TODO

**Files:** `src/capabilities/validation.rs`, `src/nouns/modules.rs`, `src/change/mod.rs`, `src/session/artifacts.rs`, `src/commands/help.rs`

**Labels:** v0-2, validate, testing, build, artifacts



**Description:** Expand validation beyond the current `check` command so the `validate` verb can target `file`, `module`, `changed`, and `workspace` scopes as promised by the language spec. The execution model should be honest about what scope means for the current language/toolchain. For example, a file-scoped validation may still have to run a broader command under the hood; if so, surface that in the result metadata rather than pretending the check was narrower than reality.

Every validation run must create a `validation_run` artifact with command, cwd, exit code, stdout, and stderr, and it should link back to the triggering change or session context when applicable. Add tests for missing-tooling cases, no-Cargo workspace behavior, and artifact generation across success and failure paths.


---

## [POL-01] Implement approval-pending outcomes and explicit `approve` flow for risky changes

**Status:** TODO

**Files:** `src/policy/engine.rs`, `src/change/mod.rs`, `src/session/events.rs`, `src/nouns/diffs.rs`, `src/shell/result.rs`

**Labels:** v0-2, policy, approvals, change, safety



**Description:** Extend the policy system so it can do more than allow or deny. For mutating operations above the current session’s approval threshold, return an explicit approval-pending outcome tied to a draft diff or change plan. The UI and command surface should then support an `approve <diff-id>` or equivalent explicit apply transition rather than forcing the user to reconstruct the original command.

This issue needs a concrete policy data model for approval thresholds, a stable record of pending approvals in the session ledger, and a reviewable object that captures what exactly is awaiting approval. Keep the implementation visible and inspectable: actors should be able to `find diffs state:awaiting-approval` and understand why they are blocked. Add tests for auto-approve, approval-pending, denial, and approval-after-review flows.


---

## [POL-02] Upgrade denial protocol to full structured payload with redirect and hint fields

**Status:** TODO

**Files:** `src/policy/engine.rs`, `src/session/artifacts.rs`, `src/nouns/denials.rs`, `src/render/human.rs`, `src/render/json.rs`

**Labels:** v0-2, policy, denials, ux, learning-surface



**Description:** Implement the full denial shape from the spec: `original`, `reason`, `redirect`, `policy_rule`, and `agent_hint`. The current denial system captures command and suggestion, but v0.2 requires a richer protocol that teaches both humans and agents how to recover. Make denial payloads explicit types, not loosely formatted strings, and render them consistently across terminal, review surfaces, and JSON mode.

Also add the standard redirect table for common Bash habits like `grep`, `find`, `cat`, `rm`, and `git diff/status`. The redirect should be a literal copyable ROY command whenever possible. Add tests ensuring every compat-trap denial carries a rule id and a meaningful redirect, and that reserved future nouns and unimplemented features also go through this same protocol.


---

## [COST-01] Add cost-class metadata to commands, nouns, and refiners

**Status:** TODO

**Files:** `src/commands/schema.rs`, `src/nouns/mod.rs`, `src/refiners/mod.rs`, `src/commands/plan.rs`

**Labels:** v0-2, cost, planning, performance, schemas



**Description:** Introduce explicit cost metadata so ROY can reason about and surface the relative weight of operations. The spec defines `instant`, `quick`, `slow`, and `expensive`, but the system needs a compositional model: noun queries have a base cost, filters can reduce it, and some refiners or relation depths increase it. The planner should combine these into the `--dry` plan and optionally annotate real execution results for diagnostics.

Keep this metadata declarative where possible. Command schemas, noun definitions, and refiner definitions should each advertise what they usually cost and what factors influence that cost. This will make later performance tuning far easier than scattering heuristics inside runtimes. Add tests for representative cost combinations and for stable user-facing text in dry-run output.


---

## [UI-01] Refit terminal composer around AST parsing, partial help, and structured results

**Status:** TODO

**Files:** `src/ui/layout/panels/terminal.rs`, `src/ui/layout/panels/terminal_model.rs`, `src/ui/layout/panels/command_line.rs`

**Labels:** v0-2, ui, terminal, language, ux



**Description:** Update the shell pane so it no longer thinks in terms of raw command strings and line-oriented output only. The terminal model should pass parsed/planned commands into the runtime, render structured results intelligently, and surface context-sensitive help flows without feeling like a separate screen. The composer should also support partial-command `?` help and make parse/planning errors visually distinct from runtime failures.

Do not throw away the current prototype aesthetic. The work here is primarily behavioral: better command handling, better result rendering, and cleaner transitions between input, plan, execution, and review. Add UI-focused tests where possible around command submission, help invocation, and session-ended behavior.


---

## [UI-02] Render typed noun results, tables, and summaries instead of plain transcript text only

**Status:** TODO

**Files:** `src/render/human.rs`, `src/ui/layout/panels/terminal.rs`, `src/ui/artifacts.rs`, `src/shell/result.rs`

**Labels:** v0-2, ui, rendering, terminal, results



**Description:** Build the human renderers for typed results so the terminal can show more than plain strings. `show hotspots`, `find files`, `show schema files`, and similar commands should render as concise tables or summaries rather than dumping opaque debug text. The key constraint is that these human renderers are views over typed results, not the source of truth; JSON and ref modes must continue to derive from the same underlying values.

Define renderer traits or modules per result type and keep them small enough to test. Long lists should truncate gracefully while making it obvious more results exist. Add tests or golden snapshots for representative outputs including files, symbols, modules, denials, artifacts, and schemas.


---

## [UI-03] Wire slide-in drawers to real nouns: activity, review, attention, diagnostics

**Status:** TODO

**Files:** `src/ui/layout/mod.rs`, `src/ui/layout/chrome.rs`, `src/ui/artifacts.rs`, `src/diagnostics/pane.rs`, `src/nouns/denials.rs`, `src/nouns/artifacts.rs`

**Labels:** v0-2, ui, drawers, review, diagnostics



**Description:** Make the prototype drawers operate on the new noun surfaces instead of hand-assembled slices of session state. The Activity drawer should be a projection over `session` events, the Review drawer over `artifacts` and recent diffs, the Attention drawer over `denials` and approval-pending items, and the Diagnostics drawer over planner/runtime metadata. This will keep the UI consistent with the command language instead of becoming a parallel bespoke interface.

Preserve the current visual framing but simplify the data paths: the same typed results used by commands should feed drawer components. Also make drawer contents drillable—selecting a denial or artifact should be able to open the fuller view or prime a review command later. Add tests for empty states, populated states, and synchronization with terminal-executed commands.


---

## [UI-04] Add help and schema browser surfaces inside the cockpit UI

**Status:** TODO

**Files:** `src/ui/layout/mod.rs`, `src/ui/layout/chrome.rs`, `src/commands/help.rs`, `src/nouns/schema.rs`

**Labels:** v0-2, ui, help, schemas, discoverability



**Description:** Add a dedicated in-cockpit help/schema browsing surface so the ROY language becomes self-teaching even before a user memorizes the command line. This does not replace `help` commands; it should be a convenient visual front-end over the same help and schema data. The browser should let users inspect verbs, nouns, filters, refiners, and schema versions without leaving the prototype chrome.

Keep the source of truth shared with command-line help so documentation cannot fork. This issue should include navigation for noun/verb detail pages, examples that are directly runnable, and a clear distinction between human-friendly help and machine schema details. Add tests or storybook-like snapshots for the main help views.


---

## [STOR-01] Extend SQLite schema for refs, issues, denials, and pending approvals

**Status:** TODO

**Files:** `migrations/002_language_state.sql`, `src/storage/sqlite.rs`, `src/session/engine.rs`, `src/issues/mod.rs`, `src/refs/mod.rs`

**Labels:** v0-2, storage, sqlite, migrations, state



**Description:** Add the persistence tables needed for the v0.2 language state: named refs, issues, structured denials, and approval-pending change records. The current store is a strong base for session events and artifact refs, but these new surfaces require queryable tables rather than only serialized event payloads. Define migrations carefully so the store remains inspectable and idempotent.

Keep the schema boring and explicit. Tables should be normalized enough for easy queries but not abstracted beyond necessity. Add storage APIs for insert/update/load/list operations and replay tests ensuring that a saved session can reconstruct its refs and issue context cleanly. Migration tests should verify that opening an existing database upgrades without data loss.


---

## [STOR-02] Add query APIs for noun-backed history and artifact lookups

**Status:** TODO

**Files:** `src/storage/sqlite.rs`, `src/nouns/artifacts.rs`, `src/nouns/denials.rs`, `src/nouns/sessions.rs`, `src/issues/mod.rs`

**Labels:** v0-2, storage, query, nouns, replay



**Description:** Build the storage-side query APIs that noun modules will need to back history-oriented commands. This includes searching artifacts by kind/time, listing denials by rule or command, reading sessions by id/time window, and looking up issues with filters. The goal is to prevent noun modules from each writing their own SQL and to keep historical behavior consistent across UI and command surfaces.

Design the query layer to return typed records rather than raw rows. It should support pagination or bounded result windows where appropriate so review commands remain responsive on larger histories. Add tests for every public query method and for common filter combinations used by the nouns defined in this backlog.


---

## [TEST-01] Add golden parser/planner tests for the full v0.2 command surface

**Status:** TODO

**Files:** `src/commands/plan_tests.rs`, `src/ui/layout/panels/command_line.rs`, `src/commands/mod.rs`

**Labels:** v0-2, tests, parser, planner, golden



**Description:** Create a comprehensive test suite covering the full command surface described in the v0.2 spec. This should include every example command, major error path, singular/plural noun misuse, refiner type mismatch, and flag/render mode. The point is not only correctness today but spec lock-in: once these tests exist, the language surface becomes much harder to accidentally erode.

Use golden-style fixtures for parse trees, planner outputs, and especially diagnostic messages that are important for agent learning. Keep the test data organized by section of the spec so future revisions are easy to apply. This issue is foundational because many of the backlog items above should land with new golden coverage rather than ad hoc assertions.


---

## [TEST-02] Add end-to-end workflow tests for onboarding, change, denial recovery, and review

**Status:** TODO

**Files:** `tests/workflows/onboarding.rs`, `tests/workflows/change_pipeline.rs`, `tests/workflows/denials.rs`, `tests/workflows/review.rs`

**Labels:** v0-2, tests, e2e, workflows, agent



**Description:** Add integration tests that exercise ROY the way an agent will actually use it. Cover at least the workflows spelled out in the spec: onboarding to an unknown codebase, finding a change location, drafting and applying a change, recovering from a denial, and reviewing the session/artifacts afterward. These tests should run against small fixture workspaces and assert both command results and resulting session/artifact state.

Focus on behavioral confidence rather than exhaustive unit coverage. If these workflows pass, ROY is converging on its real product promise. If they fail, the mismatch will be much more meaningful than a low-level unit failure. Make the fixtures easy to inspect and keep the command scripts readable enough to serve as executable examples.


---

## [DOC-01] Sync built-in help surfaces and examples with the v0.2 language spec

**Status:** TODO

**Files:** `src/commands/help.rs`, `src/commands/registry.rs`, `src/ui/layout/chrome.rs`, `README.md`

**Labels:** v0-2, docs, help, examples, discoverability



**Description:** Update every built-in help surface so it matches the v0.2 language spec exactly. That includes `help`, `help <verb>`, `help <noun>`, `help refiners`, and any inline examples rendered in the cockpit. The key requirement is consistency: the examples a user sees in-product must all be valid, useful, and aligned with the parser/planner behavior.

Avoid hand-maintained duplicated strings wherever possible. Help examples should be generated from or at least validated against the command registry and noun/refiner schemas. Add tests ensuring the top-level help output stays within the intended size budget and that every example command in help actually parses and plans successfully.


---

## [DOC-02] Add dedicated help pages for denials, schemas, refs, and mutation workflow

**Status:** TODO

**Files:** `src/commands/help.rs`, `src/schema_registry/mod.rs`, `src/refs/mod.rs`, `src/change/mod.rs`

**Labels:** v0-2, docs, help, denials, mutations, refs



**Description:** Extend the help system with the high-value pages that make ROY learnable for both humans and agents: `help denials`, `help schemas`, `help refs`, and `help change`. These pages should do more than list syntax—they should explain the recovery model, why named refs exist instead of shell variables, how schema versioning works, and how the draft/review/apply pipeline behaves.

Keep these help pages grounded in live system data where feasible. For example, `help schemas` should enumerate the current schema families, and `help denials` should pull examples from the redirect table rather than a disconnected markdown blob. Add tests for page availability, example validity, and consistency with the underlying command surface.


---

## [NOUN-01] Create shared noun trait, result model, and registry for all noun modules

**Status:** TODO

**Files:** `src/nouns/mod.rs`, `src/nouns/registry.rs`, `src/schema_registry/mod.rs`, `src/commands/plan.rs`

**Labels:** v0-2, nouns, architecture, core, registry



**Description:** Introduce a shared noun architecture so each noun module plugs into the language the same way. Define common traits or interfaces for discovery, singular lookup, summary rendering, read support, supported filters, projections, and result schemas. This avoids every noun inventing its own lifecycle and makes `help`, JSON rendering, and planner validation consistent.

The design should separate noun definition metadata from runtime execution so planner and help features can inspect nouns without running them. Also make room for nouns that are workspace-backed, session-backed, or meta-backed. Add tests verifying registry completeness and preventing duplicate noun names or mismatched schema registrations.


---

## [READ-01] Create unified read subsystem for files, symbols, artifacts, denials, and schemas

**Status:** TODO

**Files:** `src/read/mod.rs`, `src/nouns/files.rs`, `src/nouns/symbols.rs`, `src/nouns/artifacts.rs`, `src/nouns/schema.rs`

**Labels:** v0-2, read, nouns, architecture, content



**Description:** Implement a common read subsystem rather than scattering `read` behavior across noun modules in incompatible ways. `read` should mean “return the full content form of this noun,” but the content form differs: files return text, symbols return signature/docs/body, artifacts return typed bodies, schemas return schema documents. A unified layer will make the verb easier to reason about and easier to extend.

Define a read result enum or trait hierarchy that preserves schema identity and renders consistently across human and JSON modes. Add planner validation so only nouns with readable content participate in the `read` verb, with corrective errors for the rest. Tests should cover supported/unsupported noun combinations and payload shape.


---

## [SHOW-01] Create unified summary subsystem for `show` across all noun families

**Status:** TODO

**Files:** `src/show/mod.rs`, `src/nouns/mod.rs`, `src/render/human.rs`, `src/render/json.rs`

**Labels:** v0-2, show, nouns, summaries, architecture



**Description:** Build a common summary subsystem so `show` has predictable semantics across noun families. `show` should return a structural overview—not full content—and every noun module should declare exactly what that overview includes. Centralizing this will prevent each noun from inventing subtly different summary conventions and will make help/docs much cleaner.

The subsystem should also support both singular and plural summaries and preserve enough metadata for later refinement. For example, `show file` and `show files` are different shapes but both belong to the same summary concept. Add tests for summary completeness and for the distinction between show vs read vs explain.


---

## [HELP-01] Generate help output from registry and schema metadata instead of hand-written strings

**Status:** TODO

**Files:** `src/commands/help.rs`, `src/commands/registry.rs`, `src/schema_registry/mod.rs`, `src/refiners/mod.rs`

**Labels:** v0-2, help, docs, registry, maintainability



**Description:** Refactor help generation so it is derived from the command registry, noun registry, and schema metadata instead of duplicated strings spread across modules. This reduces drift, makes new nouns/refiners cheaper to ship, and lets context-sensitive help be authoritative by construction. The top-level `help` page should still be tightly curated, but the verb/noun/refiner details should be generated from the live surface.

Build explicit formatting layers so generated help remains readable and deliberate rather than dumping raw metadata. Also expose hooks for adding carefully written explanatory notes where the schema alone is insufficient. Add tests confirming that every registered noun and refiner appears in help and that removed registry entries cannot linger in docs.


---

## [JSON-01] Define stable result schemas and serde models for every noun/result type

**Status:** TODO

**Files:** `src/render/json.rs`, `src/schema_registry/mod.rs`, `src/nouns/mod.rs`, `src/refiners/types.rs`

**Labels:** v0-2, json, schemas, serde, api



**Description:** Define explicit serde models for every JSON-emitting result type instead of serializing ad hoc Rust structs directly. This gives ROY control over naming, versioning, compatibility, and omission rules. Each noun/result type should have a clear contract that maps onto the schema registry and can evolve with deliberate version bumps.

Keep the JSON models close to the language surface rather than the internal implementation structs, because internal execution state is likely to change more often. Add tests for serialized shape, version envelope, optional-field behavior, and backward-compatibility guarantees within a major version.


---

## [RENDER-01] Separate result execution from human rendering and machine serialization

**Status:** TODO

**Files:** `src/render/mod.rs`, `src/render/human.rs`, `src/render/json.rs`, `src/shell/result.rs`, `src/commands/plan.rs`

**Labels:** v0-2, rendering, architecture, separation-of-concerns



**Description:** Refactor the output path so execution returns typed values and renderers decide how those values appear in human, JSON, or ref form. Right now much of the shell runtime assumes primary output is a string; that will not scale to noun surfaces, projections, and machine modes. This separation is also what keeps the default terminal render from becoming a hidden API surface.

Define a single typed result container that includes value, schema identity, render mode hints, cost metadata, and optional diagnostics. Then implement format-specific renderers against that container. Add tests proving the same underlying value can be rendered multiple ways without re-execution or semantic drift.
