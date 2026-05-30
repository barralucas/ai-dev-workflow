//! aidw — AI Dev Workflow CLI
//!
//! An opinionated, stack-agnostic workflow tool for building software with AI agents.

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod commands;

#[derive(Parser)]
#[command(
    name = "aidw",
    version,
    about = "AI Dev Workflow — opinionated workflow tool for building software with AI agents",
    long_about = "An interactive terminal tool that manages the 7-phase development workflow:\nContext → Design → Plan → Execute → Verify → Document → Handoff\n\nRun without subcommands to launch the interactive TUI dashboard."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Project directory (defaults to current directory)
    #[arg(short, long, global = true)]
    dir: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new project with the ai-dev-workflow
    Init {
        /// Project name (defaults to directory name)
        #[arg(short, long)]
        name: Option<String>,

        /// Technology stack (nextjs, node-backend, python, mobile, rust, go)
        #[arg(short, long)]
        stack: Option<String>,

        /// Skip interactive prompts (use defaults/flags)
        #[arg(short = 'y', long)]
        yes: bool,
    },

    /// Adopt the workflow into an existing project
    Adopt {
        /// Force stack detection override
        #[arg(short, long)]
        stack: Option<String>,

        /// Only install minimal files (workflow.instructions.md + PROGRESS.md)
        #[arg(long)]
        minimal: bool,

        /// Show what would be done without writing files
        #[arg(long)]
        dry_run: bool,

        /// Skip confirmations
        #[arg(short = 'y', long)]
        yes: bool,
    },

    /// View and manage PROGRESS.md
    Progress {
        #[command(subcommand)]
        action: Option<ProgressAction>,
    },

    /// Manage workflow phases
    Phase {
        #[command(subcommand)]
        action: Option<PhaseAction>,
    },

    /// Manage Architecture Decision Records
    Adr {
        #[command(subcommand)]
        action: Option<AdrAction>,
    },

    /// Run quality gate pipeline (lint, typecheck, test, build)
    Verify {
        /// Run only a specific gate (lint, typecheck, test, build)
        step: Option<String>,
    },

    /// Show one-line project status summary
    Status,

    /// Check project health and configuration
    Doctor,

    /// Session lifecycle management
    Session {
        #[command(subcommand)]
        action: SessionAction,
    },
}

#[derive(Subcommand)]
enum ProgressAction {
    /// Show current progress summary
    Show,
    /// Add an item to a section
    Add {
        /// Section: done, in-progress, next, debt, decisions
        #[arg(short, long)]
        section: String,
        /// Item text
        text: String,
    },
    /// Move an item between sections
    Move {
        /// Text to search for
        text: String,
        /// Source section
        #[arg(long)]
        from: String,
        /// Target section
        #[arg(long)]
        to: String,
    },
}

#[derive(Subcommand)]
enum PhaseAction {
    /// Show current phase
    Show,
    /// Advance to next phase
    Next,
    /// Go back to previous phase
    Back,
    /// Set a specific phase
    Set {
        /// Phase name: context, design, plan, execute, verify, document, handoff
        phase: String,
    },
    /// Start tracking a new task
    Start {
        /// Task ID (e.g., US-001)
        id: String,
        /// Task title
        title: String,
    },
}

#[derive(Subcommand)]
enum AdrAction {
    /// Create a new ADR
    New {
        /// ADR title
        title: String,
    },
    /// List all ADRs
    List,
}

#[derive(Subcommand)]
enum SessionAction {
    /// Begin a work session (shows context, progress, git status)
    Start,
    /// End a work session (checklist, reminders, uncommitted changes)
    End,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let project_dir = cli.dir.unwrap_or_else(|| std::env::current_dir().unwrap());

    match cli.command {
        None => {
            // No subcommand: launch TUI
            let mut app = aidw_tui::App::new(project_dir);
            app.run()?;
        }
        Some(Commands::Init { name, stack, yes }) => {
            commands::init::run(project_dir, name, stack, yes)?;
        }
        Some(Commands::Adopt { stack, minimal, dry_run, yes }) => {
            commands::adopt::run(project_dir, stack, minimal, dry_run, yes)?;
        }
        Some(Commands::Progress { action }) => {
            commands::progress::run(project_dir, action)?;
        }
        Some(Commands::Phase { action }) => {
            commands::phase::run(project_dir, action)?;
        }
        Some(Commands::Adr { action }) => {
            commands::adr::run(project_dir, action)?;
        }
        Some(Commands::Verify { step }) => {
            commands::verify::run(&project_dir, step.as_deref())?;
        }
        Some(Commands::Status) => {
            commands::status::run(&project_dir)?;
        }
        Some(Commands::Doctor) => {
            commands::doctor::run(&project_dir)?;
        }
        Some(Commands::Session { action }) => match action {
            SessionAction::Start => commands::session::start(&project_dir)?,
            SessionAction::End => commands::session::end(&project_dir)?,
        },
    }

    Ok(())
}
