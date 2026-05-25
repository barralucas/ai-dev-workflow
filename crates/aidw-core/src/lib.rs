//! aidw-core: Core library for the ai-dev-workflow CLI tool.
//!
//! Provides:
//! - Configuration management (.aidw.toml)
//! - Stack detection
//! - Template management (embedded templates)
//! - PROGRESS.md parsing and manipulation
//! - ADR management
//! - Workflow phase tracking

pub mod config;
pub mod detect;
pub mod progress;
pub mod templates;
pub mod workflow;

pub use config::Config;
pub use detect::{Stack, StackDetector};
pub use progress::Progress;
pub use workflow::{Phase, WorkflowState};
