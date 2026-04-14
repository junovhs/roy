//! Tests for the plan layer.

use super::*;
use crate::commands::ast::parse;
use crate::commands::registry::CommandRegistry;

// Helper: parse a command string and plan it in one step.
fn plan(input: &str) -> Result<Plan, PlanError> {
    let reg = CommandRegistry::new();
    let cmd = parse(input).expect("parse must succeed for planner tests");
    Planner::new(&reg).plan(&cmd)
}

// ── valid plans ───────────────────────────────────────────────────────────────

#[test]
fn pwd_produces_readonly_text() {
    let p = plan("pwd").unwrap();
    assert_eq!(p.verb, "pwd");
    assert_eq!(p.noun, NounKind::DirPath);
    assert_eq!(p.cardinality, Cardinality::Singular);
    assert_eq!(p.result_type, ResultType::Text);
    assert_eq!(p.mutation_class, MutationClass::ReadOnly);
    assert_eq!(p.render_mode, RenderMode::Human);
    assert!(!p.dry_run);
}

#[test]
fn read_with_target_is_readonly_file_content() {
    let p = plan("read src/main.rs").unwrap();
    assert_eq!(p.noun, NounKind::FilePath);
    assert_eq!(p.result_type, ResultType::FileContent);
    assert_eq!(p.mutation_class, MutationClass::ReadOnly);
    assert_eq!(p.cardinality, Cardinality::Singular);
}

#[test]
fn ls_is_plural_filelist_readonly() {
    let p = plan("ls").unwrap();
    assert_eq!(p.noun, NounKind::DirPath);
    assert_eq!(p.cardinality, Cardinality::Plural);
    assert_eq!(p.result_type, ResultType::FileList);
    assert_eq!(p.mutation_class, MutationClass::ReadOnly);
}

#[test]
fn write_is_mutating_empty() {
    let p = plan("write foo.txt hello").unwrap();
    assert_eq!(p.result_type, ResultType::Empty);
    assert_eq!(p.mutation_class, MutationClass::Mutating);
}

#[test]
fn exit_is_mutating_exit_status() {
    let p = plan("exit").unwrap();
    assert_eq!(p.result_type, ResultType::ExitStatus);
    assert_eq!(p.mutation_class, MutationClass::Mutating);
}

#[test]
fn json_flag_sets_render_mode() {
    let p = plan("pwd --json").unwrap();
    assert_eq!(p.render_mode, RenderMode::Json);
}

#[test]
fn dry_flag_sets_dry_run() {
    let p = plan("pwd --dry").unwrap();
    assert!(p.dry_run);
}

#[test]
fn ref_flag_sets_ref_name() {
    let p = plan("pwd --ref myref").unwrap();
    assert_eq!(p.ref_name, Some("myref".to_string()));
}

#[test]
fn top_refiner_valid_on_filelist() {
    let p = plan("ls | top 5").unwrap();
    assert_eq!(p.refiners, vec![Refiner::Top(5)]);
    assert_eq!(p.result_type, ResultType::FileList);
}

#[test]
fn count_refiner_valid_on_filelist() {
    let p = plan("ls | count").unwrap();
    assert_eq!(p.refiners, vec![Refiner::Count]);
}

#[test]
fn skip_refiner_valid_on_text() {
    let p = plan("pwd | skip 2").unwrap();
    assert_eq!(p.refiners, vec![Refiner::Skip(2)]);
}

// ── filter classification ─────────────────────────────────────────────────────

#[test]
fn kv_filter_is_source_pushdown() {
    let p = plan("ls role:test").unwrap();
    assert_eq!(p.classified_filters.len(), 1);
    assert_eq!(p.classified_filters[0].class, FilterClass::SourcePushdown);
}

#[test]
fn lines_filter_is_source_pushdown() {
    let p = plan("read src/main.rs lines 1..40").unwrap();
    let lines_filter = p
        .classified_filters
        .iter()
        .find(|cf| matches!(cf.filter, Filter::Lines(_)));
    assert!(lines_filter.is_some());
    assert_eq!(lines_filter.unwrap().class, FilterClass::SourcePushdown);
}

#[test]
fn about_filter_is_post_materialisation() {
    let p = plan("ls about foo").unwrap();
    assert_eq!(p.classified_filters.len(), 1);
    assert_eq!(
        p.classified_filters[0].class,
        FilterClass::PostMaterialization
    );
}

#[test]
fn negated_kv_filter_is_source_pushdown() {
    let p = plan("ls !role:test").unwrap();
    assert_eq!(p.classified_filters.len(), 1);
    assert_eq!(p.classified_filters[0].class, FilterClass::SourcePushdown);
}

// ── error cases ───────────────────────────────────────────────────────────────

