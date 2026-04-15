# ROY Language Specification — v0.2

**Status:** Draft 0.2. Complete at altitude. Fillable.  
**Audience:** Spencer (design owner), future implementers (human or AI), and any agent that will operate inside ROY.  
**Companion:** ROY SEMMAP, omni-ast SEMMAP.  
**Changes since v0.1:** Example surface normalized to the eight-verb model (§1.1). `schema/schemas` added as explicit introspection nouns (§3.13). `task/tasks` isolated as reserved future nouns (§13.1). `--dry` wording corrected (§10.2). Default render no longer references an undefined `--all` flag (§11.1). Minor consistency fixes throughout.

---

## 0. What this document is

This is the full surface design of the ROY command language: every shape an actor (human or agent) can type, and every shape ROY can type back. It is not the implementation. It is the contract the implementation must satisfy.

The design follows three constraints, in priority order:

1. **The agent must not have Bash.** Every Bash habit it would reach for must have a native ROY path that is at least as good, or a deliberate denial that points at one.
2. **The substrate is semantic.** Commands address meaningful objects (files, symbols, hotspots, surfaces) before they address bytes.
3. **The shape is learnable in one sitting.** Few verbs, rich nouns, predictable refinement. An agent trained on Bash should not need to be re-trained — it should be able to read `help` and start working.

These three are in tension. Most of the design decisions below are negotiations between them.

---

## 1. The shape of a command

Every ROY command has the same shape:

```text
<verb> <noun> [<filter>...] [| <refiner>...]
````

That's it. There is no other top-level grammar. No subshells, no command substitution, no shell variable expansion, no backgrounding. If you need those, you are doing something ROY is deliberately not for.

### 1.1 Concrete examples

```text
find files about auth
find symbols role:controller surface:http-handler
show hotspots | top 5
read symbol dispatch
read file src/shell/runtime.rs lines 1..40
trace callers of resolve_command
explain module src/policy
show deps from:src/shell/runtime.rs depth:1
show diff staged
review denial last
validate changed
```

### 1.2 The pipe is structural, not textual

`|` in ROY is **not** a byte-stream pipe. It passes a typed result set from one stage to the next. A `find files` produces a `FileSet`; a `| top 5` consumes a `FileSet` and produces a smaller `FileSet`; a `| names` projects a `FileSet` to a `StringList`.

This is the single most important departure from Bash. In Bash, every pipe stage re-parses bytes and the meaning lives in the user's head. In ROY, the meaning lives in the type of the value flowing through the pipe, and ROY can introspect it for policy, denial, and display.

### 1.3 What's deliberately missing

| Bash thing         | Why it's not here                                                                                                                                   |        |                                                                            |
| ------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------- | ------ | -------------------------------------------------------------------------- |
| `&&`, `            |                                                                                                                                                     | `, `;` | One command per line. Sequencing is done by the agent loop, not the shell. |
| Subshells `$( )`   | Re-introduces text-substitution semantics. Use named refs (§7) instead.                                                                             |        |                                                                            |
| Variables `$FOO`   | Use the session's named refs (§7) and the `last` keyword (§1.7).                                                                                    |        |                                                                            |
| Backgrounding `&`  | ROY commands are foreground and inspectable. Long work will eventually go through the reserved `task/tasks` nouns (§13.1), not shell backgrounding. |        |                                                                            |
| Globs `*.rs`       | Use `find files lang:rust` or `find files path:src/**/*.rs` with explicit syntax.                                                                   |        |                                                                            |
| Redirects `>`, `<` | Use `save as <ref>` and `load <ref>` (§7). Output is structured, not bytes.                                                                         |        |                                                                            |

If the absence of one of these blocks a real workflow, the answer is to extend ROY's vocabulary, not to reintroduce the Bash form.

### 1.4 Whitespace and quoting

* Tokens are whitespace-separated.
* Double quotes group tokens with spaces: `find files about "user authentication"`.
* No backslash escapes inside quotes. If you need a literal quote, use single quotes around double-quoted content: `'he said "hi"'`. This is a design choice — ROY's quoting is for grouping multi-word values, not for arbitrary string construction.
* Identifiers (file paths, symbol names) do not need quoting unless they contain spaces.

### 1.5 Filters: `key:value` after the noun

Filters narrow the noun selection before refinement. They are typed and validated against the noun.

```text
find files lang:rust role:controller
find symbols name:dispatch* kind:fn
find hotspots in:src/shell
```

Filter keys come from the noun's schema (§3). Unknown filter keys are a hard error with a suggestion: `unknown filter 'language' for noun 'files'; did you mean 'lang'?`

Filter values support:

* Exact: `lang:rust`
* Glob: `name:dispatch*`
* Set: `kind:[fn,method]`
* Negation: `!role:test`
* Range: `lines:>200`, `fan_in:5..20`

**The filter/refiner rule, blunt:** *filters constrain what the source returns; refiners transform what's already been returned.* The behavioral test is whether the source can use an index to prune cheaply. `lang:rust` is a filter because the source (SEMMAP, file walker) can skip non-Rust files without reading them. `where lines>200` is a refiner because the source must produce candidates first, then materialization measures them. When in doubt: if the constraint is on a property the source already knows, it's a filter; if it's on a property the result has, it's a refiner.

This matters for cost (§10): filters are cheap, refiners scale with result-set size.

### 1.6 Singular and plural nouns

Most nouns in ROY exist in two forms: singular (`file`, `symbol`, `module`, `schema`) and plural (`files`, `symbols`, `modules`, `schemas`). These are **not aliases**. They are different nouns with different signatures and different result types.

| Form                           | Takes                      | Returns                                     |
| ------------------------------ | -------------------------- | ------------------------------------------- |
| Singular (`file`, `symbol`, …) | A single target identifier | A single typed object (`File`, `Symbol`, …) |
| Plural (`files`, `symbols`, …) | Filters                    | A list (`FileSet`, `SymbolSet`, …)          |

```text
show file src/shell/runtime.rs            # File      → object summary
show files lang:rust                      # FileSet   → list

