//! `aidw session` — Session start/end automation

use anyhow::Result;
use std::path::Path;
use std::process::Command;

use aidw_core::{Config, Phase, Progress};

pub fn start(project_dir: &Path) -> Result<()> {
    println!("  ━━━ Session Start ━━━");
    println!();

    // 1. Load and show progress
    let config = Config::load(project_dir).ok();
    let progress_path = config
        .as_ref()
        .map(|c| project_dir.join(&c.paths.progress))
        .unwrap_or_else(|| project_dir.join("docs/progress/PROGRESS.md"));

    if let Ok(progress) = Progress::load(&progress_path) {
        // Show in-progress items
        if !progress.in_progress.is_empty() {
            println!("  🚧 In Progress:");
            for item in &progress.in_progress {
                println!("     • {}", item.text);
            }
            println!();
        }

        // Show next items
        if !progress.next.is_empty() {
            println!("  🎯 Next:");
            for item in progress.next.iter().take(3) {
                println!("     • {}", item.text);
            }
            if progress.next.len() > 3 {
                println!("     ... and {} more", progress.next.len() - 3);
            }
            println!();
        }
    } else {
        println!("  ⚠ No PROGRESS.md found. Run `aidw init` to set up.");
        println!();
    }

    // 2. Show current task/phase
    if let Some(ref config) = config {
        if let Some(ref task) = config.current_task {
            let phase = Phase::parse(&task.phase).unwrap_or(Phase::Context);
            println!("  📋 Current task: {} — {}", task.id, task.title);
            println!("     {}", phase.render_bar());
            println!();
        } else {
            println!("  📋 No active task. Use `aidw phase start <id> <title>` to begin.");
            println!();
        }
    }

    // 3. Show git status
    let git_output = Command::new("git")
        .args(["status", "--short", "--branch"])
        .current_dir(project_dir)
        .output();

    if let Ok(output) = git_output {
        if output.status.success() {
            let status = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = status.lines().collect();
            if let Some(branch_line) = lines.first() {
                println!("  🔀 {}", branch_line.trim_start_matches("## "));
            }
            let changes = lines.len() - 1; // subtract branch line
            if changes > 0 {
                println!("     {} uncommitted change(s)", changes);
            } else {
                println!("     Working tree clean");
            }
            println!();
        }
    }

    println!("  ━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!("  Ready. What are we working on?");

    Ok(())
}

pub fn end(project_dir: &Path) -> Result<()> {
    println!("  ━━━ Session End ━━━");
    println!();

    let config = Config::load(project_dir).ok();
    let progress_path = config
        .as_ref()
        .map(|c| project_dir.join(&c.paths.progress))
        .unwrap_or_else(|| project_dir.join("docs/progress/PROGRESS.md"));

    // 1. Show what was accomplished (git log since session)
    let git_output = Command::new("git")
        .args(["log", "--oneline", "-5", "--no-decorate"])
        .current_dir(project_dir)
        .output();

    if let Ok(output) = git_output {
        if output.status.success() {
            let log = String::from_utf8_lossy(&output.stdout);
            if !log.is_empty() {
                println!("  📦 Recent commits:");
                for line in log.lines().take(5) {
                    println!("     {}", line);
                }
                println!();
            }
        }
    }

    // 2. Show current progress state
    if let Ok(progress) = Progress::load(&progress_path) {
        if !progress.in_progress.is_empty() {
            println!("  🚧 Still in progress:");
            for item in &progress.in_progress {
                println!("     • {}", item.text);
            }
            println!();
        }
    }

    // 3. Show current phase
    if let Some(ref config) = config {
        if let Some(ref task) = config.current_task {
            let phase = Phase::parse(&task.phase).unwrap_or(Phase::Context);
            println!("  📋 Task: {} — {} [{}]", task.id, task.title, phase.display_name());
            println!();
        }
    }

    // 4. Reminders
    println!("  📝 Session-end checklist:");
    println!("     [ ] PROGRESS.md updated?  (`aidw progress add -s done \"...\"`)");
    println!("     [ ] Decisions logged?     (`aidw adr new \"...\"` if relevant)");
    println!("     [ ] Code committed?       (`git status`)");
    println!("     [ ] Tests passing?        (`aidw verify`)");
    println!();

    // 5. Check git status
    let git_status = Command::new("git")
        .args(["status", "--short"])
        .current_dir(project_dir)
        .output();

    if let Ok(output) = git_status {
        if output.status.success() {
            let status = String::from_utf8_lossy(&output.stdout);
            let uncommitted: Vec<&str> = status.lines().collect();
            if !uncommitted.is_empty() {
                println!("  ⚠ {} uncommitted file(s):", uncommitted.len());
                for line in uncommitted.iter().take(5) {
                    println!("     {}", line);
                }
                if uncommitted.len() > 5 {
                    println!("     ... and {} more", uncommitted.len() - 5);
                }
                println!();
            }
        }
    }

    println!("  ━━━━━━━━━━━━━━━━━━━━━");

    Ok(())
}