#[test]
fn unknown_verb_returns_unknown_verb_error() {
    let err = plan("completelyfakecommand_xyz_99999").unwrap_err();
    assert!(
        matches!(err, PlanError::UnknownVerb { ref verb } if verb == "completelyfakecommand_xyz_99999")
    );
    let msg = err.to_string();
    assert!(msg.contains("completelyfakecommand_xyz_99999"));
    assert!(msg.contains("help"));
}

#[test]
fn compat_trap_returns_verb_denied() {
    let err = plan("bash").unwrap_err();
    assert!(matches!(err, PlanError::VerbDenied { ref verb, .. } if verb == "bash"));
    assert!(err.to_string().contains("bash"));
}

#[test]
fn blocked_curl_returns_verb_denied() {
    let err = plan("curl").unwrap_err();
    assert!(matches!(err, PlanError::VerbDenied { .. }));
}

#[test]
fn read_without_target_returns_missing_target() {
    let err = plan("read").unwrap_err();
    assert!(matches!(err, PlanError::MissingTarget { ref verb } if verb == "read"));
    assert!(err.to_string().contains("read"));
}

#[test]
fn write_without_target_returns_missing_target() {
    let err = plan("write").unwrap_err();
    assert!(matches!(err, PlanError::MissingTarget { ref verb } if verb == "write"));
}

#[test]
fn sorted_by_on_filelist_is_type_error() {
    let err = plan("ls | sorted by name").unwrap_err();
    assert!(
        matches!(err, PlanError::RefinerTypeMismatch { ref result_type, .. }
            if *result_type == ResultType::FileList),
        "expected RefinerTypeMismatch(FileList), got {err:?}"
    );
    let msg = err.to_string();
    assert!(msg.contains("sorted by name"), "message was: {msg}");
    assert!(msg.contains("record set"), "message was: {msg}");
}

#[test]
fn grouped_by_on_file_content_is_type_error() {
    let err = plan("read foo.rs | grouped by ext").unwrap_err();
    assert!(
        matches!(err, PlanError::RefinerTypeMismatch { ref result_type, .. }
            if *result_type == ResultType::FileContent),
        "expected RefinerTypeMismatch(FileContent), got {err:?}"
    );
}

#[test]
fn sorted_by_on_env_map_is_type_error() {
    let err = plan("env | sorted by key").unwrap_err();
    assert!(matches!(
        err,
        PlanError::RefinerTypeMismatch {
            result_type: ResultType::EnvMap,
            ..
        }
    ));
}

#[test]
fn grouped_by_on_text_is_type_error() {
    let err = plan("check | grouped by file").unwrap_err();
    assert!(matches!(
        err,
        PlanError::RefinerTypeMismatch {
            result_type: ResultType::Text,
            ..
        }
    ));
}

#[test]
fn refiner_type_mismatch_display_names_refiner_and_type() {
    let err = plan("ls | grouped by extension").unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("grouped by extension"), "message was: {msg}");
    assert!(msg.contains("record set"), "message was: {msg}");
}

// ── edge cases ────────────────────────────────────────────────────────────────

#[test]
fn cd_without_target_is_valid() {
    let p = plan("cd").unwrap();
    assert_eq!(p.noun, NounKind::DirPath);
    assert!(p.classified_filters.is_empty());
}

#[test]
fn help_produces_plural_text() {
    let p = plan("help").unwrap();
    assert_eq!(p.cardinality, Cardinality::Plural);
    assert_eq!(p.result_type, ResultType::Text);
    assert_eq!(p.mutation_class, MutationClass::ReadOnly);
}

#[test]
fn commands_noun_is_help() {
    let p = plan("commands").unwrap();
    assert_eq!(p.noun, NounKind::Help);
}

#[test]
fn multiple_pushdown_filters_all_classified() {
    let p = plan("ls role:test !kind:dir").unwrap();
    assert_eq!(p.classified_filters.len(), 2);
    assert!(p
        .classified_filters
        .iter()
        .all(|cf| cf.class == FilterClass::SourcePushdown));
}

#[test]
fn env_is_plural_envmap_readonly() {
    let p = plan("env").unwrap();
    assert_eq!(p.noun, NounKind::EnvMap);
    assert_eq!(p.cardinality, Cardinality::Plural);
    assert_eq!(p.result_type, ResultType::EnvMap);
    assert_eq!(p.mutation_class, MutationClass::ReadOnly);
}

#[test]
fn check_is_fileset_plural_text() {
    let p = plan("check").unwrap();
    assert_eq!(p.noun, NounKind::FileSet);
    assert_eq!(p.cardinality, Cardinality::Plural);
    assert_eq!(p.result_type, ResultType::Text);
}
