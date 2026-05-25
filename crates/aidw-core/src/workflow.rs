//! Workflow phase tracking.
//!
//! Tracks the current phase of the 7-phase workflow:
//! Context -> Design -> Plan -> Execute -> Verify -> Document -> Handoff

use serde::{Deserialize, Serialize};

/// The 7 phases of the ai-dev-workflow
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Phase {
    Context,
    Design,
    Plan,
    Execute,
    Verify,
    Document,
    Handoff,
}

impl Phase {
    /// All phases in order
    pub fn all() -> &'static [Phase] {
        &[
            Phase::Context,
            Phase::Design,
            Phase::Plan,
            Phase::Execute,
            Phase::Verify,
            Phase::Document,
            Phase::Handoff,
        ]
    }

    /// Get the next phase (None if at Handoff)
    pub fn next(self) -> Option<Phase> {
        match self {
            Phase::Context => Some(Phase::Design),
            Phase::Design => Some(Phase::Plan),
            Phase::Plan => Some(Phase::Execute),
            Phase::Execute => Some(Phase::Verify),
            Phase::Verify => Some(Phase::Document),
            Phase::Document => Some(Phase::Handoff),
            Phase::Handoff => None,
        }
    }

    /// Get the previous phase (None if at Context)
    pub fn prev(self) -> Option<Phase> {
        match self {
            Phase::Context => None,
            Phase::Design => Some(Phase::Context),
            Phase::Plan => Some(Phase::Design),
            Phase::Execute => Some(Phase::Plan),
            Phase::Verify => Some(Phase::Execute),
            Phase::Document => Some(Phase::Verify),
            Phase::Handoff => Some(Phase::Document),
        }
    }

    /// Get the display name
    pub fn display_name(self) -> &'static str {
        match self {
            Phase::Context => "CONTEXT",
            Phase::Design => "DESIGN",
            Phase::Plan => "PLAN",
            Phase::Execute => "EXECUTE",
            Phase::Verify => "VERIFY",
            Phase::Document => "DOCUMENT",
            Phase::Handoff => "HANDOFF",
        }
    }

    /// Get description of what this phase does
    pub fn description(self) -> &'static str {
        match self {
            Phase::Context => "Understand what and why — read PROGRESS.md, stories, ADRs",
            Phase::Design => "Decide how — trade-offs, ADR, threat model",
            Phase::Plan => "Break into steps — TODO list, files, criteria",
            Phase::Execute => "Implement — schemas, data, logic, UI, tests",
            Phase::Verify => "Prove it works — lint, typecheck, test, build",
            Phase::Document => "Leave a trail — feature doc, PROGRESS.md, ADR",
            Phase::Handoff => "Deliver — DoD check, summary, next suggestion",
        }
    }

    /// Get the index (0-based) of this phase
    pub fn index(self) -> usize {
        match self {
            Phase::Context => 0,
            Phase::Design => 1,
            Phase::Plan => 2,
            Phase::Execute => 3,
            Phase::Verify => 4,
            Phase::Document => 5,
            Phase::Handoff => 6,
        }
    }

    /// Parse from string
    pub fn parse(s: &str) -> Option<Phase> {
        match s.to_lowercase().as_str() {
            "context" => Some(Phase::Context),
            "design" => Some(Phase::Design),
            "plan" => Some(Phase::Plan),
            "execute" => Some(Phase::Execute),
            "verify" => Some(Phase::Verify),
            "document" | "doc" => Some(Phase::Document),
            "handoff" => Some(Phase::Handoff),
            _ => None,
        }
    }

    /// Render a visual progress bar showing current phase
    pub fn render_bar(self) -> String {
        let phases = Phase::all();
        let mut parts = Vec::new();
        for (i, phase) in phases.iter().enumerate() {
            if *phase == self {
                parts.push(format!("[{}]", phase.display_name()));
            } else {
                parts.push(phase.display_name().to_string());
            }
            if i < phases.len() - 1 {
                parts.push(" → ".to_string());
            }
        }
        parts.concat()
    }
}

impl std::fmt::Display for Phase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Current workflow state for a task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowState {
    pub task_id: String,
    pub task_title: String,
    pub current_phase: Phase,
    pub started_at: String,
    pub phase_history: Vec<PhaseTransition>,
}

/// Record of a phase transition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseTransition {
    pub from: Phase,
    pub to: Phase,
    pub timestamp: String,
}

impl WorkflowState {
    /// Create a new workflow state starting at Context
    pub fn new(task_id: String, task_title: String) -> Self {
        Self {
            task_id,
            task_title,
            current_phase: Phase::Context,
            started_at: chrono::Utc::now().to_rfc3339(),
            phase_history: Vec::new(),
        }
    }

    /// Advance to the next phase
    pub fn advance(&mut self) -> Option<Phase> {
        if let Some(next) = self.current_phase.next() {
            self.phase_history.push(PhaseTransition {
                from: self.current_phase,
                to: next,
                timestamp: chrono::Utc::now().to_rfc3339(),
            });
            self.current_phase = next;
            Some(next)
        } else {
            None
        }
    }

    /// Go back to previous phase (e.g., from Verify back to Execute)
    pub fn retreat(&mut self) -> Option<Phase> {
        if let Some(prev) = self.current_phase.prev() {
            self.phase_history.push(PhaseTransition {
                from: self.current_phase,
                to: prev,
                timestamp: chrono::Utc::now().to_rfc3339(),
            });
            self.current_phase = prev;
            Some(prev)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_order() {
        let mut phase = Phase::Context;
        let expected = [
            Phase::Design,
            Phase::Plan,
            Phase::Execute,
            Phase::Verify,
            Phase::Document,
            Phase::Handoff,
        ];
        for expected_next in expected {
            phase = phase.next().unwrap();
            assert_eq!(phase, expected_next);
        }
        assert_eq!(phase.next(), None);
    }

    #[test]
    fn test_phase_bar() {
        let bar = Phase::Execute.render_bar();
        assert!(bar.contains("[EXECUTE]"));
        assert!(bar.contains("CONTEXT"));
        assert!(!bar.contains("[CONTEXT]"));
    }

    #[test]
    fn test_workflow_state() {
        let mut state = WorkflowState::new("US-001".to_string(), "Test task".to_string());
        assert_eq!(state.current_phase, Phase::Context);

        state.advance();
        assert_eq!(state.current_phase, Phase::Design);
        assert_eq!(state.phase_history.len(), 1);

        state.retreat();
        assert_eq!(state.current_phase, Phase::Context);
        assert_eq!(state.phase_history.len(), 2);
    }
}
