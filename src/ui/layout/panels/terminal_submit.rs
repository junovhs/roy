#[path = "terminal_submit_record.rs"]
mod record;

#[cfg(test)]
pub(crate) use record::record_session_outcome;
#[cfg(not(test))]
pub(crate) use record::record_session_outcome;
