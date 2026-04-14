# ROY v0.2 — Architecture

## Purpose

ROY is a controlled shell host for terminal-native agents.

Its architecture exists to solve one problem: if an agent has access to a broad fallback substrate, it will eventually use it. ROY therefore owns the environment itself. It decides what commands resolve, what is denied, what is redirected, what evidence is required, and what session state is preserved for review.

The architecture is built around that responsibility.

---

## Architectural position

ROY is **not** a model, and it is **not** just a tool bundle.

ROY sits between a terminal-native agent and the underlying machine. It acts as:

- a shell host
- a command-resolution and policy engine
- an integration host for operator-owned applications
- a workflow and evidence boundary
- a session and artifact ledger
- a workstation UI around that controlled environment

In architectural terms, ROY is the **governed runtime surface** for agent work.

That means ROY must remain focused on the host role. Rich tools, project-specific workflows, and higher-level systems may be integrated into that host, but they are not the center of the system. The center is the controlled environment itself.

---

## Design goals

### 1. Own the substrate

ROY must decide what machine surface is available to the agent. The host OS is not the source of truth for command meaning.

### 2. Deny with intent

A blocked behavior should not produce a dead-end error. It should produce a structured, inspectable redirect into the intended path.

### 3. Keep workflow visible

Important events should not disappear into transcript blur. Denials, approvals, validations, diffs, and other high-value outputs should become first-class session artifacts.

### 4. Support operator-owned applications

ROY should make it easy to integrate serious external applications and workflows without requiring those applications to be fused into the host core.

### 5. Preserve terminal-native usability

Agents like Claude Code and Codex should be able to inhabit ROY naturally. The environment must remain shell-shaped enough to host them while still being governed internally.

### 6. Stay modular and small at the core

The host must remain understandable and stable even as more integrated applications are added around it.

---

## Non-goals

ROY is not trying to be:

- a general-purpose replacement for every shell workflow
- a monolithic IDE
- a frontier model product
- a grab bag of tools with no governing center
- an all-in-one semantic analysis engine
- an issue tracker, test runner, verifier, or orientation engine in its own right

ROY should host and integrate those kinds of concerns cleanly, not absorb them indiscriminately.

---

## Top-level system model

ROY has six core architectural responsibilities:

1. **Agent hosting**
2. **Command and policy control**
3. **Application integration**
4. **Session and artifact recording**
5. **Persistence and replay**
6. **Operator-facing workstation UI**

These responsibilities map to a small number of stable subsystems.

---

## Core subsystems

## 1. Agent host subsystem

The agent host subsystem is responsible for embedding terminal-native agents inside ROY.

It handles:

- discovering supported agent binaries
- launching and supervising them
- attaching them to PTY-backed terminal sessions
- routing terminal input and output
- tracking whether an agent session is active
- keeping agent runtime concerns separate from ROY-native command handling

The architectural rule is simple:

**Agents inhabit ROY; they do not own ROY.**

ROY may host Claude Code, Codex, and future terminal-native agents, but those agents remain guests inside an environment whose policy and command surface are owned by ROY.

### Responsibilities

- adapter contract for each hosted agent
- process lifecycle and auth detection
- PTY integration
- raw input/output bridging
- agent-session state tracking

### Constraints

- agent adapters should remain narrow and replaceable
- agent-specific logic should not leak into the general shell runtime
- terminal rendering fidelity matters because the agent experiences ROY through that surface

---

## 2. Shell and control-plane subsystem

The shell subsystem is the governed execution surface.

It owns:

- shell runtime
- command interception
- dispatch
- built-in behaviors
- compatibility traps
- environment and current directory behavior
- prompt and exit status semantics

This is the heart of the product.

The shell runtime should be understood as a **control plane**, not as a generic pass-through terminal. A command only exists if ROY exposes it. A denial only exists if ROY chooses to deny it. A redirect only exists if ROY provides the replacement path.

### Responsibilities

- command intake
- resolution against ROY-owned surfaces
- compatibility behavior for familiar shell habits
- workspace-bounded execution context
- integration with policy, applications, and session recording

### Constraints

- the shell should remain shell-shaped, not shell-neutral
- broad fallback behavior must not sneak back in through convenience shortcuts
- “host OS happens to know this command” must never become the source of truth

