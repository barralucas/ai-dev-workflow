//! aidw-tui: Terminal User Interface for ai-dev-workflow.
//!
//! Provides an interactive TUI using ratatui for:
//! - Dashboard overview (progress, phase, config)
//! - PROGRESS.md viewer/editor
//! - Workflow phase visualization
//! - ADR browser

pub mod app;
pub mod ui;

pub use app::App;
