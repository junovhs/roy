#[path = "terminal_submit_lines.rs"]
mod lines;
#[path = "terminal_submit_parse.rs"]
mod parse;
#[path = "terminal_submit_record.rs"]
mod record;
#[path = "terminal_submit_handle.rs"]
mod submit;

#[cfg(test)]
pub(crate) use record::record_session_outcome;
pub(crate) use submit::handle_submit;