read file src/shell/runtime.rs            # File body → text
read symbol dispatch                      # Symbol    → signature + body

find file ...                             # error: 'file' takes a target, not filters
                                          # use 'find files <filters>' or 'show file <path>'
```

This matters because it makes refiner types correct. `show file foo.rs` returns a `File`, not a one-element `FileSet`, so list refiners (`top`, `sort`, `where`) don't apply — you'd get a parse-time error pointing you at the right form. The two forms also map to different access patterns: singular is point-lookup against a known identifier; plural is index-scan with filters.

**Exception:** `deps` remains plural-first in v0.2 because dependency relations are naturally relation sets rather than point-lookups. If a singular `dep` surface becomes necessary later, it will be added deliberately.

The rule for choosing: **if you know exactly which one you want, use the singular form with its identifier; if you're discovering or filtering, use the plural form.**

### 1.7 The `last` keyword

`last` refers to the most recent result in the session. It is the closest thing ROY has to a default variable.

```text
find hotspots
| top 5
read symbol last.first
explain last.first
```

`last` is typed. `last` after `find files` is a `FileSet`. After `read symbol`, it's a `SymbolReadout`. The agent never needs to remember what `last` means structurally — `show ref last` will tell you (§7).

---

## 2. The verbs

There are **eight** verbs. This is the entire action surface. New capabilities add nouns or refiners, not verbs.

| Verb       | Question it answers                           | Mutates?                |
| ---------- | --------------------------------------------- | ----------------------- |
| `find`     | What exists matching this?                    | No                      |
| `show`     | What is the structural state of X?            | No                      |
| `read`     | What does X contain?                          | No                      |
| `trace`    | What is connected to X, and how?              | No                      |
| `explain`  | Why does X look this way / what does it mean? | No                      |
| `change`   | Modify X in this way.                         | **Yes**                 |
| `validate` | Does X hold up against this expectation?      | No (but can spawn work) |
| `review`   | Inspect a past action, denial, or artifact.   | No                      |

Three things to notice:

1. **Seven of eight verbs are read-only.** Mutation is one verb, deliberately set apart, and `change` is the only verb that flows through the approval pipeline (§9).
2. **`find`, `show`, `read`, `trace` are a spectrum of zoom.** `find` discovers what exists. `show` summarizes structure. `read` returns content. `trace` follows relationships. An agent learning ROY should learn that order: discover → structure → content → relations.
3. **`explain` is special.** It's the one verb whose output is natural-language commentary rather than structured data. It exists because both humans and agents sometimes need "why" not "what."

### 2.1 Verb–noun matrix

Not every verb applies to every noun. The matrix is small enough to memorize:

```text
             find  show  read  trace  explain  change  validate  review
