# ROY v0.2.2 — Current Capabilities

This document describes what ROY can actually do now.

It is intentionally practical. It is not a roadmap and it is not a promise document. It distinguishes between:

- capabilities that are real and working now
- capabilities that are real but still rough
- internal foundations that exist in code but are not yet a stable user-facing product surface
- planned capabilities that do not exist yet

The standard for inclusion here is simple: if a user sat down with the current tree, what would they genuinely find?

---

## Executive summary

ROY is already a real controlled shell host for terminal-native agents.

Today, it can:

- host real agents like Claude Code and Codex inside ROY
- run them through a PTY-backed terminal surface
- control command resolution through ROY-owned runtime logic
- enforce workspace boundaries
- apply policy and compatibility denials
- record sessions and promote important outputs to artifacts
- persist and query host state in SQLite
- render a dedicated cockpit UI around the shell and agent session

ROY is **not yet** a complete operator-facing environment for richly integrated external applications, workflow doctrine enforcement, or broad noun-driven semantic operation. Some internal foundations for those directions exist, but they should not be mistaken for finished product surfaces.

The short version:

- **host/runtime:** real
- **agent embedding:** real
- **terminal fidelity:** real and advancing fast, but still under active parity work
- **policy/workspace/session/artifacts/storage:** real
- **application integration surface:** not finished
- **doctrine enforcement:** not finished
- **broad semantic operating surface:** foundational pieces exist, but large parts remain incomplete

---

## 1. Real product capabilities available now

## 1.1 Controlled shell host runtime

ROY has a real shell host runtime.

This is not just a concept or prompt convention. The runtime exists and owns:

- command intake
- command resolution
- built-in behavior
- compatibility traps
- workspace-scoped environment behavior
- prompt and exit-status handling
- dispatch into ROY-owned paths

What this means in practice:

- ROY is already a governed shell host rather than a generic terminal wrapper
- familiar shell habits can be intercepted and redirected
- ROY-native behavior can be routed through its own runtime instead of ambient PATH lookup

What this does **not** yet mean:

- ROY is not yet a fully mature replacement environment for every workflow an operator may want
- the long tail of controlled replacement behaviors is still under construction

Status: **real and central**

---

## 1.2 Hosted agent execution

ROY can host real terminal-native agents inside the shell.

Currently, this includes:

- Claude Code
- Codex

ROY has working adapter and session machinery for hosted agents, including:

- discovery
- process supervision
- auth handling
- session state
- agent lifecycle tracking
- shell dispatch hooks for launching agents

What this means in practice:

- ROY is not merely planning to host agents; it already does
- hosted agents can live inside a ROY-owned environment rather than being conceptual future targets
- the host/guest split is real in the implementation

Status: **real and significant**

---

## 1.3 PTY-backed terminal operation

ROY has moved past the early “fake transcript terminal” stage.

It now supports:

- PTY-backed agent execution
- raw keyboard passthrough to the agent session
- terminal state tracking via an owned/forked terminal stack
- alacritty-based render-state preservation
- viewport and scrollback support
- cursor rendering
- color and SGR handling
- alt-screen and TUI-oriented rendering behavior

What this means in practice:

- embedded agents can operate inside something much closer to a true terminal surface
- Claude Code-style and Codex-style TUI flows are part of the real product, not theoretical future work
- terminal fidelity is now a first-class architectural concern

What remains unfinished:

- parity and verification work is still active
- exact rendering fidelity is still being tightened
- replay/screenshot harness work is still backlog
- some test/platform issues remain open, especially around Windows path handling and final parity polish

Status: **real, substantial, still actively hardening**

---

## 1.4 Workspace-bounded execution

ROY has explicit workspace boundaries and workspace-scoped current-directory semantics.

This includes:

- declared workspace root
- path normalization and validation
- boundary enforcement
- workspace-scoped `cwd`
- denial of out-of-bounds movement or access through ROY-owned flows

What this means in practice:

- ROY sessions are not implicitly machine-wide
- the environment has a real notion of “the world the agent is allowed to inhabit”
- path handling is part of host control, not just a convenience wrapper

Status: **real and foundational**

---

## 1.5 Policy and compatibility denials

ROY has a real policy subsystem.

Today it already supports:

- profile-based policy evaluation
- allow/deny decisions
- compatibility-trap behavior
- human-visible denial experiences
- policy participation in command dispatch
- denial-related storage/query foundations

What this means in practice:

- ROY can already say “no” in a structured and intentional way
- blocked commands are not just missing; they can be made part of the product’s teaching surface
- ROY already behaves like a governed environment, not just a shell plus suggestions

What remains unfinished:

