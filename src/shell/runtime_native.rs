use super::*;
use crate::capabilities::CapabilityOutput;
use crate::session::SessionArtifact;

impl ShellRuntime {
    pub(super) fn dispatch_native(&mut self, command: &str, args: &[&str]) -> DispatchResult {
        let request = match parse_native_request(command, args) {
            Ok(Some(request)) => request,
            Ok(None) => return self.not_found(command),
            Err(message) => return self.native_usage_error(command, message),
        };

        let runtime = CapabilityRuntime::new(self.workspace.clone(), self.env.cwd().to_path_buf());
        match runtime.execute(&request) {
            Ok(output) => {
                let primary = output.primary_text();
                let exit_code = output.exit_code();
                let artifacts = promoted_artifacts(&output);

                if !primary.is_empty() {
                    if exit_code == 0 {
                        self.io.write_line(&primary);
                    } else {
                        self.io.write_error(&primary);
                    }
                }

                if let Some(stderr) = output.error_text() {
                    if !stderr.is_empty() && stderr != primary {
                        self.io.write_error(stderr);
                    }
                }

                self.last_exit_status = Some(exit_code);
                DispatchResult::Executed {
                    output: primary,
                    exit_code,
                    artifacts,
                }
            }
            Err(err) => {
                let message = format!("{command}: {err}");
                self.io.write_error(&message);
                self.last_exit_status = Some(1);
                DispatchResult::Executed {
                    output: message,
                    exit_code: 1,
                    artifacts: Vec::new(),
                }
            }
        }
    }

    fn native_usage_error(&mut self, command: &str, message: String) -> DispatchResult {
        let message = format!("{command}: {message}");
        self.io.write_error(&message);
        self.last_exit_status = Some(2);
        DispatchResult::Executed {
            output: message,
            exit_code: 2,
            artifacts: Vec::new(),
        }
    }
}

fn promoted_artifacts(output: &CapabilityOutput) -> Vec<SessionArtifact> {
    match output {
        CapabilityOutput::ValidationRun {
            command,
            cwd,
            exit_code,
            stdout,
            stderr,
        } => vec![SessionArtifact::validation_run(
            command.clone(),
            cwd.clone(),
            *exit_code,
            stdout.clone(),
            stderr.clone(),
        )],
        CapabilityOutput::FileWritten {
            path,
            previous_contents,
            contents,
            ..
        } => SessionArtifact::diff(path.clone(), previous_contents.clone(), contents.clone())
            .into_iter()
            .collect(),
        _ => Vec::new(),
    }
}