files         ✓     ✓     ✓     -      ✓        ✓       ✓         -
symbols       ✓     ✓     ✓     ✓      ✓        ✓       ✓         -
modules       ✓     ✓     -     ✓      ✓        -       ✓         -
hotspots      ✓     ✓     -     ✓      ✓        -       -         -
roles         ✓     ✓     -     -      ✓        -       -         -
surfaces      ✓     ✓     -     -      ✓        -       -         -
deps          -     ✓     -     ✓      ✓        -       -         -
diffs         -     ✓     ✓     -      ✓        -       ✓         ✓
artifacts     ✓     ✓     ✓     -      ✓        -       -         ✓
issues        ✓     ✓     ✓     -      ✓        ✓       -         -
denials       ✓     ✓     ✓     -      ✓        -       -         ✓
sessions      -     ✓     -     -      ✓        -       -         ✓
schemas       ✓     ✓     ✓     -      ✓        -       -         ✓
```

If a verb–noun pair is `-`, ROY says so explicitly: `cannot 'read' a 'module' — use 'read file' for content or 'show module' for structure`.

---

## 3. The nouns (the ontology, concretely)

Nouns are the canonical object classes. Each one has a schema (what fields it has, what filters apply) and a set of projections (what it can be turned into for display or refinement).

This section is the **complete** noun set at v0.2 altitude. New nouns will be rare. New filters and projections on existing nouns will be common.

### 3.1 `files`

A file in the workspace.

**Filters:** `lang`, `path`, `role`, `surface`, `quality`, `coupling`, `behavior`, `in:<dir>`, `lines`, `fan_in`, `fan_out`, `changed`, `staged`.

**Projections:** `names`, `paths`, `summaries`, `roles`, `surfaces`, `badges`, `lines`, `imports`, `dependents`, `symbols`.

**Backed by:** filesystem walk + SEMMAP entries. Available today.

### 3.2 `symbols`

A named code entity: function, type, method, constant, module-level static.

**Filters:** `name`, `kind` (`fn`/`type`/`method`/`const`/`static`/`field`), `lang`, `in:<file-or-dir>`, `visibility` (`pub`/`crate`/`private`), `role`, `behavior`, `fan_in`, `fan_out`, `complexity`.

**Projections:** `names`, `qualified_names`, `signatures`, `bodies`, `spans`, `callers`, `callees`, `docs`.

**Backed by:** omni-ast `functions/`, `types/`, `statics/`, `fields/`. Available today for Rust + the languages omni-ast supports; degrades to "name + file + line" for unsupported languages.

### 3.3 `modules`

A coherent directory or package. ROY infers module boundaries from the workspace structure plus SEMMAP layers.

**Filters:** `name`, `path`, `layer` (matches SEMMAP's Layer 0/1/2/3), `contains:<symbol>`.

**Projections:** `names`, `paths`, `files`, `entrypoints`, `roles`, `surfaces`, `summary`.

**Backed by:** SEMMAP layer assignments + directory structure. Available today.

### 3.4 `hotspots`

Files or symbols with high structural centrality. ROY computes these from the dependency graph plus SEMMAP's `[HOTSPOT]` and `[GLOBAL-UTIL]` markers.

**Filters:** `kind:file|symbol`, `metric:fan_in|fan_out|both`, `min`, `in:<dir>`.

**Projections:** `names`, `paths`, `metrics`, `summary`.

**Backed by:** SEMMAP DependencyGraph. Available today.

### 3.5 `roles`

Semantic role badges: `controller`, `model`, `view`, `rendering`, `dialog`, `config`, `os-integration`, `utility`, `bootstrap`, `build-only`. (The exact set comes from omni-ast `taxonomy_role_inference.rs` and may grow.)

**Filters:** `name`.

**Projections:** `names`, `definitions`, `members` (files playing this role).

**Backed by:** omni-ast taxonomy. Available today.

### 3.6 `surfaces`

External-coupling badges: `filesystem`, `http-handler`, `database`, `external-api`, `template`, `clipboard`, `shell`, etc. (Set comes from `taxonomy_surface.rs` and `taxonomy_function_badges.rs`.)

**Filters:** `name`.

**Projections:** `names`, `definitions`, `members`.

**Backed by:** omni-ast taxonomy. Available today.

### 3.7 `deps`

A dependency relation set.

**Filters:** `from:<file-or-symbol>`, `to:<file-or-symbol>`, `direction:imports|imported_by|both`, `depth:N`, `kind:import|call|field-access`.

**Projections:** `pairs`, `summary`, `graph` (an artifact, §3.9).

**Backed by:** SEMMAP DependencyGraph + omni-ast `calls/`, `fields/`. Available today.

### 3.8 `diffs`

A change set. Either staged, unstaged, or recorded as an artifact.

**Filters:** `scope:staged|unstaged|workspace|since:<ref>`, `path`, `lang`.

**Projections:** `summary`, `files`, `hunks`, `text`.

**Backed by:** Git, plus ROY's own session ledger. Available today via Git.

### 3.9 `artifacts`

Stable, inspectable outputs of work. ROY's session/artifacts.rs already defines the categories: `denied_command`, `validation_run`, `diff`, semantic map, etc. Artifacts are **not transcript text** — they are addressable.

**Filters:** `kind`, `since`, `for_issue`, `actor`.

**Projections:** `summary`, `body`, `metadata`.

**Backed by:** session ledger. Available today.

### 3.10 `issues`

Tracked work items. If the workspace has Ishoo or another tracker, ROY proxies through it. Otherwise, ROY maintains a minimal local issue store.

**Filters:** `state:backlog|active|done`, `id`, `tag`, `assignee`, `since`.

**Projections:** `ids`, `titles`, `bodies`, `summary`.

**Backed by:** Ishoo (when present) or local store.

### 3.11 `denials`

A first-class noun. Every denied command is recorded and inspectable. This is what makes denial educational instead of arbitrary.

**Filters:** `since`, `command`, `reason`, `actor`.

**Projections:** `summary`, `original_command`, `redirect`, `policy_rule`.

**Backed by:** session ledger. Available today.

### 3.12 `sessions`

The session itself, as a noun. You can `show session`, `review session since:1h`, `explain session`.

**Filters:** `id`, `since`, `actor`.

**Projections:** `summary`, `events`, `artifacts`, `denials`.

**Backed by:** session ledger.

### 3.13 `schemas`

Machine-readable schema contracts for nouns and projections. Schemas are meta-objects: they describe ROY's output surface rather than workspace content.

**Filters:** `noun`, `projection`, `version`, `major`, `kind:noun|projection`.

**Projections:** `summary`, `fields`, `versions`, `compatibility`, `examples`.

**Backed by:** ROY's internal schema registry. Always available.

---

## 4. Refiners (the precision layer)

Refiners are what saves ROY from being mushy. They give the precision Bash gives without surrendering structure.

A refiner runs after `|`. It always takes a typed input and produces a typed output. It never reaches outside the value flowing through the pipe.

### 4.1 The complete refiner set

Grouped by what they do. This is the full v0.2 list.

**Slicing**

| Refiner      | Input    | Output              | Example |              |
| ------------ | -------- | ------------------- | ------- | ------------ |
| `top N`      | any list | same, length ≤ N    | `       | top 5`       |
| `last N`     | any list | same, length ≤ N    | `       | last 3`      |
| `nth N`      | any list | one item            | `       | nth 0`       |
| `first`      | any list | one item            | `       | first`       |
| `skip N`     | any list | same, drops first N | `       | skip 10`     |
| `range A..B` | any list | sublist [A, B)      | `       | range 5..10` |

**Filtering**

| Refiner                | Input                                  | Output         | Example |                 |
| ---------------------- | -------------------------------------- | -------------- | ------- | --------------- |
| `where <key><op><val>` | any list                               | same, filtered | `       | where fan_in>5` |
| `matching <pattern>`   | `StringList`, `FileBody`, `SymbolBody` | same, filtered | `       | matching auth*` |
| `unique`               | any list                               | deduplicated   | `       | unique`         |
| `unique by <key>`      | any list                               | dedup by field | `       | unique by name` |

`matching` is the only refiner that operates on raw text content. It exists for the cases where you've already used semantic selection to narrow to a small set and now need exact textual filtering inside it (e.g., "of the five files about auth, which lines mention `bcrypt`"). The type system enforces this: `matching` is rejected on `FileSet`, `SymbolSet`, and other structural types. To filter a list of files by content, you must first read them — and the cost of doing so is what makes `matching` self-limiting.

**`matching` is labeled as a fallback refiner in `help refiners`.** Help output describes it as "exact-text filter for already-read content; for discovery, use `find files about <topic>`." This is documentation, not enforcement — the type restriction does the real work.

**Sorting**

| Refiner              | Input    | Output           | Example |                      |
| -------------------- | -------- | ---------------- | ------- | -------------------- |
| `sort`               | any list | sorted ascending | `       | sort`                |
| `sort by <key>`      | any list | sorted by field  | `       | sort by fan_in`      |
| `sort by <key> desc` | any list | reverse          | `       | sort by fan_in desc` |
| `reverse`            | any list | reversed         | `       | reverse`             |

**Counting / aggregating**

| Refiner          | Input    | Output         | Example |                |
| ---------------- | -------- | -------------- | ------- | -------------- |
| `count`          | any list | integer        | `       | count`         |
| `count by <key>` | any list | `KeyCountList` | `       | count by role` |
| `group by <key>` | any list | `GroupedList`  | `       | group by lang` |

**Projecting** (typed reshaping)

| Refiner             | Input       | Output         | Example |           |           |        |
| ------------------- | ----------- | -------------- | ------- | --------- | --------- | ------ |
| `<projection-name>` | typed value | the projection | `       | names`, ` | paths`, ` | roles` |
| `as <projection>`   | typed value | the projection | `       | as paths` |           |        |

The available projections come from the noun's schema (§3). This is how an agent gets things like "just the file names" or "just the spans."

**String-level (last-resort, only on `StringList`)**

| Refiner         | Input            | Output           | Example |                  |
| --------------- | ---------------- | ---------------- | ------- | ---------------- |
| `chars A..B`    | `StringList`     | `StringList`     | `       | chars 1..3`      |
| `prefix N`      | `StringList`     | `StringList`     | `       | prefix 3`        |
| `suffix N`      | `StringList`     | `StringList`     | `       | suffix 3`        |
| `contains <s>`  | `StringList`     | `StringList`     | `       | contains "auth"` |
| `split on <s>`  | `StringList`     | `StringListList` | `       | split on /`      |
| `join with <s>` | `StringListList` | `StringList`     | `       | join with /`     |

These exist because sometimes you really do need "first 3 chars." But notice they only operate on `StringList` — you can't apply them to a `FileSet` or `SymbolReadout` without first projecting to strings (`| names | prefix 3`). This is intentional. It keeps string-fiddling as a final step, not a substrate.

### 4.2 What's deliberately not a refiner

* **No `map` with arbitrary code.** No lambdas, no inline scripts. If you need a transformation that isn't a built-in refiner, you build a new refiner; you don't write code in a pipe.
* **No `exec`.** You cannot pipe a `FileSet` into "run this command on each one." Iteration is something the agent loop does at the prompt level, not something the shell composes silently.
* **No raw text grep.** `where matches:<regex>` is intentionally not in v0.2. If it goes in later, it should be on `bodies` projections only and clearly marked as a fallback.

These omissions are the line that prevents collapse into Bash.

### 4.3 The one composition rule

Refiners chain left-to-right. Each refiner's output type determines what refiners are valid next. ROY validates this at parse time:

```text
find files about auth | top 5 | names | prefix 3
                       ^FileSet ^FileSet ^StringList ^StringList   ✓

