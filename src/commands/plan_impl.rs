//! [`Planner::plan`] implementation and private noun-profile helpers.

use super::*;

impl<'r> Planner<'r> {
    /// Resolve a parsed [`Command`] into an executable [`Plan`].
    ///
    /// Returns [`PlanError`] when:
    /// - the verb is not in the registry,
    /// - the verb's backend is denied (CompatTrap / Blocked),
    /// - a required target argument is missing, or
    /// - a refiner is type-incompatible with the command's result type.
    pub fn plan(&self, cmd: &Command) -> Result<Plan, PlanError> {
        let verb = &cmd.verb.value;

        // 1. Registry lookup — unknown verbs fail here.
        let schema = self
            .registry
            .resolve(verb)
            .ok_or_else(|| PlanError::UnknownVerb { verb: verb.clone() })?;

        // 2. Denied backends — the plan layer rejects them immediately so the
        //    caller never needs to check the backend for executability.
        if schema.backend.is_denied() {
            let reason = schema
                .backend
                .suggestion()
                .unwrap_or("blocked by ROY policy")
                .to_string();
            return Err(PlanError::VerbDenied {
                verb: verb.clone(),
                reason,
            });
        }

        // 3. Noun profile — cardinality, result type, and mutation class.
        let (noun, cardinality, result_type, mutation_class) = noun_profile(verb);

        // 4. Target validation — some verbs require a positional target.
        if requires_target(verb) && cmd.target.is_none() {
            return Err(PlanError::MissingTarget { verb: verb.clone() });
        }

        // 5. Classify filters (pushdown vs post-materialisation).
        let classified_filters = cmd
            .filters
            .iter()
            .map(|f| ClassifiedFilter {
                filter: f.clone(),
                class: classify_filter(f),
            })
            .collect();

        // 6. Validate refiners against the result type (fail on first mismatch).
        for refiner in &cmd.refiners {
            if let Some(desc) = refiner_type_error(refiner, &result_type) {
                return Err(PlanError::RefinerTypeMismatch {
                    refiner_desc: desc,
                    result_type: result_type.clone(),
                });
            }
        }

        let render_mode = if cmd.flags.json {
            RenderMode::Json
        } else {
            RenderMode::Human
        };

        Ok(Plan {
            verb: verb.clone(),
            noun,
            cardinality,
            result_type,
            classified_filters,
            refiners: cmd.refiners.clone(),
            mutation_class,
            render_mode,
            dry_run: cmd.flags.dry,
            ref_name: cmd.flags.ref_name.clone(),
            backend: schema.backend,
            risk_level: schema.risk_level,
        })
    }
}

// ── helpers ───────────────────────────────────────────────────────────────────

/// Returns `(NounKind, Cardinality, ResultType, MutationClass)` for an allowed,
/// non-denied verb. Called only after registry validation.
fn noun_profile(verb: &str) -> (NounKind, Cardinality, ResultType, MutationClass) {
    use Cardinality::{Plural, Singular};
    use MutationClass::{Mutating, ReadOnly};
    use NounKind::*;
    use ResultType::*;

    match verb {
        "claude" => (AgentTarget, Singular, AgentSession, Mutating),
        "cd" => (DirPath, Singular, Empty, Mutating),
        "pwd" => (DirPath, Singular, Text, ReadOnly),
        "env" | "printenv" => (NounKind::EnvMap, Plural, ResultType::EnvMap, ReadOnly),
        "exit" | "quit" => (Session, Singular, ExitStatus, Mutating),
        "help" | "roy" | "?" | "commands" => (Help, Plural, Text, ReadOnly),
        "ls" => (DirPath, Plural, FileList, ReadOnly),
        "read" => (FilePath, Singular, FileContent, ReadOnly),
        "write" => (FilePath, Singular, Empty, Mutating),
        "check" => (FileSet, Plural, Text, ReadOnly),
        "schemas" => (NounKind::SchemaSet, Plural, Text, ReadOnly),
        "schema" => (NounKind::SchemaRef, Singular, Text, ReadOnly),
        // Safe fallback for registry entries not yet in the profile table.
        _ => (Help, Singular, Text, ReadOnly),
    }
}

/// True when this verb requires a non-empty [`Command::target`] argument.
fn requires_target(verb: &str) -> bool {
    matches!(verb, "read" | "write" | "schema")
}

/// Classify a single [`Filter`] by evaluation order.
fn classify_filter(filter: &Filter) -> FilterClass {
    match filter {
        Filter::Lines(_) | Filter::KeyValue { .. } => FilterClass::SourcePushdown,
        Filter::About(_) | Filter::Bare(_) | Filter::Flag(_) => FilterClass::PostMaterialization,
    }
}

/// Returns `Some(description)` when `refiner` cannot be applied to `result_type`.
///
/// [`Refiner::SortedBy`] and [`Refiner::GroupedBy`] require a structured record
/// set; no current result type qualifies.
fn refiner_type_error(refiner: &Refiner, result_type: &ResultType) -> Option<String> {
    match refiner {
        Refiner::SortedBy(key) if !is_record_set(result_type) => Some(format!("sorted by {key}")),
        Refiner::GroupedBy(key) if !is_record_set(result_type) => Some(format!("grouped by {key}")),
        _ => None,
    }
}

/// True when `result_type` supports named-field refiners (SortedBy, GroupedBy).
///
/// Returns `false` for all current result types.  When `ResultType::RecordSet`
/// is added, this match must be updated — the compiler enforces exhaustiveness.
fn is_record_set(result_type: &ResultType) -> bool {
    match result_type {
        ResultType::FileContent
        | ResultType::FileList
        | ResultType::EnvMap
        | ResultType::Text
        | ResultType::ExitStatus
        | ResultType::AgentSession
        | ResultType::Empty => false,
    }
}
