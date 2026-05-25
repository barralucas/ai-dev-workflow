//! `aidw phase` — Workflow phase management

use anyhow::Result;
use std::path::{Path, PathBuf};

use aidw_core::{Config, Phase};

use crate::PhaseAction;

pub fn run(project_dir: PathBuf, action: Option<PhaseAction>) -> Result<()> {
    match action {
        None | Some(PhaseAction::Show) => show(&project_dir),
        Some(PhaseAction::Next) => advance(&project_dir),
        Some(PhaseAction::Back) => retreat(&project_dir),
        Some(PhaseAction::Set { phase }) => set_phase(&project_dir, &phase),
        Some(PhaseAction::Start { id, title }) => start_task(&project_dir, id, title),
    }
}

fn show(project_dir: &Path) -> Result<()> {
    let config = Config::load(project_dir);

    let (phase, task_info) = match &config {
        Ok(c) => {
            let phase = c
                .current_task
                .as_ref()
                .and_then(|t| Phase::parse(&t.phase))
                .unwrap_or(Phase::Context);
            let task = c
                .current_task
                .as_ref()
                .map(|t| format!("{} — {}", t.id, t.title));
            (phase, task)
        }
        Err(_) => (Phase::Context, None),
    };

    println!();
    println!("  {}", phase.render_bar());
    println!();

    if let Some(task) = task_info {
        println!("  Task: {}", task);
    } else {
        println!("  No active task. Use `aidw phase start <id> <title>` to begin.");
    }

    println!();
    println!("  Current: {} — {}", phase.display_name(), phase.description());
    println!();

    Ok(())
}

fn advance(project_dir: &Path) -> Result<()> {
    let mut config = Config::load(project_dir)?;

    let current = config
        .current_task
        .as_ref()
        .and_then(|t| Phase::parse(&t.phase))
        .unwrap_or(Phase::Context);

    if let Some(next) = current.next() {
        if let Some(ref mut task) = config.current_task {
            task.phase = format!("{:?}", next).to_lowercase();
        }
        config.save(project_dir)?;
        println!("  ✓ Advanced: {} → {}", current.display_name(), next.display_name());
        println!();
        println!("  {}", next.render_bar());
        println!();
        println!("  {}: {}", next.display_name(), next.description());
    } else {
        println!("  Already at HANDOFF (final phase).");
        println!("  Use `aidw phase start <id> <title>` to begin a new task.");
    }

    Ok(())
}

fn retreat(project_dir: &Path) -> Result<()> {
    let mut config = Config::load(project_dir)?;

    let current = config
        .current_task
        .as_ref()
        .and_then(|t| Phase::parse(&t.phase))
        .unwrap_or(Phase::Context);

    if let Some(prev) = current.prev() {
        if let Some(ref mut task) = config.current_task {
            task.phase = format!("{:?}", prev).to_lowercase();
        }
        config.save(project_dir)?;
        println!("  ← Retreated: {} → {}", current.display_name(), prev.display_name());
        println!();
        println!("  {}", prev.render_bar());
    } else {
        println!("  Already at CONTEXT (first phase).");
    }

    Ok(())
}

fn set_phase(project_dir: &Path, phase_str: &str) -> Result<()> {
    let phase = Phase::parse(phase_str).ok_or_else(|| {
        anyhow::anyhow!(
            "Invalid phase '{}'. Valid: context, design, plan, execute, verify, document, handoff",
            phase_str
        )
    })?;

    let mut config = Config::load(project_dir)?;
    if let Some(ref mut task) = config.current_task {
        task.phase = format!("{:?}", phase).to_lowercase();
    } else {
        return Err(anyhow::anyhow!(
            "No active task. Use `aidw phase start <id> <title>` first."
        ));
    }
    config.save(project_dir)?;

    println!("  ✓ Phase set to: {}", phase.display_name());
    println!("  {}", phase.render_bar());

    Ok(())
}

fn start_task(project_dir: &Path, id: String, title: String) -> Result<()> {
    let mut config = match Config::load(project_dir) {
        Ok(c) => c,
        Err(_) => {
            return Err(anyhow::anyhow!(
                "No .aidw.toml found. Run `aidw init` or `aidw adopt` first."
            ));
        }
    };

    config.current_task = Some(aidw_core::config::TaskConfig {
        id: id.clone(),
        title: title.clone(),
        phase: "context".to_string(),
        started_at: Some(chrono::Utc::now().to_rfc3339()),
    });

    config.save(project_dir)?;

    println!("  ✓ Started task: {} — {}", id, title);
    println!("  Phase: CONTEXT — {}", Phase::Context.description());
    println!();
    println!("  {}", Phase::Context.render_bar());

    Ok(())
}
