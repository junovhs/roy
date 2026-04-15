# Agent Protocol (SEMMAP-first)

Repo browsing allowed, but orientation/verification MUST be SEMMAP/Neti-driven. Act without unnecessary permission prompts, while still obeying real tool, sandbox, and approval constraints. `neti` and `semmap` are on PATH (`neti check`, `semmap generate`, `semmap trace`, `semmap cat`).

## Hard rule: no source before orientation

Before reading implementation source beyond task-defining docs, you MUST:

1. run `semmap generate` (unless already current for this repo state),
2. read `SEMMAP.md` and cite exact lines used (Purpose, relevant layer entries, hotspots),
3. run `semmap trace <entry_file>` when flow, ownership, or execution path matters,
4. state minimal working set: 2-5 files to read next, and why.

You may read task-defining docs first: this prompt. Issue discovery and updates MUST go through `ishoo` CLI against canonical store. After orientation, if you read files beyond working set, justify why SEMMAP + trace were insufficient.

## Required evidence per iteration

In any iteration that plans or changes code, include:

- SEMMAP lines used (Purpose + relevant layer entries + hotspots),
- exact `semmap trace ...` command(s), when applicable,
- exact `neti check` result after changes; read `neti-report.txt` if output truncates.

`neti check` is canonical verification. It already runs configured verification suite, including `cargo test`, configured Clippy gate, and Neti scan. Ad hoc `cargo test` / `cargo clippy` do **not** substitute.

If any evidence missing, stop and run missing SEMMAP/Neti steps first.

## Workflow

1. Run `semmap generate`, read `SEMMAP.md`, inspect issues with `ishoo`. Use `semmap deps` if dependency graph needed.
2. Write short Orientation: Purpose, entrypoint, trace target, hotspots, plan.
3. Run `semmap trace <entry_file>` for flow-dependent work or unclear ownership.
4. Declare minimal working set; read only those files (prefer `semmap cat`).
5. Make minimal edits within SEMMAP boundaries; in hotspots, keep diffs smaller and tests stronger.
6. After changes, delete existing `neti-report.txt`, then run `neti check` from repo/worktree root. Inspect regenerated `neti-report.txt` there for full output. Iterate until clean, or until only clearly pre-existing failures remain and are called out explicitly. You must resolve all technical debt before moving forward; never say “I didnt break it so im leaving it broken”.
7. If you manually exercise a CLI or user-facing flow, report exact command, important output, and exit code when relevant.

## Issue discipline

Work only through canonical issue store via CLI. Storage file is compressed binary; do not read or modify it directly.

Primary commands:

- `ishoo agenda --next` — source of truth for what is next
- `ishoo list --compact` — board state
- `ishoo show <ID>` — full task details
- `ishoo new "<Title>" --category <CAT> --labels <labels>` — create work
- `ishoo edit <ID> --description "<Text>" --depends-on <ID>` — refine/link work
- `ishoo set <ID> <status>` — set status (`active`, `backlog`, `done`)
- `ishoo help --all` — full command help

When refining or closing an issue, `Resolution` MUST include:

1. **What changed:** concrete code-change summary.
2. **Why:** architectural justification.
3. **Verification:** exact commands run + `neti check` outcome.
4. **Handoff:** note any newly unblocked issue.

Issue is only DONE when `neti check` = PASS and status is updated with `ishoo set <ID> done`.

## Minimal close-out

Final report for code work should usually include:

1. issue handled,
2. SEMMAP evidence used,
3. key files changed,
4. exact `neti check` outcome,
5. any manual CLI/UX verification,
6. whether issue records were updated.
