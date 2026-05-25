//! `aidw adopt` — Adopt the workflow in an existing project.

use anyhow::Result;
use std::path::PathBuf;

use aidw_core::{Config, Stack, StackDetector, templates::Templates};

pub fn run(
    project_dir: PathBuf,
    stack: Option<String>,
    minimal: bool,
    dry_run: bool,
    _yes: bool,
) -> Result<()> {
    println!("→ Adopting ai-dev-workflow in: {}", project_dir.display());
    println!();

    // Inventory
    println!("  Inventory:");
    if project_dir.join("package.json").exists() {
        println!("    • package.json found");
    }
    if project_dir.join("pyproject.toml").exists() {
        println!("    • pyproject.toml found");
    }
    if project_dir.join("Cargo.toml").exists() {
        println!("    • Cargo.toml found");
    }
    if project_dir.join("go.mod").exists() {
        println!("    • go.mod found");
    }
    if project_dir.join(".git").exists() {
        println!("    • git repository: yes");
    }
    if project_dir.join("docs").exists() {
        println!("    • docs/ exists (will be preserved)");
    }
    println!();

    // Detect stack
    let detected_stack = StackDetector::detect(&project_dir);
    let chosen_stack = if let Some(s) = stack {
        Stack::parse(&s)
    } else {
        detected_stack
    };

    println!("  Stack detected: {}", chosen_stack.display_name());

    // Project name
    let project_name = project_dir
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "my-project".to_string());

    if dry_run {
        println!();
        println!("  [dry-run] Would create:");
        println!("    - .aidw.toml");
        println!("    - docs/progress/PROGRESS.md (if not exists)");
        println!("    - docs/progress/decisions-log.md (if not exists)");
        println!("    - docs/adr/0000-template.md (if not exists)");
        if !minimal {
            println!("    - docs/architecture/ (if not exists)");
            println!("    - docs/features/ (if not exists)");
            println!("    - docs/risks/ (if not exists)");
        }
        println!();
        println!("  [dry-run] No files written.");
        return Ok(());
    }

    // Create config
    let mut config = Config::new(project_name.clone(), chosen_stack.as_str().to_string());
    let cmds = StackDetector::suggested_commands(chosen_stack);
    config.commands.lint = cmds.lint;
    config.commands.typecheck = cmds.typecheck;
    config.commands.test = cmds.test;
    config.commands.build = cmds.build;

    // Write .aidw.toml (only if not exists)
    let config_path = project_dir.join(Config::FILE_NAME);
    if !config_path.exists() {
        config.save(&project_dir)?;
        println!("  ✓ Created .aidw.toml");
    } else {
        println!("  • .aidw.toml already exists — preserved");
    }

    // Write minimal docs
    let progress_dir = project_dir.join("docs/progress");
    std::fs::create_dir_all(&progress_dir)?;

    let progress_path = progress_dir.join("PROGRESS.md");
    if !progress_path.exists() {
        Templates::write_to(
            "docs/progress/PROGRESS.md",
            &progress_path,
            &[("{{PROJECT_NAME}}", &project_name)],
            false,
        )?;
        println!("  ✓ Created docs/progress/PROGRESS.md");
    } else {
        println!("  • docs/progress/PROGRESS.md already exists — preserved");
    }

    let decisions_path = progress_dir.join("decisions-log.md");
    if !decisions_path.exists() {
        Templates::write_to(
            "docs/progress/decisions-log.md",
            &decisions_path,
            &[("{{PROJECT_NAME}}", &project_name)],
            false,
        )?;
        println!("  ✓ Created docs/progress/decisions-log.md");
    }

    // ADR template
    let adr_dir = project_dir.join("docs/adr");
    std::fs::create_dir_all(&adr_dir)?;
    let adr_template = adr_dir.join("0000-template.md");
    if !adr_template.exists() {
        Templates::write_to(
            "docs/adr/0000-template.md",
            &adr_template,
            &[("{{PROJECT_NAME}}", &project_name)],
            false,
        )?;
        println!("  ✓ Created docs/adr/0000-template.md");
    }

    if !minimal {
        // Additional directories
        for sub in &["architecture", "features", "risks", "user-stories", "postmortem", "spikes"] {
            let dir = project_dir.join(format!("docs/{}", sub));
            if !dir.exists() {
                std::fs::create_dir_all(&dir)?;
                println!("  ✓ Created docs/{}/", sub);
            }
        }

        // Write templates for additional dirs (non-destructive)
        let written = Templates::write_docs(&project_dir, &project_name, false)?;
        for file in &written {
            if !file.contains("progress/") && !file.contains("adr/0000") {
                println!("  ✓ Created {}", file);
            }
        }
    }

    println!();
    println!("✅ Adoption complete!");
    println!();
    println!("Next steps:");
    println!("  1. Open the project with your AI agent (Copilot, Claude, Codex)");
    println!("  2. Ask it to populate PROGRESS.md from the existing code");
    println!("  3. Run `aidw` to see the dashboard");
    println!("  4. git add -A && git commit -m \"chore: adopt ai-dev-workflow\"");
    println!();

    Ok(())
}