find files about auth | prefix 3
                       ^FileSet — `prefix` requires StringList
                       error: 'prefix' cannot follow a FileSet; project to strings first (try '| names')
```

This is the type system doing the work the user would otherwise have to do in their head with Bash.

---

## 5. Verb behavior in detail

### 5.1 `find <noun> [filters]`

Returns a typed list of the noun. Always read-only. Always cheap (or marked as expensive — see §10).

```text
find files
find files lang:rust
find files about auth
find symbols name:dispatch*
find hotspots in:src/shell
find issues state:active
find denials since:1h
find schemas noun:files
```

`about <topic>` is a special filter that does semantic matching. Because it's the primary replacement for `grep -R foo`, its behavior is specified, not left to implementer taste. See §5.1.1.

#### 5.1.1 The semantics of `about <topic>`

`about` is a scored retrieval, not a boolean match. Every result has an internal relevance score in `[0, 1]`. The default rendering hides scores; `--json` exposes them as the `score` field on each result.

**Signal sources, in priority order:**

1. **SEMMAP file descriptions** — the prose summary of what each file does. This is the primary signal because it's deliberately written for relevance, not extracted heuristically.
2. **Symbol documentation** — doc comments on functions, types, modules, when omni-ast can extract them for the language. Augments the file-level signal; cannot override it alone.
3. **Symbol names and identifiers** — split via SWUM so `dispatchCommand` matches "dispatch." A weaker signal; useful for breaking ties.
4. **Lexical match in file content** — exact occurrences of the topic terms. This is a **boost**, not a primary signal. A file that lexically contains "auth" 200 times but has no semantic relationship to authentication ranks below a file whose SEMMAP description says "handles user authentication."

**Scoring rules:**

* Each signal contributes additively, weighted by source priority.
* Lexical-only matches (signal 4 alone) are capped below the threshold for inclusion when stronger signals exist for other files. They surface only when no semantic signal matches.
* Results below a relevance floor are dropped; the floor is calibrated so that returning *nothing* is a valid and meaningful answer ("no files in this workspace are about that topic").
* Tie-breaking uses fan-in (more central files first), then path (stable order).

**What this means for the agent:**

* "Why did this file show up?" is always answerable. `find files about auth --json` returns scores; `explain file <path>` includes "matched topic 'auth' via SEMMAP description and 3 doc comments" when relevant.
* "About" is *not* substring search. `find files about config` will not return every file containing the word "config" — it returns files whose described purpose involves configuration.
* Empty results are real. If no file is semantically about the topic, the answer is empty, not "here are 47 files that mention the word."

**What it depends on:**

`about` is most accurate when SEMMAP descriptions are well-written and current. The quality of `about` is therefore the quality of the workspace's semantic map. ROY exposes this dependency: `show session` includes a SEMMAP freshness indicator, and `explain about` (a help target, §6) describes the current signal availability.

### 5.2 `show <noun> [target]`

Returns a structured summary, not full content. The summary's fields come from the noun's schema.

```text
show file src/shell/runtime.rs
show module src/policy
show hotspots
show diff staged
show session
show denials since:1h
show last
show schemas
show schema files
```

`show` is what an agent runs to orient before deciding to `read`. It's the SEMMAP-style "look at the map" verb.

### 5.3 `read <noun> <target> [span]`

Returns full content. This is the verb that replaces `cat`.

```text
read file src/shell/runtime.rs
read file src/shell/runtime.rs lines 1..40
read symbol dispatch
read symbol shell::runtime::ShellRuntime::dispatch
read artifact denial-2026-04-11-3
read issue ISS-04
read diff staged
read schema files
```

`read symbol` is more useful than `read file` for most agent work — it returns just the symbol's body plus its signature and doc, not the surrounding 800 lines.

### 5.4 `trace <relation> of <target> [depth N]`

Walks structural relationships. The relation is the first argument because that's what makes the command read naturally.

```text
trace callers of resolve_command
trace callees of dispatch depth 2
trace dependents of src/shell/runtime.rs
trace imports of src/shell/runtime.rs depth 1
trace fields of ShellRuntime
```

Default depth is 1. Depth >1 produces a tree-shaped result, projectable to a flat list with `| as flat`.

### 5.5 `explain <noun> <target>`

The natural-language verb. Uses SEMMAP description + omni-ast badges + dependency context to write a short prose explanation.

```text
explain file src/shell/runtime.rs
explain module src/policy
explain hotspot src/shell/env.rs
explain denial last
explain symbol dispatch
explain diff staged
explain schema files
```

This is the only verb whose output is intentionally not structured. It's for the moments when a human or agent needs to understand, not consume.

### 5.6 `change <noun> <target> <change-spec>` *(mutating)*

The single mutation verb. Always goes through the policy engine. Always produces a `diff` artifact. Always reviewable.

```text
change file src/shell/runtime.rs apply patch:<artifact-ref>
change symbol dispatch rename to dispatch_command
change file src/shell/io.rs replace lines 10..20 with:<artifact-ref>
change issue ISS-04 state:done
```

Notice the `with:<artifact-ref>` and `patch:<artifact-ref>` patterns. ROY does not accept multi-line text inline in commands. Mutations operate on artifacts that were produced earlier (by the agent staging a patch, by `change ... draft`, etc.). This is what makes mutations reviewable — there's always a referenceable thing being applied.

`change ... draft` is a non-mutating variant that produces a diff artifact without applying it:

```text
change file src/shell/runtime.rs apply patch:<artifact-ref> draft
# → produces artifact diff-2026-04-11-7, doesn't apply
```

#### 5.6.1 The closed change-spec set

`<change-spec>` is **not** open-ended. It is a closed set of operation types, each with a fixed signature. Adding a new operation type is a deliberate spec revision. This is what prevents `change` from sprawling into a second command language.

The v0.2 operation set, by noun:

**On `file`:**

| Operation                       | Signature                     | Notes                                              |
| ------------------------------- | ----------------------------- | -------------------------------------------------- |
| `apply patch:<ref>`             | A patch artifact              | The general escape hatch; patch must apply cleanly |
| `replace lines A..B with:<ref>` | Line range + content artifact | Bounded textual replacement                        |
| `move to <path>`                | New path                      | Workspace-bounded                                  |
| `copy to <path>`                | New path                      | Workspace-bounded                                  |
| `delete`                        | (none)                        | Always requires approval                           |
| `create with:<ref>`             | Content artifact              | New file at the target path                        |

**On `symbol`:**

| Operation           | Signature                     | Notes                                        |
| ------------------- | ----------------------------- | -------------------------------------------- |
| `rename to <name>`  | New name                      | Updates symbol + all references in workspace |
| `move to <module>`  | Target module                 | Updates references; may require approval     |
| `apply patch:<ref>` | Patch scoped to symbol's span |                                              |

**On `issue`:**

| Operation         | Signature        | Notes                                 |
| ----------------- | ---------------- | ------------------------------------- |
| `state:<value>`   | New state        | From the issue tracker's known states |
| `tag <name>`      | Tag name         |                                       |
| `untag <name>`    | Tag name         |                                       |
| `assign <actor>`  | Actor identifier |                                       |
| `unassign`        | (none)           |                                       |
| `note with:<ref>` | Content artifact | Append a note                         |

That's it for v0.2. Six file operations, three symbol operations, six issue operations. If a workflow needs an operation outside this set, the right answer is to add it deliberately to this list, not to slip free-form change specs in through the cracks.

The agent-facing rule: **every `change` command's third position must be one of these operation tokens, or the command is rejected at parse time with a list of valid operations for the noun.** No regex, no inline scripts, no shell substitution.

### 5.7 `validate <noun> [scope]`

Runs the workspace's validation pipeline (`cargo fmt/clippy/test/build` for Rust, equivalents for other languages) scoped to the target.

```text
validate changed
validate file src/shell/runtime.rs
validate module src/policy
validate workspace
```

Always produces a `validation_run` artifact. Failures are inspectable: `read artifact last`.

### 5.8 `review <noun> <target>`

Inspect history. The "what happened and why" verb.

```text
review denial last
review denials since:1h
review session since:1h
review artifact diff-2026-04-11-7
review diff staged
review schema files
```

`review` differs from `show` in that it's oriented to past actions rather than current state. `show denials since:1h` and `review denials since:1h` return similar data but `review` is what an agent should reach for when the question is "what did I just do."

---

## 6. Discovery and help

The shell must be discoverable from cold. An agent running `help` should be working productively within one round-trip.

### 6.1 `help` (no arguments)

Shows: the eight verbs, the noun list, and three example commands. Always under 40 lines.

```text
ROY shell — semantic command surface

