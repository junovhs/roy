use std::process::Command;

use super::*;

impl CapabilityRuntime {
    pub(super) fn execute_validation(
        &self,
        capability: &ValidationCapability,
    ) -> Result<CapabilityOutput, CapabilityError> {
        match capability {
            ValidationCapability::CargoCheck => self.run_cargo_check(),
        }
    }

    fn run_cargo_check(&self) -> Result<CapabilityOutput, CapabilityError> {
        let manifest = self.cwd.join("Cargo.toml");
        if !manifest.is_file() {
            return Err(CapabilityError::new(format!(
                "check: no Cargo.toml in {}",
                self.cwd.display()
            )));
        }

        let output = Command::new("cargo")
            .args(["check", "--quiet", "--offline"])
            .current_dir(&self.cwd)
            .output()
            .map_err(|err| CapabilityError::new(format!("check: failed to run cargo: {err}")))?;

        let exit_code = output.status.code().unwrap_or(1);
        Ok(CapabilityOutput::ValidationRun {
            command: "cargo check --quiet --offline".to_string(),
            cwd: self.cwd.clone(),
            exit_code,
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        })
    }
}
