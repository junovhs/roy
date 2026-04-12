# Mutation Survivors

Focused run used while tightening UI/session coverage:

```bash
cargo mutants \
  -f src/ui/layout/mod.rs \
  -f src/ui/layout/panels/terminal_model.rs \
  -f src/shell/runtime.rs \
  -f src/commands/registry.rs \
  --baseline skip
```

After the added tests in this branch, that run dropped from the earlier full-run survivor set on these files down to a small residual set.

## Equivalent survivors

- `src/shell/runtime.rs: ShellRuntime::registry`
  Returning `&self.registry` versus a freshly-defaulted `CommandRegistry` is behaviorally equivalent in the current design because `CommandRegistry` is stateless and fully derives its data from static tables.

- `src/commands/registry.rs: CommandRegistry::is_empty`
  Forcing `is_empty` to `false` is equivalent under the current registry contract because the command table is statically non-empty and the codebase does not support an empty registry configuration.

## Low-signal survivors

These are not currently treated as blockers because they sit in UI-only state toggles or initialization paths where the behavior is already covered at the intended contract boundary:

- drawer-open equality checks in `src/ui/layout/mod.rs`
- some `Cockpit` initialization timestamp arithmetic in `src/ui/layout/mod.rs`

If these become operationally important, prefer asserting the user-visible state transition rather than chasing the mutation mechanically.