VERBS    find  show  read  trace  explain  change  validate  review
NOUNS    files  symbols  modules  hotspots  roles  surfaces  deps
         diffs  artifacts  issues  denials  sessions  schemas

Use singular nouns for known targets (`read file path.rs`)
and plural nouns for discovery (`find files lang:rust`).

Try:
  show hotspots
  find files about <topic>
  read symbol <name>

For a verb:    help <verb>
For a noun:    help <noun>
For refiners:  help refiners
For denials:   help denials
```

### 6.2 `help <verb>`

Shows the verb's signature, the nouns it accepts, and 2–3 examples per noun.

### 6.3 `help <noun>`

Shows the noun's filters, projections, and example commands.

### 6.4 `help refiners`

Shows the full refiner table from §4.

### 6.5 `help denials`

Shows the redirect table from §8: "if you tried to do X, do Y instead." This is the single most important help page for agents trained on Bash.

### 6.6 `?` suffix

Appending `?` to any partial command returns context-sensitive help without executing.

```text
find files ?
# → lists filters available for 'files'
find files | ?
# → lists refiners valid on a FileSet
```

This is how an agent explores without committing. It's also how a human stumbles into competence.

---

## 7. Named refs (the "variable" replacement)

ROY has no shell variables. It has named refs, which are typed and persistent within a session.

```text
find hotspots | top 5 | save as topfive
read symbol last
read file topfive.first
trace callers of topfive.first
```

### 7.1 Operations on refs

```text
save as <name>      # name the current 'last' result
list refs           # show all named refs in this session
show ref <name>     # inspect a ref (type + summary)
drop ref <name>     # delete a ref
```

### 7.2 Why this instead of variables

Variables in Bash are untyped strings. They lose information across pipe stages and can be substituted into arbitrary positions, which means policy can't reason about them. Named refs are typed values that flow through the same pipe machinery. Policy can see what's in them; ROY can display them; the agent can inspect them.

### 7.3 Lifetime

Named refs live for the duration of the session. They are not persisted across sessions unless the actor explicitly promotes one to an artifact:

```text
save as topfive
promote topfive to artifact
```

---

## 8. The denial protocol

This is the part that justifies ROY's existence. The protocol has four parts.

### 8.1 Every denial is structured

A denial is not "command not found." It's an artifact with five fields:

| Field         | Example                                                                 |
| ------------- | ----------------------------------------------------------------------- |
| `original`    | `grep -R "auth" src`                                                    |
| `reason`      | `text-search via grep is not the primary navigation path in ROY`        |
| `redirect`    | `find files about auth`                                                 |
| `policy_rule` | `policy.shell.no-text-search`                                           |
| `agent_hint`  | `ROY semantic search ranks files by SEMMAP relevance, not match count.` |

The denial is dispatched as the response, and recorded as a `denial` artifact (§3.11). The agent can `review denial last` to see the full structure.

### 8.2 The standard redirect table

This is the v0.2 redirect table. Every entry has a deliberate ROY-native equivalent. Adding to this table is the right way to handle "the agent reaches for X."

| Bash habit                | ROY redirect                                     | Reason                                         |                                                 |
| ------------------------- | ------------------------------------------------ | ---------------------------------------------- | ----------------------------------------------- |
| `grep -R <text> <path>`   | `find files about <text>`                        | Semantic relevance, not occurrence             |                                                 |
| `grep -l <text>`          | `find files about <text>                         | names`                                         | Same, projected to names                        |
| `grep <text> <file>`      | `read file <file>                                | matching <text>`                               | Filtering on read content                       |
| `find <path> -name <pat>` | `find files path:<pat> in:<path>`                | Typed filter                                   |                                                 |
| `find <path> -type f`     | `find files in:<path>`                           | `files` already excludes dirs                  |                                                 |
| `ls <path>`               | `find files in:<path>                            | names`                                         | Or `show module <path>`                         |
| `ls -la`                  | `find files                                      | as paths`                                      | Detailed projection is explicit, not flag-based |
| `cat <file>`              | `read file <file>`                               | Same intent, structured output                 |                                                 |
| `head -n N <file>`        | `read file <file> lines 1..N`                    | Explicit span                                  |                                                 |
| `tail -n N <file>`        | `read file <file> lines -N..-1`                  | Negative indexing                              |                                                 |
| `wc -l <file>`            | `show file <file>`                               | Line count is in summary                       |                                                 |
| `awk` / `cut` / `sed`     | Refiners (§4)                                    | Structured projection beats text munging       |                                                 |
| `xargs <cmd>`             | No equivalent — agent loops at prompt level      | Iteration is supervised                        |                                                 |
| `which <cmd>`             | `help <verb>` or `find symbols name:<cmd>`       | Different question                             |                                                 |
| `find ... -exec ...`      | Denied; no implicit fan-out                      | Per-item action requires explicit confirmation |                                                 |
| `cd <path>`               | `cd <path>` (allowed; bounded by workspace)      | Genuinely useful, kept                         |                                                 |
| `pwd`                     | `pwd`                                            | Same                                           |                                                 |
| `mv` / `cp` / `rm`        | `change file <path> move/copy/delete to <path>`  | Routed through change pipeline                 |                                                 |
| `git diff`                | `show diff staged` / `read diff staged`          | Native                                         |                                                 |
| `git status`              | `show diff workspace`                            | Native                                         |                                                 |
| `git log`                 | `review session` or `review artifacts since:<t>` | Different model                                |                                                 |

### 8.3 Denial is loud, not silent

When a command is denied, ROY prints all five fields. The redirect is rendered as a literal command the agent can copy. The `policy_rule` is a stable identifier so denials are debuggable.

```text
> grep -R auth src
✗ DENIED  grep -R auth src
  reason   text-search via grep is not the primary navigation path in ROY
  try      find files about auth
  rule     policy.shell.no-text-search
  hint     ROY semantic search ranks files by SEMMAP relevance, not match count.
  see      review denial last
