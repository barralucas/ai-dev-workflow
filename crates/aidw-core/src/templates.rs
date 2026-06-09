//! Template management with embedded assets.
//!
//! All workflow templates are embedded into the binary at compile time,
//! making the tool a single self-contained binary.

use rust_embed::Embed;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TemplateError {
    #[error("Template not found: {0}")]
    NotFound(String),
    #[error("Failed to write template: {0}")]
    WriteError(#[from] std::io::Error),
    #[error("Target file already exists: {0}")]
    AlreadyExists(String),
}

/// Embedded template assets from the templates/ directory
#[derive(Embed)]
#[folder = "../../templates/"]
pub struct Templates;

/// Embedded agent-agnostic skill assets from the repository skill directory.
#[derive(Embed)]
#[folder = "../../skills/"]
pub struct Skills;

/// Embedded workflow metadata assets.
#[derive(Embed)]
#[folder = "../../"]
#[include = "VERSION"]
pub struct WorkflowMetadata;

impl Templates {
    pub const VERSION_FILE: &'static str = ".aidw-version";

    /// Current AI Dev Workflow version embedded in the binary.
    pub fn workflow_version() -> String {
        WorkflowMetadata::get("VERSION")
            .map(|f| String::from_utf8_lossy(&f.data).trim().to_string())
            .unwrap_or_else(|| env!("CARGO_PKG_VERSION").to_string())
    }

    /// Read the workflow version previously installed in a project.
    pub fn installed_version(target_dir: &Path) -> Option<String> {
        let path = target_dir.join(Self::VERSION_FILE);
        std::fs::read_to_string(path).ok().map(|s| s.trim().to_string()).filter(|s| !s.is_empty())
    }

    /// Write the workflow version marker to a project.
    pub fn write_version_marker(target_dir: &Path) -> Result<(), TemplateError> {
        std::fs::write(target_dir.join(Self::VERSION_FILE), format!("{}\n", Self::workflow_version()))?;
        Ok(())
    }

    /// Significant updates are major/minor changes. Patch updates stay quiet.
    pub fn is_significant_update(previous: &str, current: &str) -> bool {
        fn major_minor(version: &str) -> Option<(&str, &str)> {
            let mut parts = version.split('.');
            Some((parts.next()?, parts.next()?))
        }

        match (major_minor(previous), major_minor(current)) {
            (Some(prev), Some(curr)) => prev != curr,
            _ => previous != current,
        }
    }

    /// List all available template files
    pub fn list() -> Vec<String> {
        Self::iter().map(|f| f.to_string()).collect()
    }

    /// Get template content by path
    pub fn get_content(path: &str) -> Option<String> {
        Self::get(path).map(|f| String::from_utf8_lossy(&f.data).to_string())
    }

    /// Write a template to a target path, replacing placeholders
    pub fn write_to(
        template_path: &str,
        target: &Path,
        replacements: &[(&str, &str)],
        overwrite: bool,
    ) -> Result<(), TemplateError> {
        if target.exists() && !overwrite {
            return Err(TemplateError::AlreadyExists(
                target.display().to_string(),
            ));
        }

        let content = Self::get_content(template_path)
            .ok_or_else(|| TemplateError::NotFound(template_path.to_string()))?;

        let mut result = content;
        for (placeholder, value) in replacements {
            result = result.replace(placeholder, value);
        }

        // Ensure parent directory exists
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(target, result)?;
        Ok(())
    }

    /// Write all docs templates to a target directory
    pub fn write_docs(target_dir: &Path, project_name: &str, overwrite: bool) -> Result<Vec<String>, TemplateError> {
        let replacements = [("{{PROJECT_NAME}}", project_name)];
        let mut written = Vec::new();

        for file_path in Self::iter() {
            let file_str = file_path.to_string();
            // Only copy docs/ templates
            if !file_str.starts_with("docs/") {
                continue;
            }

            let target = target_dir.join(&file_str);

            if target.exists() && !overwrite {
                continue;
            }

            match Self::write_to(&file_str, &target, &replacements, overwrite) {
                Ok(()) => written.push(file_str),
                Err(TemplateError::AlreadyExists(_)) => continue,
                Err(e) => return Err(e),
            }
        }

        Ok(written)
    }

    /// Write all agent-agnostic skills to a target directory.
    pub fn write_skills(target_dir: &Path, overwrite: bool) -> Result<Vec<String>, TemplateError> {
        let mut written = Vec::new();

        for file_path in Skills::iter() {
            let file_str = file_path.to_string();
            let target = target_dir.join("skills").join(&file_str);

            if target.exists() && !overwrite {
                continue;
            }

            let content = Skills::get(&file_str)
                .ok_or_else(|| TemplateError::NotFound(format!("skills/{file_str}")))?;

            if let Some(parent) = target.parent() {
                std::fs::create_dir_all(parent)?;
            }

            std::fs::write(&target, content.data.as_ref())?;
            written.push(format!("skills/{file_str}"));
        }

        Ok(written)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_templates_embedded() {
        let files = Templates::list();
        assert!(!files.is_empty(), "Templates should be embedded");
        // Check for key template files
        assert!(
            files.iter().any(|f| f.contains("PROGRESS.md")),
            "PROGRESS.md template should exist"
        );
    }

    #[test]
    fn test_required_sdd_harness_templates_are_embedded() {
        let required_templates = [
            "docs/progress/PROGRESS.md",
            "docs/progress/decisions-log.md",
            "docs/adr/0000-template.md",
            "docs/adr/0001-stack-inicial.md",
            "docs/architecture/overview.md",
            "docs/architecture/tech-stack.md",
            "docs/architecture/data-model.md",
            "docs/features/_template.md",
            "docs/user-stories/backlog.md",
            "docs/risks/risk-register.md",
            "docs/postmortem/_template.md",
            "docs/spikes/_template.md",
        ];

        for template in required_templates {
            assert!(
                Templates::get_content(template).is_some(),
                "Required template should be embedded: {template}"
            );
        }
    }

    #[test]
    fn test_required_skills_are_embedded() {
        let required_skills = [
            "atlas/SKILL.md",
            "workflow/SKILL.md",
            "new-feature/SKILL.md",
            "bug-fix/SKILL.md",
            "testing/SKILL.md",
            "documentation/SKILL.md",
        ];

        for skill in required_skills {
            assert!(
                Skills::get(skill).is_some(),
                "Required skill should be embedded: {skill}"
            );
        }
    }

    #[test]
    fn test_significant_update_detection() {
        assert!(!Templates::is_significant_update("0.1.0", "0.1.1"));
        assert!(Templates::is_significant_update("0.1.0", "0.2.0"));
        assert!(Templates::is_significant_update("0.1.0", "1.0.0"));
    }

    #[test]
    fn test_get_template_content() {
        let content = Templates::get_content("docs/progress/PROGRESS.md");
        assert!(content.is_some(), "Should be able to read PROGRESS.md template");
        let text = content.unwrap();
        assert!(
            text.contains("{{PROJECT_NAME}}") || text.contains("PROGRESS"),
            "Template should contain expected content"
        );
    }

    #[test]
    fn test_write_docs_creates_required_project_context() {
        let dir = tempfile::tempdir().unwrap();
        let written = Templates::write_docs(dir.path(), "Harness Project", false).unwrap();

        assert!(!written.is_empty(), "Expected docs templates to be written");
        assert!(dir.path().join("docs/progress/PROGRESS.md").exists());
        assert!(dir.path().join("docs/architecture/tech-stack.md").exists());
        assert!(dir.path().join("docs/adr/0001-stack-inicial.md").exists());
        assert!(dir.path().join("docs/risks/risk-register.md").exists());

        let progress =
            std::fs::read_to_string(dir.path().join("docs/progress/PROGRESS.md")).unwrap();
        assert!(progress.contains("Harness Project"));
    }

    #[test]
    fn test_write_skills_creates_atlas_and_preserves_existing() {
        let dir = tempfile::tempdir().unwrap();
        let atlas_path = dir.path().join("skills/atlas/SKILL.md");
        std::fs::create_dir_all(atlas_path.parent().unwrap()).unwrap();
        std::fs::write(&atlas_path, "custom atlas").unwrap();

        let written = Templates::write_skills(dir.path(), false).unwrap();

        assert!(!written.is_empty(), "Expected skills to be written");
        assert_eq!(std::fs::read_to_string(&atlas_path).unwrap(), "custom atlas");
        assert!(dir.path().join("skills/workflow/SKILL.md").exists());
    }
}
