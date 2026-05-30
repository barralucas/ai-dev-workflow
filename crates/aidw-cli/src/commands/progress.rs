//! `aidw progress` — View and manage PROGRESS.md

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

use aidw_core::{Config, Progress};

use crate::ProgressAction;

pub fn run(project_dir: PathBuf, action: Option<ProgressAction>) -> Result<()> {
    let progress_path = Config::load(&project_dir)
        .map(|c| project_dir.join(&c.paths.progress))
        .unwrap_or_else(|_| project_dir.join("docs/progress/PROGRESS.md"));

    match action {
        None | Some(ProgressAction::Show) => show(&progress_path),
        Some(ProgressAction::Add { section, text }) => add(&progress_path, &section, text),
        Some(ProgressAction::Move { text, from, to }) => move_item(&progress_path, &text, &from, &to),
    }
}

fn show(path: &Path) -> Result<()> {
    let progress = Progress::load(path)
        .with_context(|| format!("Failed to load PROGRESS.md from {}", path.display()))?;

    println!("# {}", progress.project_name);
    println!();

    // In Progress
    println!("🚧 In Progress:");
    if progress.in_progress.is_empty() {
        println!("  (none)");
    }
    for item in &progress.in_progress {
        println!("  • {}", item.text);
        for sub in &item.sub_items {
            println!("    - {}", sub);
        }
    }
    println!();

    // Next
    println!("🎯 Next:");
    if progress.next.is_empty() {
        println!("  (none)");
    }
    for item in &progress.next {
        println!("  • {}", item.text);
    }
    println!();

    // Done (last 5)
    println!("✅ Done (recent):");
    if progress.done.is_empty() {
        println!("  (none)");
    }
    for item in progress.done.iter().rev().take(5) {
        println!("  • {}", item.text);
    }
    println!();

    // Tech Debt
    if !progress.tech_debt.is_empty() {
        println!("⚠️  Tech Debt:");
        for item in &progress.tech_debt {
            println!("  • {}", item.text);
        }
        println!();
    }

    // Decisions
    if !progress.decisions.is_empty() {
        println!("📝 Recent Decisions:");
        for d in progress.decisions.iter().rev().take(3) {
            println!("  • {}", d);
        }
        println!();
    }

    Ok(())
}

fn add(path: &Path, section: &str, text: String) -> Result<()> {
    let mut progress = Progress::load(path)
        .with_context(|| format!("Failed to load PROGRESS.md from {}", path.display()))?;

    progress.add_item(section, text.clone());
    progress.save(path)?;

    println!("✓ Added to {}: {}", section, text);
    Ok(())
}

fn move_item(path: &Path, text: &str, from: &str, to: &str) -> Result<()> {
    let mut progress = Progress::load(path)
        .with_context(|| format!("Failed to load PROGRESS.md from {}", path.display()))?;

    if progress.move_item(text, from, to) {
        progress.save(path)?;
        println!("✓ Moved '{}' from {} to {}", text, from, to);
    } else {
        println!("✗ Item containing '{}' not found in section '{}'", text, from);
    }

    Ok(())
}