```

### 8.4 Some Bash commands pass through unchanged

Not everything Bash does is something ROY needs to replace. Things that are genuinely cheap, scoped, and not bypasses:

* `cd`, `pwd`, `clear`
* `echo` (within reason, no command substitution)
* `exit`

These are available because forbidding them would be theater, not safety.

---

## 9. The mutation pipeline

Every mutation goes through one path. This is the only way to write to disk.

```text
1. PROPOSE      change <noun> <target> <spec> draft
                → produces a diff artifact, no on-disk change

2. INSPECT      review artifact <diff-id>
                read artifact <diff-id>

3. APPLY        change <noun> <target> apply patch:<diff-id>
                → mediated by policy; may auto-approve, may require human

4. VALIDATE     validate changed
                → produces a validation_run artifact

5. RECORD       (automatic) the session ledger gets:
                  - the original change spec
                  - the diff artifact id
                  - the apply event
                  - the validation run id
                  - any denials encountered
```

The agent can fuse 1–3 into a single command (`change ... apply`) but ROY internally still goes through these stages. This means review is always possible even when the actor was in a hurry.

### 9.1 Approval gates

The policy engine decides whether a mutation:

* auto-applies
* requires explicit `approve <diff-id>` from a human
* is hard-denied

The approval state of a diff artifact is itself a noun-projection, so the agent can `find diffs state:awaiting-approval`.

---

## 10. Cost and signaling

Some commands are cheap (`show file`); some are expensive (`find symbols lang:rust` across the full workspace). ROY signals cost so the agent doesn't accidentally fire a 30-second query.

### 10.1 Cost classes

| Class       | Latency budget | Examples                                            |
| ----------- | -------------- | --------------------------------------------------- |
| `instant`   | <50ms          | `pwd`, `help`, `show file`, `read symbol` (cached)  |
| `quick`     | <500ms         | `find files`, `show hotspots`, `read file`          |
| `slow`      | <5s            | `find files about <topic>`, `trace callers depth:3` |
| `expensive` | >5s            | `validate workspace`, full-graph traces             |

### 10.2 The `--dry` flag

`--dry` is one of the three allowed flags in ROY. It makes any command return its cost class and projected result count without running.

```text
> find symbols lang:rust --dry
plan       find symbols lang:rust
class      slow
estimate   ~12,000 symbols across 480 files
suggestion add a filter: in:<dir>, name:<pattern>, kind:<kind>
```

This is what an agent should do before any `find` it isn't sure about. It's also what a human types to learn what a command would do.

---

## 11. What renders, and how

ROY's output is structured, but it has to display somewhere. Three rendering modes:

### 11.1 Default: human-formatted

Tables, summaries, color. Optimized for the human in the cockpit. Truncates long lists and indicates how many more results exist, but does not introduce extra flags beyond the core three.

### 11.2 `--json`

Returns the typed structure as JSON. This is what an agent should use when it needs to post-process. JSON output is stable and versioned.

```text
find hotspots | top 5 --json
```

### 11.3 `--ref`

Returns just the ref id of the result, suitable for carrying into the next command without rendering the full structure. Used for chained agent work.

```text
find hotspots | top 5 --ref
# → ref:r-2026-04-11-12
```

### 11.4 Three flags only

`--dry`, `--json`, `--ref`. That's the entire flag set. If a fourth becomes necessary, this section is the place that has to be revised deliberately.

### 11.5 Schema versioning

Every noun and projection has a versioned machine schema. `--json` and `--ref` outputs include a `_schema` envelope identifying what the consumer is reading. Without this, agent tooling will eventually depend on accidental render structure and break silently when a noun gains a field.

**Envelope shape:**

```json
{
  "_schema": { "noun": "files", "projection": "default", "version": "1.2" },
  "results": [ ... ]
}
```

**Versioning rules:**

| Change                                                                    | Version bump                              |
| ------------------------------------------------------------------------- | ----------------------------------------- |
| New optional field added to a result                                      | minor (1.2 → 1.3)                         |
| Existing field's meaning unchanged, additional field populated more often | minor                                     |
| Field removed, renamed, or semantics changed                              | major (1.x → 2.0)                         |
| New noun added                                                            | new schema family, starts at 1.0          |
| New projection added to existing noun                                     | new schema under same noun, starts at 1.0 |

Because `schema/schemas` are explicit nouns (§3.13), introspection uses the normal surface:

* `show schemas` lists every noun/projection pair and its current version
* `show schema files` returns the summary for the `files` schema
* `read schema files` returns the full schema document

**Stability commitment:** within a major version, fields only get added, never removed or renamed. Within a minor version, no field is added that an existing consumer would need to handle to be correct. This is the contract that makes ROY scriptable from outside without becoming brittle.

**The default human render is explicitly not versioned.** It can change freely between ROY versions because no machine consumer should be parsing it. If you're parsing default output, you have made a mistake; switch to `--json`.

---

## 12. Worked examples

These are the workflows the design must make easy. If any of them are awkward, the design is wrong.

### 12.1 Onboarding to an unknown codebase (the SEMMAP loop)

```text
show hotspots
explain module .
find files about <task-keyword> | top 10
read file <chosen-file>
trace dependents of <chosen-file>
```

Five commands. An agent should be able to orient in any codebase in this many steps.

### 12.2 Finding where to make a change

```text
find files about "rate limiting" | top 10 | paths
read file last.first
find symbols name:*rate* in:src
read symbol RateLimiter
trace callers of RateLimiter
```

### 12.3 Making a change

```text
read symbol dispatch
change symbol dispatch rename to dispatch_command draft
review artifact last
change symbol dispatch apply patch:last
validate changed
```

### 12.4 Recovering from a denial

```text
> grep error src
✗ DENIED  ...redirect: find files about error
> find files about error | top 5
> read file <chosen> | matching error
```

### 12.5 Inspecting a session

```text
review session since:1h
find denials since:1h
explain denial last
```

---

## 13. What's intentionally not in v0.2

Listed so they don't get smuggled in by accident.

* **No scripting language.** No `roy script`, no batched files of ROY commands. If you want to script, you do it from outside ROY (Python or Rust calling the ROY library) — or you wait for a future spec that adds it deliberately.
* **No background tasks.** Long work is not part of v0.2. The `task/tasks` noun family is reserved for a future version (§13.1), and backgrounding via `&` is intentionally absent.
* **No environment variables.** Configuration is loaded from a `.royrc` (TBD), not from per-command env.
* **No aliases.** Aliases create dialect; dialect kills the "learnable in one sitting" property. If a command is so long that it needs an alias, the underlying command is wrong.
* **No plugins yet.** Extensibility happens by adding nouns/refiners to ROY itself, reviewed deliberately. Plugins are a v2 concern.
* **No multi-workspace.** One workspace per session. Cross-workspace is a future capability.

### 13.1 Reserved future nouns

The following nouns are reserved for future versions and are **not valid in v0.2**:

* `task`
* `tasks`

These names are reserved for eventual long-running, background, or asynchronous work units. They are referenced in this document only as future design placeholders. Any command using `task` or `tasks` in v0.2 must fail with:

```text
'task' is a reserved future noun and is not available in v0.2
```

---

## 14. How this maps onto what ROY already has

Nothing in this spec asks for greenfield modules. Every piece lands somewhere in the existing ROY tree.

| Spec section           | ROY module                                                                              |
| ---------------------- | --------------------------------------------------------------------------------------- |
| §1 grammar, parsing    | `src/ui/layout/panels/command_line.rs`, `src/shell/resolve.rs`                          |
| §2 verbs               | `src/commands/registry.rs`, `src/commands/registry_data/builtins.rs`                    |
| §3 nouns + projections | New: `src/nouns/` (mod per noun); backed by omni-ast adapters                           |
| §4 refiners            | New: `src/refiners/`; types live in `src/commands/schema.rs`                            |
| §5 verb dispatch       | `src/shell/runtime.rs`, `src/capabilities/`                                             |
| §7 named refs          | `src/session/engine.rs` (extend with ref table)                                         |
| §8 denial protocol     | `src/policy/engine.rs`, `src/session/artifacts.rs` (`denied_command` is already a kind) |
| §9 mutation pipeline   | `src/capabilities/validation.rs`, `src/policy/engine.rs`, `src/session/artifacts.rs`    |
| §10 cost classes       | New: `cost: CostClass` field on `CommandSchema`                                         |
| §11 rendering          | `src/ui/layout/panels/terminal.rs`, plus `--json` / `--ref` formatters                  |

The biggest piece of new code is `src/nouns/` — one module per noun (§3), each one wrapping its omni-ast or filesystem source. That's the substrate everything else sits on.

---

## 15. The single rule that decides all future questions

When something new is proposed (new verb, new refiner, new flag, new noun), ask:

> Does this give the actor more reach inside the ROY world, or does it recreate an escape hatch out of it?

Keep things that narrow selection, refine projections, slice structured output, or expose a meaningful object that didn't have a name.

Reject things that recreate arbitrary universality, let the actor tunnel under the semantic layer, or turn structured output back into raw text streams too early.

That rule, more than any specific section above, is what keeps ROY from drifting into "Bash but custom."

---

*End of v0.2. This is the complete surface at altitude. Filling in implementation is iterative; the surface is fixed until deliberately revised.*