- the richer v0.2 structured denial protocol is not yet complete
- approval-pending and full explicit approve-flow work remains backlog
- denial noun/review surfaces are not yet fully productized

Status: **real, useful, not yet final-form**

---

## 1.6 Session ledger and artifact model

ROY already treats the session as more than scrollback.

It has:

- typed session events
- ordered session history
- artifact promotion for important outputs
- artifact categories such as denial/validation/diff-related outputs
- session-level recording and replay-oriented structure

What this means in practice:

- ROY already has the beginnings of a trustworthy audit trail
- important outputs are not forced to live only as transcript text
- the system already knows the difference between ordinary terminal chatter and higher-value reviewable outputs

Status: **real and strategically important**

---

## 1.7 Local persistence and queryable history

ROY has a SQLite-backed persistence layer.

Current persisted/queryable areas include:

- sessions
- events
- artifacts
- denials
- approvals
- issues
- refs
- session metadata and related lookup/query APIs

What this means in practice:

- ROY is already past the purely in-memory demo stage
- history and review can be grounded in durable local storage
- the host has a real state spine rather than ephemeral UI memory

What remains unfinished:

- not every future user-facing noun/review surface is fully built on top of this
- persistence exists more strongly than some of the final user-facing workflows that will consume it

Status: **real and ahead of some higher-level UX layers**

---

## 1.8 Cockpit-style UI

ROY has a real UI shell around the controlled environment.

Current UI capabilities include:

- cockpit layout
- shell-centered presentation
- terminal pane
- drawers / auxiliary panels
- activity/review/diagnostic-oriented supporting surfaces
- visual framing that makes the terminal part of a larger controlled system

What this means in practice:

- ROY is already more than a CLI experiment
- operator visibility is part of the product shape
- the shell is presented as one surface inside a broader workstation, not the whole product

What remains unfinished:

- many drawers still need to be wired to richer noun-backed or review-backed data flows
- the UI is ahead of some of the final result and workflow surfaces it is meant to visualize

Status: **real, usable, still maturing**

---

## 2. Internal foundations that are real in code

This section matters because the current tree contains real internal foundations that are easy to over-read as finished product capabilities.

They exist. They matter. But they are not yet the same thing as polished, operator-facing product surfaces.

## 2.1 Command parser and plan layer

ROY now has:

- a typed parser
- command AST
- tokenization and parsing infrastructure
- a planning layer
- classification concepts such as render mode, result typing, and mutation class

This is real and meaningful.

What it means:

- ROY is no longer only dispatching raw shell-like text
- the system has started building a more explicit internal execution model

What it does **not** yet mean:

- the full command surface envisioned by the broader v0.2 direction is already delivered
- planner-driven user experience is complete
- all downstream noun/refiner/render/help systems are present and stable

Status: **real foundation, not complete product surface**

---

## 2.2 Noun registry and schema registry

ROY now has:

- a noun registry
- schema registry
- schema data tables
- schema tests and coverage expectations
- result rendering separation tied to typed structures

This is a serious internal advance.

What it means:

- ROY has a real machine-readable substrate for describing parts of its command/result surface
- some future-facing result/help/render consistency work now has an actual backbone

What it does **not** yet mean:

- the broad noun ecosystem is shipped
- major noun families like files, symbols, modules, hotspots, deps, denials, sessions, artifacts, and issues are all complete in product terms
- the final operator-facing semantic surface is ready

Status: **real foundation, partial user value, far from complete breadth**

---

## 2.3 Result/render separation

ROY now has explicit render separation.

There is real code supporting:

- typed result containers
- human rendering
- JSON rendering
- separation between execution result and presentation format

What it means:

- ROY has crossed an important architectural threshold
- the system no longer has to treat all output as transcript-first text
- machine-readable and human-readable render paths have an actual boundary

What it does **not** yet mean:

- every important result family is implemented
- JSON and ref-oriented user-facing workflows are complete
- the UI fully consumes these structured results everywhere

Status: **real foundation, still propagating through the system**

---

## 3. What ROY can do today from a user/operator perspective

If someone used the current tree honestly, the strongest current story would be:

### A. Controlled agent hosting

ROY can host Claude Code and Codex in a real PTY-backed controlled shell environment with visible runtime structure.

### B. Policy-bearing environment

ROY can intercept, deny, and redirect within a ROY-owned command/runtime world instead of just yielding to the ambient machine.

### C. Workspace-bounded operation

ROY can keep the agent inside an explicit workspace scope.

### D. Reviewable host history

ROY can record sessions, store artifacts, and persist host-level state.

### E. Terminal-native UX

ROY can present the agent through a serious terminal surface rather than a toy emulator.

That is already enough to make ROY real.

What is **not yet** the strongest current story is:

- “ROY is already a complete semantic operating language for engineering work”
- “ROY already fully integrates external applications through a clean product-facing adapter system”
- “ROY already enforces full workflow doctrine as environmental law”

Those are directions, not present-tense core claims.

---

## 4. Active rough edges and hard truths

This section is intentionally direct.

## 4.1 Terminal work is real but not done

ROY’s terminal stack is now serious, but it is still under active parity work.

Open realities:

- exact cell rendering is still being tightened
- fidelity harness work is still pending
- there are still active terminal-specific backlog items
- platform-specific test issues remain open

Interpretation:

- terminal fidelity is no longer speculative risk
- it is now a concrete hardening track

---

## 4.2 Some internal surfaces are ahead of the user experience

Parser, planner, noun registry, schema registry, and typed results have moved faster than the fully integrated user-facing workflows around them.

That is not necessarily bad, but it means:

- some foundations are stronger than the visible product experience they are meant to support
- parts of the internal architecture have outrun the external story

Interpretation:

- the current tree contains real architecture work that is not yet fully realized as a user-facing system

---

## 4.3 External application integration is not yet the productized center

ROY clearly wants to be a strong host for operator-owned applications and workflows.

But at the current state, the explicit integration boundary for serious external applications is not yet the mature center of the shipped product.

That means:

- the idea is strong
- the architectural direction is visible
- the actual generalized integration surface is not yet finished

Interpretation:

- ROY can host agents well now
- ROY is not yet the finished “agent host for your full external application ecosystem” product

---

## 4.4 Workflow doctrine enforcement is not yet environmental law

ROY already has many ingredients that support doctrine:

- session ledger
- policy
- artifacts
- structured runtime control
- persistence
- UI visibility

But things like:

- mandatory orientation
- mandatory declared working set
- justification for scope expansion
- mandatory canonical verification before close-out
- workflow-oath style completion gates

are not yet first-class enforced host features.

Interpretation:

- a human can still impose doctrine around ROY
- ROY does not yet fully enforce that doctrine itself

---

## 5. What is clearly not done

The following should be treated as unfinished, even if pieces exist:

- broad noun family implementation
- unified read/show subsystems
- typed refiner engine
- full JSON/ref product surface
- named ref workflow
- mutation pipeline with draft/review/apply/approve flow
- rich denial protocol and denial review surfaces
- complete issue and external-tracker operating surface
- doctrine-first workflow enforcement
- full application integration model
- complete terminal fidelity verification harness

These are not small polish tasks. They are substantial areas of remaining product work.

---

## 6. Current architectural strengths

ROY’s strongest current architectural strengths are:

### The host is real

This is no longer mostly a design document. ROY genuinely hosts agents inside a controlled shell/runtime/UI system.

### The substrate is owned

Command resolution, policy, workspace, session, artifacts, and persistence are not imaginary. They are implemented concerns.

### The terminal bet is serious

ROY has committed to PTY-backed terminal fidelity instead of staying in a toy transcript world.

### The history spine exists

Session and artifact recording plus SQLite persistence give ROY a credible base for review and trust.

### The system has internal shape

Even unfinished areas are increasingly backed by explicit registries, result models, and typed boundaries rather than ad hoc glue.

---

## 7. Current architectural risks

ROY’s main current risks are:

### Terminal complexity risk

Terminal fidelity is valuable, but it can consume enormous energy if not kept disciplined.

### Foundation/product mismatch risk

Some internal architecture is more advanced than the corresponding operator-facing workflow.

### Surface-area expansion risk

The tree contains signs of broad ambition. ROY will need continued discipline to stay centered on the host role rather than becoming an everything-system.

### Integration-boundary risk

If external application integration is not given a crisp boundary, ROY could become overcoupled to individual tools instead of remaining a stable host.

---

## 8. Honest present-tense product statement

The most honest short description of ROY v0.2.2 right now is:

**ROY is a real controlled shell host for terminal-native agents, with PTY-backed terminal operation, policy-bearing command control, workspace boundaries, session/artifact history, local persistence, and an operator-facing cockpit UI. It also contains significant internal foundations for richer structured command, result, and review surfaces, but those broader surfaces are not yet fully finished as product capabilities.**

That is the current truth.

---

## 9. What this document deliberately does not claim

This document does **not** claim that ROY already is:

- a complete semantic engineering operating system
- a finished external-application integration platform
- a finished doctrine-enforcing workflow machine
- a finished terminal-fidelity endpoint
- a fully coherent end-user v0.2 surface across all nouns, reads, shows, changes, traces, reviews, and approvals

Those would be overclaims.

The right current claim is narrower and stronger:
ROY already works as a real controlled host, and it has moved well past mockup status.

That is enough to matter.
