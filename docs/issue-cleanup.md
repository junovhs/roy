I would **not delete** the sprawling issues. I would do three things:

1. **Externalize** the language-heavy backlog into a separate doc partition so it stops polluting ROY’s board.
2. **Normalize** the live ROY board around a small label vocabulary and a few focused categories.
3. **Replace the plan** with two concrete initiatives:
   - terminal hardening
   - operator-owned application integrations

Because I can’t reach your live `.ishoo` store from here, the best I can do is give you the exact rewrite I’d have another agent execute.

## 1. Target label vocabulary

Keep the whole board under this label set:

`v0-2, host, terminal, agents, policy, workspace, session, storage, ui, integration, docs, tests, windows, fidelity, verification, workflow, externalize`

That is 17 labels total.

A few rules:

- `v0-2` stays on everything current
- `externalize` means “belongs to the split-out language repo / doc, not the ROY core board”
- no more labels like `nouns`, `schemas`, `parser`, `semantic-map`, `discoverability`, `rendering`, `mutations`, `review`, `tracking`, etc. Those are either title material or doc partition material now

## 2. What stays on the ROY board

Keep and normalize these as ROY-core issues:

- `BUG-03` → rename to `QA-01`
- `TRM-02`
- `TRM-06`
- `TRM-08`
- `TRM-09`
- `POL-03`
- `ISSU-02` → rename to `INT-01` and reframe as issue-tracker integration boundary

These are the ones that still clearly serve ROY’s host/product thesis.

## 3. What gets externalized

Move these out of `issues-active.md` into something like `issues-language-split.md` and relabel them to just `v0-2,externalize`:

`ART-02 CHG-01 CHG-02 CHG-03 CHG-04 COST-01 DEN-01 DEP-01 DIFF-01 DOC-01 DOC-02 EXPL-01 FIND-01 FIND-02 HELP-02 HOT-01 ISSU-01 JSON-01 LANG-03 LANG-04 LANG-05 LANG-06 LANG-07 LANG-09 LANG-10 MOD-01 POL-02 READ-01 REFN-01 REFN-02 REFN-03 REFN-04 REFN-05 SES-02 SHOW-01 SYM-01 SYM-02 TAX-01 TAX-02 TEST-01 TEST-02 TRAC-01 UI-01 UI-02 UI-03 UI-04 VAL-01`

That preserves the work, keeps references intact, and gets the ROY board back under control.

## 4. New issues to create

Create these new ROY issues:

- `INT-02` — Add ROY integration registry and application manifest boundary
- `INT-03` — Integrate SEMMAP as a ROY-hosted application
- `INT-04` — Integrate Neti as a ROY-hosted application
- `INT-05` — Integrate Ishoo as a ROY-hosted application
- `WF-01` — Enforce workflow doctrine gates for orientation, verification, and issue hygiene
- `TEST-03` — Add end-to-end host workflow tests for integrations, denials, and doctrine gates

That gives you a board aligned to the product you actually want.

## 5. Reframed issue meanings

I would retitle and relabel the kept issues like this:

- `QA-01` — Fix Windows path-comparison failures in shell builtin tests
  Labels: `v0-2,host,tests,windows`

- `TRM-02` — Prune forked alacritty_terminal to ROY-relevant core
  Labels: `v0-2,terminal,host`

- `TRM-06` — ROY VTE hooks: intercept layer for control plane
  Labels: `v0-2,terminal,policy,host`

- `TRM-08` — Terminal parity: exact cell renderer for agent PTY
  Labels: `v0-2,terminal,ui,fidelity`

- `TRM-09` — Terminal parity: replay and screenshot fidelity harness
  Labels: `v0-2,terminal,tests,fidelity`

- `POL-03` — Upgrade core denial payloads with redirect and hint fields
  Labels: `v0-2,policy,workflow,host`

- `INT-01` — Add issue-tracker integration adapter boundary for operator-owned applications
  Labels: `v0-2,integration,workflow,storage`

- `INT-02` — Add ROY integration registry and application manifest boundary
  Labels: `v0-2,integration,host,docs`

- `INT-03` — Integrate SEMMAP as a ROY-hosted application
  Labels: `v0-2,integration,workflow`

- `INT-04` — Integrate Neti as a ROY-hosted application
  Labels: `v0-2,integration,verification`

- `INT-05` — Integrate Ishoo as a ROY-hosted application
  Labels: `v0-2,integration,workflow,storage`

- `WF-01` — Enforce workflow doctrine gates for orientation, verification, and issue hygiene
  Labels: `v0-2,workflow,policy,host`

- `TEST-03` — Add end-to-end host workflow tests for integrations, denials, and doctrine gates
  Labels: `v0-2,tests,workflow,integration`

## 6. New plan

This is the plan I’d replace the current one with:

1. `QA-01`
2. `TRM-02`
3. `TRM-06`
4. `TRM-08`
5. `TRM-09`
6. `POL-03`
7. `INT-01`
8. `INT-02`
9. `INT-03`
10. `INT-04`
11. `INT-05`
12. `WF-01`
13. `TEST-03`

That gets you through two important initiatives:

### Initiative A — production-grade terminal host

- Windows stability
- leaner owned terminal core
- control-plane interception layer
- exact cell rendering
- replay/screenshot verification harness

