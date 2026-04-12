use super::super::super::terminal_model::ShellLine;

pub(super) fn append_io_lines(
    new_lines: &mut Vec<ShellLine>,
    output_lines: &[String],
    error_lines: &[String],
) {
    for line in output_lines {
        new_lines.push(ShellLine::output(line.clone()));
    }
    for line in error_lines {
        new_lines.push(ShellLine::error(line.clone()));
    }
}

pub(super) fn append_dispatch_lines(
    new_lines: &mut Vec<ShellLine>,
    result: &crate::shell::DispatchResult,
    output_lines: &[String],
    error_lines: &[String],
) {
    match result {
        crate::shell::DispatchResult::Denied {
            command,
            suggestion,
            ..
        } => {
            new_lines.push(ShellLine::denial_header(command));
            if let Some(hint) = suggestion.as_ref().filter(|hint| !hint.trim().is_empty()) {
                new_lines.push(ShellLine::denial_hint(hint));
            }
        }
        crate::shell::DispatchResult::NotFound { command } => {
            new_lines.push(ShellLine::not_found(command));
        }
        crate::shell::DispatchResult::Exit { code } => {
            append_io_lines(new_lines, output_lines, error_lines);
            new_lines.push(ShellLine::output(format!(
                "[session ended · exit {}]",
                code
            )));
        }
        _ => append_io_lines(new_lines, output_lines, error_lines),
    }
}