---

## 3. Policy subsystem

Policy is one of ROY’s defining organs.

It sits in the path of action and determines whether a command or invocation is:

- allowed
- denied
- approval-gated
- transformed in some policy-relevant way

Policy is not merely a safety afterthought. It is part of the meaning of the environment.

### Responsibilities

- command and application invocation evaluation
- risk classification
- profile-based permission rules
- structured denials
- approval-pending outcomes
- redirect and hint attachment

### Constraints

- policy outcomes must be typed and inspectable
- denials should be reviewable after the fact
- policy logic should remain independent of UI wording
- policy should evaluate the intended action, not just raw text

---

## 4. Application integration subsystem

ROY is designed to integrate operator-owned applications.

These applications may handle orientation, verification, issues, testing, review, or domain-specific work. They remain real applications in their own right. ROY does not need to absorb them into one monolith. Its role is to host them behind a controlled interface.

The architectural rule is:

**ROY dictates the agent-facing interface. Integrated applications supply functionality behind that interface.**

An external application does not present an arbitrary freeform interface directly to the agent. It is mounted through a ROY-owned adapter boundary and exposed through ROY-owned command, policy, result, and artifact contracts.

### Responsibilities

- define the boundary for integrating external applications
- expose integrated application functionality through ROY-owned commands and results
- keep execution inside workspace and policy boundaries
- translate external application behavior into ROY session and artifact models

### Constraints

- integrated applications must not bypass workspace or policy boundaries
- external application surfaces must not leak directly into the agent-facing environment without ROY mediation
- ROY remains the owner of:
  - command resolution
  - help and discoverability
  - policy enforcement
  - result shaping
  - artifact promotion
  - persistence
  - UI projection

### Integration model

Each integrated application should enter ROY through an **integration module** backed by an **adapter**.

An integration should provide:

- registration metadata
- command exposure through ROY’s surface
- help/discoverability metadata
- policy/risk metadata
- execution hooks
- structured results or artifact mappings

The external application remains independent. The ROY integration module is the thin boundary that maps that application into ROY’s world.

This keeps the dignity and autonomy of the application intact while preserving ROY’s authority over the environment.

---

## 5. Session and artifact subsystem

A ROY session must be more than scrollback.

The session subsystem records the ordered ledger of what happened during work. The artifact subsystem promotes important outputs above transcript text into typed, inspectable objects.

Together, these are what make the system reviewable.

### Session responsibilities

- maintain ordered event history
- record command attempts, outputs, denials, notices, and lifecycle events
- support replay and filtering
- provide a durable notion of session state

### Artifact responsibilities

- capture high-value outputs such as:
  - denied-command traces
  - validation runs
  - diffs
  - future reviewable outputs
- make these outputs queryable and visible in the UI
- decouple important results from raw terminal transcript

### Constraints

- the ledger must remain stable and replayable
- artifacts should be typed, not just blobs of unstructured text
- storage and UI should both consume the same artifact model

---

## 6. Persistence subsystem

ROY needs durable local state.

Persistence is not the product’s center, but it is essential to trust. Sessions, artifacts, denials, approvals, named references, and other host-level history must survive beyond one live runtime.

SQLite is the right default posture: local, inspectable, boring, durable.

### Responsibilities

- session persistence
- artifact persistence
- denial and approval persistence
- query APIs for review surfaces
- future support for additional host-owned state

### Constraints

- the storage layer should be boring and explicit
- storage APIs should follow the host’s typed models
- persistence should support replay and UI review without forcing the UI to reconstruct history from transcript text

---

## 7. Workspace subsystem

ROY’s world is scoped, not machine-wide.

The workspace subsystem defines the allowed filesystem region and current-directory semantics for a session.

### Responsibilities

- declare workspace root
- normalize and validate paths
- enforce boundary checks
- maintain workspace-scoped current working directory
- surface workspace identity to the shell and UI

### Constraints

- silent escape from the declared workspace must never happen
- path handling must remain explicit and testable
- workspace identity is part of the session state, not an incidental detail

---

## 8. Terminal and rendering subsystem

ROY is shell-shaped to the agent, which means terminal fidelity is not cosmetic. It is part of the runtime contract.

