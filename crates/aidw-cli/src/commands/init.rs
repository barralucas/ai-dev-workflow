//! `aidw init` — Initialize a new project with the workflow.

use anyhow::Result;
use std::path::PathBuf;

use aidw_core::{Config, Stack, StackDetector, templates::Templates};

pub fn run(
    project_dir: PathBuf,
    name: Option<String>,
    stack: Option<String>,
    _yes: bool,
) -> Result<()> {
    println!("→ Initializing ai-dev-workflow in: {}", project_dir.display());
    println!();

    // Determine project name
    let project_name = name.unwrap_or_else(|| {
        project_dir
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "my-project".to_string())
    });

    // Determine stack
    let detected_stack = StackDetector::detect(&project_dir);
    let chosen_stack = if let Some(s) = stack {
        Stack::parse(&s)
    } else if detected_stack != Stack::Unknown {
        println!("  Detected stack: {}", detected_stack.display_name());
        detected_stack
    } else {
        println!("  No stack detected, using 'none'");
        Stack::Unknown
    };

    println!("  Project name: {}", project_name);
    println!("  Stack: {}", chosen_stack.display_name());
    println!();

    // Create config
    let config = Config::new(project_name.clone(), chosen_stack.as_str().to_string());

    // Set suggested commands based on stack
    let mut config = config;
    let cmds = StackDetector::suggested_commands(chosen_stack);
    config.commands.lint = cmds.lint;
    config.commands.typecheck = cmds.typecheck;
    config.commands.test = cmds.test;
    config.commands.build = cmds.build;

    // Write .aidw.toml
    config.save(&project_dir)?;
    println!("  ✓ Created .aidw.toml");

    // Write docs templates
    let written = Templates::write_docs(&project_dir, &project_name, false)?;
    for file in &written {
        println!("  ✓ Created {}", file);
    }

    if written.is_empty() {
        println!("  (docs/ already exists — skipped)");
    }

    // Write agent-agnostic skills
    let written_skills = Templates::write_skills(&project_dir, false)?;
    for file in &written_skills {
        println!("  ✓ Created {}", file);
    }

    if written_skills.is_empty() {
        println!("  (skills/ already exists — skipped)");
    }

    // Write AGENTS.md if not exists
    let agents_path = project_dir.join("AGENTS.md");
    if !agents_path.exists() {
        if let Some(content) = Templates::get_content("README.template.md") {
            let content = content.replace("{{PROJECT_NAME}}", &project_name);
            std::fs::write(&agents_path, content)?;
            println!("  ✓ Created AGENTS.md");
        }
    }

    println!();
    println!("✅ Initialization complete!");
    println!();
    println!("Next steps:");
    println!("  1. Edit docs/progress/PROGRESS.md (set current sprint)");
    println!("  2. Edit docs/architecture/tech-stack.md (set versions)");
    println!("  3. Start with the `atlas` skill in your agent");
    println!("  4. Run `aidw` to open the dashboard");
    println!("  5. git add -A && git commit -m \"chore: bootstrap ai-dev-workflow\"");
    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use aidw_core::Config;

    #[test]
    fn test_init_creates_config_and_required_docs() {
        let dir = tempfile::tempdir().unwrap();

        run(
            dir.path().to_path_buf(),
            Some("Harness Project".to_string()),
            Some("rust".to_string()),
            true,
        )
        .unwrap();

        let config = Config::load(dir.path()).unwrap();
        assert_eq!(config.project.name, "Harness Project");
        assert_eq!(config.project.stack, "rust");
        assert_eq!(config.commands.test.as_deref(), Some("cargo test"));

        assert!(dir.path().join("docs/progress/PROGRESS.md").exists());
        assert!(dir.path().join("docs/architecture/tech-stack.md").exists());
        assert!(dir.path().join("docs/adr/0001-stack-inicial.md").exists());
        assert!(dir.path().join("skills/atlas/SKILL.md").exists());
        assert!(dir.path().join("skills/workflow/SKILL.md").exists());
    }
}
