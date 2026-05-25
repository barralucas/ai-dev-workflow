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

impl Templates {
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
    fn test_get_template_content() {
        let content = Templates::get_content("docs/progress/PROGRESS.md");
        assert!(content.is_some(), "Should be able to read PROGRESS.md template");
        let text = content.unwrap();
        assert!(text.contains("{{PROJECT_NAME}}") || text.contains("PROGRESS"),
            "Template should contain expected content");
    }
}
