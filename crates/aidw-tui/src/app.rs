//! Main TUI application state and event loop.

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;
use std::io::stdout;
use std::path::PathBuf;

use aidw_core::{Config, Phase, Progress};

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

/// Input mode for text entry
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputMode {
    /// Normal navigation mode
    Normal,
    /// Typing into input field
    Editing {
        /// What the input is for
        purpose: InputPurpose,
        /// Current input buffer
        buffer: String,
    },
}

/// What the text input is being used for
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputPurpose {
    AddToInProgress,
    AddToNext,
    AddToDone,
    AddToDebt,
}

impl InputPurpose {
    pub fn section_name(&self) -> &'static str {
        match self {
            InputPurpose::AddToInProgress => "in_progress",
            InputPurpose::AddToNext => "next",
            InputPurpose::AddToDone => "done",
            InputPurpose::AddToDebt => "tech_debt",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            InputPurpose::AddToInProgress => "Add to In Progress",
            InputPurpose::AddToNext => "Add to Next",
            InputPurpose::AddToDone => "Add to Done",
            InputPurpose::AddToDebt => "Add to Tech Debt",
        }
    }
}

/// Which list panel is focused in Progress tab
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgressPanel {
    InProgress,
    Next,
    Done,
    Debt,
}

impl ProgressPanel {
    pub fn next(self) -> Self {
        match self {
            ProgressPanel::InProgress => ProgressPanel::Next,
            ProgressPanel::Next => ProgressPanel::Done,
            ProgressPanel::Done => ProgressPanel::Debt,
            ProgressPanel::Debt => ProgressPanel::InProgress,
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
    /// Input mode (normal vs editing)
    pub input_mode: InputMode,
    /// Currently focused panel in Progress tab
    pub active_panel: ProgressPanel,
    /// Status message (shown briefly)
    pub status_message: Option<String>,
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
            input_mode: InputMode::Normal,
            active_panel: ProgressPanel::InProgress,
            status_message: None,
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
                        self.handle_key(key.code, key.modifiers);
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
    fn handle_key(&mut self, key: KeyCode, modifiers: KeyModifiers) {
        match &self.input_mode {
            InputMode::Normal => self.handle_normal_key(key, modifiers),
            InputMode::Editing { .. } => self.handle_editing_key(key),
        }
    }

    /// Handle keys in normal mode
    fn handle_normal_key(&mut self, key: KeyCode, _modifiers: KeyModifiers) {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Tab => self.active_tab = self.active_tab.next(),
            KeyCode::BackTab => self.active_tab = self.active_tab.prev(),
            KeyCode::Char('1') => self.active_tab = Tab::Progress,
            KeyCode::Char('2') => self.active_tab = Tab::Workflow,
            KeyCode::Char('3') => self.active_tab = Tab::Info,
            KeyCode::Down | KeyCode::Char('j') => {
                self.selected_index = self.selected_index.saturating_add(1);
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.selected_index = self.selected_index.saturating_sub(1);
            }
            // Tab-specific actions
            _ => match self.active_tab {
                Tab::Progress => self.handle_progress_key(key),
                Tab::Workflow => self.handle_workflow_key(key),
                Tab::Info => {}
            },
        }
    }

    /// Handle keys specific to Progress tab
    fn handle_progress_key(&mut self, key: KeyCode) {
        match key {
            // Switch panels with h/l
            KeyCode::Char('l') => {
                self.active_panel = self.active_panel.next();
                self.selected_index = 0;
            }
            KeyCode::Char('h') => {
                self.active_panel = match self.active_panel {
                    ProgressPanel::InProgress => ProgressPanel::Debt,
                    ProgressPanel::Next => ProgressPanel::InProgress,
                    ProgressPanel::Done => ProgressPanel::Next,
                    ProgressPanel::Debt => ProgressPanel::Done,
                };
                self.selected_index = 0;
            }
            // Add item to current panel
            KeyCode::Char('a') => {
                let purpose = match self.active_panel {
                    ProgressPanel::InProgress => InputPurpose::AddToInProgress,
                    ProgressPanel::Next => InputPurpose::AddToNext,
                    ProgressPanel::Done => InputPurpose::AddToDone,
                    ProgressPanel::Debt => InputPurpose::AddToDebt,
                };
                self.input_mode = InputMode::Editing {
                    purpose,
                    buffer: String::new(),
                };
            }
            // Mark selected item as done (move from in_progress → done)
            KeyCode::Char('d')
                if self.active_panel == ProgressPanel::InProgress =>
            {
                self.move_selected_to_done();
            }
            // Move selected item to in_progress (from next)
            KeyCode::Enter
                if self.active_panel == ProgressPanel::Next =>
            {
                self.move_selected_to_in_progress();
            }
            _ => {}
        }
    }

    /// Handle keys specific to Workflow tab
    fn handle_workflow_key(&mut self, key: KeyCode) {
        match key {
            // Advance phase
            KeyCode::Char('n') | KeyCode::Right => {
                if let Some(next) = self.current_phase.next() {
                    self.current_phase = next;
                    self.save_phase();
                    self.status_message = Some(format!("Advanced to {}", next.display_name()));
                }
            }
            // Retreat phase
            KeyCode::Char('p') | KeyCode::Left => {
                if let Some(prev) = self.current_phase.prev() {
                    self.current_phase = prev;
                    self.save_phase();
                    self.status_message = Some(format!("Retreated to {}", prev.display_name()));
                }
            }
            _ => {}
        }
    }

    /// Handle keys in editing mode
    fn handle_editing_key(&mut self, key: KeyCode) {
        let (purpose, buffer) = match &mut self.input_mode {
            InputMode::Editing { purpose, buffer } => (purpose.clone(), buffer),
            _ => return,
        };

        match key {
            KeyCode::Enter => {
                if !buffer.is_empty() {
                    let text = buffer.clone();
                    self.add_progress_item(&purpose, &text);
                }
                self.input_mode = InputMode::Normal;
            }
            KeyCode::Esc => {
                self.input_mode = InputMode::Normal;
            }
            KeyCode::Backspace => {
                buffer.pop();
            }
            KeyCode::Char(c) => {
                buffer.push(c);
            }
            _ => {}
        }
    }

    /// Add an item to progress and save
    fn add_progress_item(&mut self, purpose: &InputPurpose, text: &str) {
        if let Some(ref mut progress) = self.progress {
            progress.add_item(purpose.section_name(), text.to_string());
            let _ = self.save_progress();
            self.status_message = Some(format!("Added: {}", text));
        }
    }

    /// Move selected item from in_progress to done
    fn move_selected_to_done(&mut self) {
        if let Some(ref mut progress) = self.progress {
            if self.selected_index < progress.in_progress.len() {
                let item = progress.in_progress.remove(self.selected_index);
                progress.done.push(item);
                let _ = self.save_progress();
                self.status_message = Some("Moved to Done".to_string());
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }
        }
    }

    /// Move selected item from next to in_progress
    fn move_selected_to_in_progress(&mut self) {
        if let Some(ref mut progress) = self.progress {
            if self.selected_index < progress.next.len() {
                let item = progress.next.remove(self.selected_index);
                progress.in_progress.push(item);
                let _ = self.save_progress();
                self.status_message = Some("Moved to In Progress".to_string());
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }
        }
    }

    /// Save progress to file
    fn save_progress(&self) -> Result<()> {
        if let Some(ref progress) = self.progress {
            let path = self
                .config
                .as_ref()
                .map(|c| self.project_dir.join(&c.paths.progress))
                .unwrap_or_else(|| self.project_dir.join("docs/progress/PROGRESS.md"));
            progress.save(&path)?;
        }
        Ok(())
    }

    /// Save current phase to config
    fn save_phase(&self) {
        if let Ok(mut config) = Config::load(&self.project_dir) {
            if let Some(ref mut task) = config.current_task {
                task.phase = format!("{:?}", self.current_phase).to_lowercase();
                let _ = config.save(&self.project_dir);
            }
        }
    }
}
