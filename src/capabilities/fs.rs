use std::path::{Path, PathBuf};

use super::*;

impl CapabilityRuntime {
    pub(super) fn execute_fs(
        &self,
        capability: &FsCapability,
    ) -> Result<CapabilityOutput, CapabilityError> {
        match capability {
            FsCapability::ListDir { path } => self.list_dir(path.as_deref()),
            FsCapability::ReadFile { path } => self.read_file(path),
            FsCapability::WriteFile { path, contents } => self.write_file(path, contents),
        }
    }

    fn list_dir(&self, path: Option<&str>) -> Result<CapabilityOutput, CapabilityError> {
        let target = self.resolve_existing(path.unwrap_or("."))?;
        if !target.is_dir() {
            return Err(CapabilityError::new(format!(
                "{} is not a directory",
                target.display()
            )));
        }

        let mut entries: Vec<WorkspaceEntry> = std::fs::read_dir(&target)
            .map_err(|err| CapabilityError::new(format!("ls: {err}")))?
            .filter_map(Result::ok)
            .map(|entry| {
                let file_type = entry.file_type().ok();
                let kind = match file_type {
                    Some(kind) if kind.is_dir() => "dir",
                    Some(kind) if kind.is_symlink() => "link",
                    _ => "file",
                };

                WorkspaceEntry {
                    name: entry.file_name().to_string_lossy().into_owned(),
                    kind,
                }
            })
            .collect();

        entries.sort_by(|left, right| left.name.cmp(&right.name));
        Ok(CapabilityOutput::DirectoryListing {
            path: target,
            entries,
        })
    }

    fn read_file(&self, path: &str) -> Result<CapabilityOutput, CapabilityError> {
        let target = self.resolve_existing(path)?;
        if !target.is_file() {
            return Err(CapabilityError::new(format!(
                "{} is not a file",
                target.display()
            )));
        }

        let contents = std::fs::read_to_string(&target)
            .map_err(|err| CapabilityError::new(format!("read: {err}")))?;

        Ok(CapabilityOutput::FileContents {
            path: target,
            contents,
        })
    }

    fn write_file(&self, path: &str, contents: &str) -> Result<CapabilityOutput, CapabilityError> {
        let target = self.resolve_write_target(path)?;
        if target.exists() && target.is_dir() {
            return Err(CapabilityError::new(format!(
                "{} is a directory",
                target.display()
            )));
        }

        let previous_contents = if target.exists() {
            Some(String::from_utf8_lossy(
                &std::fs::read(&target)
                    .map_err(|err| CapabilityError::new(format!("write: {err}")))?,
            )
            .into_owned())
        } else {
            None
        };

        std::fs::write(&target, contents)
            .map_err(|err| CapabilityError::new(format!("write: {err}")))?;

        Ok(CapabilityOutput::FileWritten {
            path: target,
            bytes_written: contents.len(),
            previous_contents,
            contents: contents.to_string(),
        })
    }

    fn resolve_existing(&self, raw: &str) -> Result<PathBuf, CapabilityError> {
        let absolute = self.absolute_path(raw);
        let canonical = absolute
            .canonicalize()
            .map_err(|_| CapabilityError::new(format!("{raw}: path not found")))?;

        if !self.workspace.contains(&canonical) {
            return Err(CapabilityError::new(format!(
                "{} escapes workspace boundary ({})",
                canonical.display(),
                self.workspace.root().display()
            )));
        }

        Ok(canonical)
    }

    fn resolve_write_target(&self, raw: &str) -> Result<PathBuf, CapabilityError> {
        let absolute = self.absolute_path(raw);
        let parent = absolute.parent().unwrap_or(self.cwd.as_path());
        let canonical_parent = parent.canonicalize().map_err(|_| {
            CapabilityError::new(format!("write: parent directory not found for {raw}"))
        })?;

        if !self.workspace.contains(&canonical_parent) {
            return Err(CapabilityError::new(format!(
                "{} escapes workspace boundary ({})",
                canonical_parent.display(),
                self.workspace.root().display()
            )));
        }

        let Some(file_name) = absolute.file_name() else {
            return Err(CapabilityError::new(format!("write: invalid target {raw}")));
        };

        Ok(canonical_parent.join(file_name))
    }

    fn absolute_path(&self, raw: &str) -> PathBuf {
        let path = Path::new(raw);
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.cwd.join(path)
        }
    }
}