### Initiative B — operator-owned application integrations

- denial payload upgrade
- application integration boundary
- integration registry
- SEMMAP / Neti / Ishoo integrations
- doctrine enforcement
- end-to-end workflow tests

## 7. Exact CLI batch I would run

```bash
# 0) Inspect before changes
ishoo labels
ishoo plan show

# 1) Externalize language-heavy backlog out of the ROY board
ishoo move \
  ART-02 CHG-01 CHG-02 CHG-03 CHG-04 COST-01 DEN-01 DEP-01 DIFF-01 \
  DOC-01 DOC-02 EXPL-01 FIND-01 FIND-02 HELP-02 HOT-01 ISSU-01 JSON-01 \
  LANG-03 LANG-04 LANG-05 LANG-06 LANG-07 LANG-09 LANG-10 MOD-01 POL-02 \
  READ-01 REFN-01 REFN-02 REFN-03 REFN-04 REFN-05 SES-02 SHOW-01 SYM-01 SYM-02 \
  TAX-01 TAX-02 TEST-01 TEST-02 TRAC-01 UI-01 UI-02 UI-03 UI-04 VAL-01 \
  --to issues-language-split.md

for id in \
  ART-02 CHG-01 CHG-02 CHG-03 CHG-04 COST-01 DEN-01 DEP-01 DIFF-01 \
  DOC-01 DOC-02 EXPL-01 FIND-01 FIND-02 HELP-02 HOT-01 ISSU-01 JSON-01 \
  LANG-03 LANG-04 LANG-05 LANG-06 LANG-07 LANG-09 LANG-10 MOD-01 POL-02 \
  READ-01 REFN-01 REFN-02 REFN-03 REFN-04 REFN-05 SES-02 SHOW-01 SYM-01 SYM-02 \
  TAX-01 TAX-02 TEST-01 TEST-02 TRAC-01 UI-01 UI-02 UI-03 UI-04 VAL-01
do
  ishoo edit "$id" --labels "v0-2,externalize"
done

# 2) Rename and normalize the live ROY-core issues
ishoo rename-id BUG-03 QA-01
ishoo edit QA-01 --labels "v0-2,host,tests,windows"

ishoo edit TRM-02 --labels "v0-2,terminal,host"
ishoo edit TRM-06 --labels "v0-2,terminal,policy,host"
ishoo edit TRM-08 --labels "v0-2,terminal,ui,fidelity"
ishoo edit TRM-09 --labels "v0-2,terminal,tests,fidelity"

ishoo edit POL-03 \
  --title "Upgrade core denial payloads with redirect and hint fields" \
  --labels "v0-2,policy,workflow,host"

ishoo rename-id ISSU-02 INT-01
ishoo edit INT-01 \
  --title "Add issue-tracker integration adapter boundary for operator-owned applications" \
  --labels "v0-2,integration,workflow,storage"

# 3) Create the new integration / doctrine issues
ishoo new "Add ROY integration registry and application manifest boundary" \
  --category int --labels "v0-2,integration,host,docs"

ishoo new "Integrate SEMMAP as a ROY-hosted application" \
  --category int --labels "v0-2,integration,workflow"

ishoo new "Integrate Neti as a ROY-hosted application" \
  --category int --labels "v0-2,integration,verification"

ishoo new "Integrate Ishoo as a ROY-hosted application" \
  --category int --labels "v0-2,integration,workflow,storage"

ishoo new "Enforce workflow doctrine gates for orientation, verification, and issue hygiene" \
  --category wf --labels "v0-2,workflow,policy,host"

ishoo new "Add end-to-end host workflow tests for integrations, denials, and doctrine gates" \
  --category test --labels "v0-2,tests,workflow,integration"

# 4) Add dependencies once the new IDs exist
# Expected new ids: INT-02 INT-03 INT-04 INT-05 WF-01 TEST-03
ishoo edit TRM-06 --depends-on "TRM-02"
ishoo edit TRM-09 --depends-on "TRM-08"

ishoo edit INT-02 --depends-on "INT-01"
ishoo edit INT-03 --depends-on "INT-02"
ishoo edit INT-04 --depends-on "INT-02"
ishoo edit INT-05 --depends-on "INT-02"
ishoo edit WF-01  --depends-on "POL-03,INT-03,INT-04,INT-05"
ishoo edit TEST-03 --depends-on "WF-01,TRM-09"

# 5) Replace the plan
ishoo plan clear
ishoo plan add QA-01
ishoo plan add TRM-02
ishoo plan add TRM-06
ishoo plan add TRM-08
ishoo plan add TRM-09
ishoo plan add POL-03
ishoo plan add INT-01
ishoo plan add INT-02
ishoo plan add INT-03
ishoo plan add INT-04
ishoo plan add INT-05
ishoo plan add WF-01
ishoo plan add TEST-03

# 6) Check the result
ishoo list --compact
ishoo labels
ishoo plan show
ishoo lint --strict
```

## 8. One thing I would _not_ do right now

I would **not** spend time mass-renaming every historical done issue unless you really want the archival history cosmetically perfect.

The payoff is in:

- cleaning the live board
- moving language work out
- normalizing the active label set
- getting a plan that matches the product

That gets you 90% of the value immediately.

If you want, I can do the next pass as a strict “historical taxonomy normalization” script for all landed issues too.
