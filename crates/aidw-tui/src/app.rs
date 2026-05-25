//! Main TUI application state and event loop.

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;
use std::io::stdout;
use std::path::PathBuf;

use aidw_core::{Config, Progress, Phase};

/// Active tab in the TUI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Progress,
    Workflow,
    Info,
}

impl Tab {
    pub fn title(&self) -> &'static str {
        match self {
            Tab::Progress => "Progress",
            Tab::Workflow => "Workflow",
            Tab::Info => "Info",
        }
    }

    pub fn all() -> &'static [Tab] {
        &[Tab::Progress, Tab::Workflow, Tab::Info]
    }

    pub fn next(&self) -> Tab {
        match self {
            Tab::Progress => Tab::Workflow,
            Tab::Workflow => Tab::Info,
            Tab::Info => Tab::Progress,
        }
    }

    pub fn prev(&self) -> Tab {
        match self {
            Tab::Progress => Tab::Info,
            Tab::Workflow => Tab::Progress,
            Tab::Info => Tab::Workflow,
        }
    }
}

/// Main application state
pub struct App {
    /// Whether the app should quit
    pub should_quit: bool,
    /// Active tab
    pub active_tab: Tab,
    /// Project root directory
    pub project_dir: PathBuf,
    /// Loaded config (if available)
    pub config: Option<Config>,
    /// Loaded progress (if available)
    pub progress: Option<Progress>,
    /// Current workflow phase
    pub current_phase: Phase,
    /// Selected item index (for lists)
    pub selected_index: usize,
}

impl App {
    /// Create a new App instance
    pub fn new(project_dir: PathBuf) -> Self {
        let config = Config::load(&project_dir).ok();
        let progress = config
            .as_ref()
            .map(|c| project_dir.join(&c.paths.progress))
            .or_else(|| Some(project_dir.join("docs/progress/PROGRESS.md")))
            .and_then(|p| Progress::load(&p).ok());

        let current_phase = config
            .as_ref()
            .and_then(|c| c.current_task.as_ref())
            .and_then(|t| Phase::parse(&t.phase))
            .unwrap_or(Phase::Context);

        Self {
            should_quit: false,
            active_tab: Tab::Progress,
            project_dir,
            config,
            progress,
            current_phase,
            selected_index: 0,
        }
    }

    /// Run the TUI event loop
    pub fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

        loop {
            terminal.draw(|frame| crate::ui::render(frame, self))?;

            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        self.handle_key(key.code);
                    }
                }
            }

            if self.should_quit {
                break;
            }
        }

        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }

    /// Handle a key press
    fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Tab | KeyCode::Char('l') => self.active_tab = self.active_tab.next(),
            KeyCode::BackTab | KeyCode::Char('h') => self.active_tab = self.active_tab.prev(),
            KeyCode::Down | KeyCode::Char('j') => self.selected_index = self.selected_index.saturating_add(1),
            KeyCode::Up | KeyCode::Char('k') => self.selected_index = self.selected_index.saturating_sub(1),
            _ => {}
        }
    }
}
