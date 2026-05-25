//! `aidw status` — One-line project status summary

use anyhow::Result;
use std::path::Path;

use aidw_core::{Config, Phase, Progress};

pub fn run(project_dir: &Path) -> Result<()> {
    let config = Config::load(project_dir).ok();
    let progress_path = config
        .as_ref()
        .map(|c| project_dir.join(&c.paths.progress))
        .unwrap_or_else(|| project_dir.join("docs/progress/PROGRESS.md"));
    let progress = Progress::load(&progress_path).ok();

    // Build the status line
    let project_name = config
        .as_ref()
        .map(|c| c.project.name.clone())
        .unwrap_or_else(|| {
            project_dir
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "unknown".to_string())
        });

    let stack = config
        .as_ref()
        .map(|c| c.project.stack.clone())
        .unwrap_or_else(|| "?".to_string());

    let phase = config
        .as_ref()
        .and_then(|c| c.current_task.as_ref())
        .and_then(|t| Phase::parse(&t.phase))
        .unwrap_or(Phase::Context);

    let task_info = config
        .as_ref()
        .and_then(|c| c.current_task.as_ref())
        .map(|t| format!("{}: {}", t.id, t.title))
        .unwrap_or_else(|| "no active task".to_string());

    let (in_progress_count, next_count, done_count, debt_count) = progress
        .as_ref()
        .map(|p| (p.in_progress.len(), p.next.len(), p.done.len(), p.tech_debt.len()))
        .unwrap_or((0, 0, 0, 0));

    // Format: [PHASE] project (stack) — task | counts
    println!(
        "  [{}] {} ({}) — {} | {} in-progress, {} next, {} done, {} debt",
        phase.display_name(),
        project_name,
        stack,
        task_info,
        in_progress_count,
        next_count,
        done_count,
        debt_count,
    );

    Ok(())
}
