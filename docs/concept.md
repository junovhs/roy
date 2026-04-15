# ROY v0.2 — Concept

## What ROY is

ROY is a controlled shell host for agentic work.

It exists because coding agents follow environment more reliably than they follow instruction. If a broad fallback substrate is available, they will use it. If Bash exists, they will use Bash. If Bash is blocked but some other open-ended command surface remains, they will retreat there instead. This is not rebellion. It is adaptation.

ROY accepts that reality and answers it at the substrate level.

Instead of asking the agent nicely to avoid certain habits, ROY removes the wrong fallbacks and replaces them with owned capabilities, visible policy, structured denials, and preferred paths. The point is not to make agents weaker. The point is to make them powerful inside a world whose rules are explicit and whose tools are shaped for the work.

ROY is not a model product. It does not try to replace Claude Code, Codex, or whatever terminal-native agent comes next. It hosts them. Their reasoning remains theirs. Their terminal fluency remains useful. What changes is the world they inhabit.

ROY decides:

- what commands resolve
- what is denied
- what is redirected
- what evidence is required before work proceeds
- what verification is canonical
- what artifacts are preserved for review
- what machine surface the agent is allowed to inhabit

That is the thesis.

---

## The problem ROY is trying to solve

Most agent workflows are fragile because the intended interface is optional.

A team may build:

- a structured issue tool
- a semantic orientation tool
- a canonical verification tool
- a disciplined mutation flow
- a safe review process

But the moment the agent is uncertain, impatient, or under-specified, it can often drop below all of that into the universal substrate: shell commands, ad hoc text manipulation, noisy repo-wide probing, quoting mistakes, or improvised composition.

That means the carefully designed workflow is not the physics of the system. It is just a suggestion sitting next to a bigger and more permissive machine surface.

ROY exists to reverse that relationship.

In ROY, the intended path is not a suggestion at the edge of the environment. It is the environment.

---

## The key idea

The best way to control an agent is not to convince it. It is to constrain the world it lives in.

That is the entire product idea.

A coding agent is a practical opportunist. It reaches for whatever gives it the most freedom with the least friction. So the right way to shape behavior is not moral instruction. It is environmental design.

If the wrong tool is available, the agent will eventually use it.  
If the wrong fallback is removed and a better owned path is present, the agent will adapt.

ROY therefore does not try to be a crippled shell. A crippled shell is frustrating and weak. ROY aims to be a useful shell-shaped world with stronger rules.

It must still support the real jobs agents need to do:

- orient in a workspace
- inspect files and structures
- search and discover
- mutate state safely
- validate work
- recover from mistakes
- leave behind inspectable evidence

The difference is that these jobs should happen through capabilities that the operator owns, not through an open-ended substrate that merely happens to be available.

---

## What makes ROY different

### 1. ROY owns command resolution

ROY is not just decorating an existing shell. It is the place where command meaning is decided.

A command is not valid because the host machine happens to know it. A command is valid because ROY chooses to expose it.

That allows the environment to be opinionated on purpose.

### 2. ROY treats denial as part of the product

A denial is not just an error. It is a teaching event.

If the agent reaches for a forbidden or out-of-bounds behavior, ROY should not merely refuse. It should redirect toward the intended path. The environment should teach its own physics.

A good ROY denial says, in effect:

- that path is unavailable here
- here is why
- here is the owned capability you should use instead

That makes “no” part of the interface rather than a dead end.

### 3. ROY is built for operator-owned toolchains

ROY should make it easy to host opinionated tools and workflows that already exist outside it.

Examples include:

- orientation and discovery tools
- canonical verification tools
- issue-management tools
- testing tools
- review tools
- domain-specific command packs

ROY does not need to absorb every such tool into one monolith. Its role is to make them first-class in the environment and to make routing around them harder or impossible.

### 4. ROY is designed for inspectability

An agent session should not collapse into transcript sludge.

Important actions and states should be visible and reviewable:

- denials
- approvals
- validation runs
- diffs
- session events
- notable outputs
- workflow evidence

The human operator should not have to reconstruct what happened by reading an undifferentiated stream of terminal text.

---

## The intended experience

To the agent, ROY should feel like a valid shell environment. That matters because terminal-native agents already know how to inhabit shells.

To the human operator, ROY should feel like a controlled workstation:

- visible state
- clear boundaries
- policy in the loop
- workflow evidence
- reviewable artifacts
- strong verification
- safer defaults

To the architecture, ROY should feel like a host with rules:

- bounded workspace semantics
- owned command surface
- capability routing
- session and artifact ledger
- policy enforcement at the point of action

That tension is part of the elegance of the product. ROY is shell-shaped on the outside and governed on the inside.

---

## Design principles

### Environment over prompting

Behavior changes most reliably when the world changes, not when the instructions get longer.

### Replace, do not merely forbid

A blocked behavior should have a better owned replacement. ROY should not merely take tools away. It should make the right tools the viable ones.

### Small, explicit work beats noisy wandering

Agent effectiveness improves when scope is bounded and context is deliberate. ROY should support focused operation rather than broad, noisy repo-wide probing.

### Evidence over claims

Work is not complete because the agent says it is complete. Completion should be grounded in explicit verification, workflow evidence, and reviewable state.

### Operator ownership matters

ROY should empower practitioners to define the machine surface their agents inhabit without depending on a model vendor to ship that worldview for them.

### Inspectability is a feature, not an afterthought

If the system denies, redirects, verifies, approves, or records something important, that should be legible and reviewable.

---

## What ROY is not

ROY is not:

- a frontier model company
- a generic shell replacement for all computing
- an IDE trying to subsume every development workflow
- a prompt wrapper pretending to be a product
- a bag of tools with no governing center

ROY is a host.

Its job is to create the conditions under which terminal-native agents work through the operator’s chosen world instead of constantly escaping into the broadest substrate available.

---

## The first great use case

The clearest use case for ROY is an opinionated engineering workflow where the operator already has strong preferences about how the agent should work.

That workflow might require:

- structured orientation before source reading
- bounded working sets
- canonical verification
- strict issue hygiene
- disciplined testing
- reviewable mutation flow
- explicit denials for noisy or unsafe fallback behavior

Today, that discipline often lives in giant prompts, repeated reminders, and manual operator supervision.

ROY’s purpose is to move that discipline into the environment itself.

That is the real promise:
not just better instructions,
but a better world.

---

## Why this matters

If the substrate determines behavior, then owning the substrate is one of the most powerful things an operator can do.

Most current agent tooling assumes the broad fallback machine is unavoidable and then tries to guide the model around its worst habits. ROY starts from the opposite premise: if a fallback surface keeps undermining the intended workflow, then remove it and replace it with something better.

That opens a different class of product:

- shells that enforce doctrine
- workstations that make policy visible
- operator-owned environments for agent work
- systems where useful constraints are part of the machine, not just part of the speech

That is why ROY exists.

It is a shell built to host agents on purpose, not to indulge them.