The terminal subsystem is responsible for rendering PTY-backed agent sessions accurately and for presenting ROY-native output coherently alongside them.

### Responsibilities

- PTY screen state handling
- terminal cell/grid representation
- viewport and scrollback behavior
- color, cursor, and layout fidelity
- integration with terminal-native agents
- coexistence of agent terminal flow and ROY-native shell results

### Constraints

- terminal fidelity should not be sacrificed lightly; incorrect terminal behavior changes what the agent believes the environment is
- terminal implementation details should remain insulated from shell policy and storage concerns
- the agent terminal and ROY-native control surfaces must coexist without confusing authorship or ownership of output

---

## 9. Workstation UI subsystem

The UI exists to make the governed environment legible to the human operator.

ROY’s UI is not merely a transcript window. It is a workstation around the controlled shell.

### Responsibilities

- render the cockpit shell
- display terminal session state
- surface workspace context
- present activity, artifacts, review, and diagnostics drawers
- expose policy and artifact state visibly
- make it obvious that the terminal is part of a larger controlled system

### Constraints

- the UI should consume host state rather than invent parallel state models
- drawers and panels should be projections of session, artifact, policy, and storage state
- ROY should remain usable as a host even if the UI evolves significantly

---

## Runtime flow

At a high level, the runtime flow is:

1. **Input enters ROY**
2. **ROY resolves what that input means**
3. **Policy evaluates the intended action**
4. **ROY either denies, redirects, gates, or dispatches**
5. **Dispatch routes to either:**
   - a ROY-native builtin
   - an integrated application through its ROY adapter
   - an active hosted-agent terminal path
6. **Results are recorded into session state**
7. **Important outputs are promoted to artifacts**
8. **Persistence stores durable state**
9. **UI projects the resulting state back to the operator**

This order matters. The environment must remain in control from the moment input arrives.

---

## Integration model

The integration model should be deliberately conservative.

ROY should allow external applications to be integrated, but it should avoid turning the host into a loose plugin bazaar. Integrations must fit the architecture rather than dissolve it.

An integrated application should provide functionality through a ROY integration module. That integration module may expose:

- registration metadata
- commands within ROY’s surface
- help/discoverability text
- policy/risk metadata
- execution hooks
- structured results or artifacts

ROY remains the owner of:

- command resolution
- policy enforcement
- workspace boundaries
- session recording
- artifact promotion
- persistence
- UI projection

That keeps the environment coherent even when the operator’s application ecosystem grows.

---

## Architectural boundaries

To keep ROY focused, the following boundaries should remain explicit.

### Host vs integrated application

ROY decides what can happen and how it is surfaced to the agent. The integrated application performs a specific kind of work behind that boundary.

### Host vs agent

Agents execute inside the host. They do not define the environment.

### Policy vs rendering

Policy decides the outcome. Rendering decides how that outcome is shown.

### Session/artifact model vs transcript

Session and artifact state are the durable truth. Transcript text is one projection of that truth, not the only one.

### Persistence vs UI

The UI should not invent history models. It should project durable host state.

### Workspace vs filesystem at large

The workspace defines the allowed world. The host machine is not the default scope.

---

## What should remain small

If ROY is healthy, the following things remain small and sharply defined:

- the host/runtime core
- the policy surface
- the workspace/boundary model
- the session/artifact model
- the application integration boundary
- the persistence contracts between runtime and storage

This is how ROY avoids becoming a swollen monolith.

---

## What can grow safely

The following things can grow without threatening the product’s center, provided they remain behind clear boundaries:

- agent adapters
- integrated applications
- UI projections and drawers
- artifact viewers
- storage query surfaces
- workflow-specific help and review surfaces

Growth is safe when it adds power around the host without blurring what the host is.

---

## Architectural summary

ROY’s architecture is built around one controlling idea:

**the host owns the environment.**

Everything else follows from that:

- agents are embedded, not obeyed
- commands are resolved by ROY, not by ambient machine availability
- policy sits in the path of action
- integrated applications provide functionality behind ROY-owned interfaces
- sessions and artifacts preserve what happened
- persistence makes the history durable
- the UI makes the governed system legible to the operator

If this remains true, ROY stays coherent.

If this stops being true, ROY becomes just another shell with decorations.
