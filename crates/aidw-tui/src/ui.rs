//! UI rendering for the TUI.

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Tabs, Wrap},
};

use crate::app::{App, InputMode, ProgressPanel, Tab};
use aidw_core::Phase;

/// Main render function
pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header + tabs
            Constraint::Min(1),   // Main content
            Constraint::Length(3), // Footer / help
        ])
        .split(frame.area());

    render_header(frame, app, chunks[0]);
    render_content(frame, app, chunks[1]);
    render_footer(frame, app, chunks[2]);

    // Render input popup overlay if in editing mode
    if let InputMode::Editing { purpose, buffer } = &app.input_mode {
        render_input_popup(frame, purpose.label(), buffer);
    }
}

/// Render the header with tabs
fn render_header(frame: &mut Frame, app: &App, area: Rect) {
    let titles: Vec<Line> = Tab::all()
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let label = format!(" {}:{} ", i + 1, t.title());
            if *t == app.active_tab {
                Line::from(label).bold()
            } else {
                Line::from(label)
            }
        })
        .collect();

    let status_suffix = app
        .status_message
        .as_deref()
        .map(|m| format!(" │ {} ", m))
        .unwrap_or_default();

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" aidw — AI Dev Workflow{} ", status_suffix)),
        )
        .highlight_style(Style::default().fg(Color::Cyan).bold())
        .select(Tab::all().iter().position(|t| *t == app.active_tab).unwrap_or(0));

    frame.render_widget(tabs, area);
}

/// Render the main content based on active tab
fn render_content(frame: &mut Frame, app: &App, area: Rect) {
    match app.active_tab {
        Tab::Progress => render_progress_tab(frame, app, area),
        Tab::Workflow => render_workflow_tab(frame, app, area),
        Tab::Info => render_info_tab(frame, app, area),
    }
}

/// Render the progress tab (PROGRESS.md content)
fn render_progress_tab(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Left panel: In Progress + Next
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[0]);

    // Right panel: Done + Tech Debt
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    if let Some(progress) = &app.progress {
        // In Progress
        let is_active = app.active_panel == ProgressPanel::InProgress;
        let in_progress_items: Vec<ListItem> = progress
            .in_progress
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let prefix = if is_active && i == app.selected_index {
                    "▸ "
                } else {
                    "  "
                };
                let style = if is_active && i == app.selected_index {
                    Style::default().fg(Color::Yellow).bold()
                } else {
                    Style::default()
                };
                ListItem::new(format!("{}{}", prefix, item.text)).style(style)
            })
            .collect();

        let in_progress_list = List::new(if in_progress_items.is_empty() {
            vec![ListItem::new("  (no items)").style(Style::default().fg(Color::DarkGray))]
        } else {
            in_progress_items
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" 🚧 In Progress ")
                .border_style(if is_active {
                    Style::default().fg(Color::Yellow).bold()
                } else {
                    Style::default().fg(Color::Yellow)
                }),
        );
        frame.render_widget(in_progress_list, left_chunks[0]);

        // Next
        let is_active = app.active_panel == ProgressPanel::Next;
        let next_items: Vec<ListItem> = progress
            .next
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let prefix = if is_active && i == app.selected_index {
                    "▸ "
                } else {
                    "  "
                };
                let style = if is_active && i == app.selected_index {
                    Style::default().fg(Color::Cyan).bold()
                } else {
                    Style::default()
                };
                ListItem::new(format!("{}{}", prefix, item.text)).style(style)
            })
            .collect();

        let next_list = List::new(if next_items.is_empty() {
            vec![ListItem::new("  (no items)").style(Style::default().fg(Color::DarkGray))]
        } else {
            next_items
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" 🎯 Next ")
                .border_style(if is_active {
                    Style::default().fg(Color::Blue).bold()
                } else {
                    Style::default().fg(Color::Blue)
                }),
        );
        frame.render_widget(next_list, left_chunks[1]);

        // Done
        let is_active = app.active_panel == ProgressPanel::Done;
        let done_items: Vec<ListItem> = progress
            .done
            .iter()
            .rev()
            .take(10)
            .enumerate()
            .map(|(i, item)| {
                let prefix = if is_active && i == app.selected_index {
                    "▸ "
                } else {
                    "  "
                };
                let style = if is_active && i == app.selected_index {
                    Style::default().fg(Color::Green).bold()
                } else {
                    Style::default()
                };
                ListItem::new(format!("{}{}", prefix, item.text)).style(style)
            })
            .collect();

        let done_list = List::new(if done_items.is_empty() {
            vec![ListItem::new("  (no items)").style(Style::default().fg(Color::DarkGray))]
        } else {
            done_items
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" ✅ Done (recent) ")
                .border_style(if is_active {
                    Style::default().fg(Color::Green).bold()
                } else {
                    Style::default().fg(Color::Green)
                }),
        );
        frame.render_widget(done_list, right_chunks[0]);

        // Tech Debt
        let is_active = app.active_panel == ProgressPanel::Debt;
        let debt_items: Vec<ListItem> = progress
            .tech_debt
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let prefix = if is_active && i == app.selected_index {
                    "▸ "
                } else {
                    "  "
                };
                let style = if is_active && i == app.selected_index {
                    Style::default().fg(Color::Red).bold()
                } else {
                    Style::default()
                };
                ListItem::new(format!("{}{}", prefix, item.text)).style(style)
            })
            .collect();

        let debt_list = List::new(if debt_items.is_empty() {
            vec![ListItem::new("  (no items)").style(Style::default().fg(Color::DarkGray))]
        } else {
            debt_items
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" ⚠️  Tech Debt ")
                .border_style(if is_active {
                    Style::default().fg(Color::Red).bold()
                } else {
                    Style::default().fg(Color::Red)
                }),
        );
        frame.render_widget(debt_list, right_chunks[1]);
    } else {
        let msg = Paragraph::new("No PROGRESS.md found.\nRun `aidw init` to set up the project.")
            .block(Block::default().borders(Borders::ALL).title(" Progress "))
            .style(Style::default().fg(Color::DarkGray));
        frame.render_widget(msg, chunks[0]);

        let msg2 = Paragraph::new("")
            .block(Block::default().borders(Borders::ALL).title(" Details "));
        frame.render_widget(msg2, chunks[1]);
    }
}

/// Render the workflow phase tab
fn render_workflow_tab(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Phase bar
            Constraint::Min(1),   // Phase details
        ])
        .split(area);

    // Phase progress bar
    let phase_bar = render_phase_bar(app.current_phase);
    let phase_widget = Paragraph::new(phase_bar)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Current Phase "),
        )
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });
    frame.render_widget(phase_widget, chunks[0]);

    // Phase details
    let phases: Vec<ListItem> = Phase::all()
        .iter()
        .map(|phase| {
            let marker = if *phase == app.current_phase {
                "▶ "
            } else if phase.index() < app.current_phase.index() {
                "✓ "
            } else {
                "  "
            };
            let style = if *phase == app.current_phase {
                Style::default().fg(Color::Cyan).bold()
            } else if phase.index() < app.current_phase.index() {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            ListItem::new(format!(
                "  {}{} — {}",
                marker,
                phase.display_name(),
                phase.description()
            ))
            .style(style)
        })
        .collect();

    let task_title = app
        .config
        .as_ref()
        .and_then(|c| c.current_task.as_ref())
        .map(|t| format!(" Task: {} - {} ", t.id, t.title))
        .unwrap_or_else(|| " No active task ".to_string());

    let phases_list = List::new(phases).block(
        Block::default()
            .borders(Borders::ALL)
            .title(task_title),
    );
    frame.render_widget(phases_list, chunks[1]);
}

/// Render the info/config tab
fn render_info_tab(frame: &mut Frame, app: &App, area: Rect) {
    let info = if let Some(config) = &app.config {
        let mut lines = vec![
            format!("  Project:  {}", config.project.name),
            format!("  Stack:    {}", config.project.stack),
            format!("  Language: {}", config.project.language),
            String::new(),
            "  Commands:".to_string(),
        ];

        if let Some(ref cmd) = config.commands.lint {
            lines.push(format!("    lint:      {}", cmd));
        }
        if let Some(ref cmd) = config.commands.typecheck {
            lines.push(format!("    typecheck: {}", cmd));
        }
        if let Some(ref cmd) = config.commands.test {
            lines.push(format!("    test:      {}", cmd));
        }
        if let Some(ref cmd) = config.commands.build {
            lines.push(format!("    build:     {}", cmd));
        }

        lines.push(String::new());
        lines.push("  Paths:".to_string());
        lines.push(format!("    progress: {}", config.paths.progress));
        lines.push(format!("    adr_dir:  {}", config.paths.adr_dir));
        lines.push(format!("    features: {}", config.paths.features_dir));

        lines.join("\n")
    } else {
        "  No .aidw.toml found in this project.\n\n  Run `aidw init` to initialize the workflow."
            .to_string()
    };

    let info_widget = Paragraph::new(info)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" Project: {} ", app.project_dir.display())),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(info_widget, area);
}

/// Render the footer with context-sensitive key bindings
fn render_footer(frame: &mut Frame, app: &App, area: Rect) {
    let help = match &app.input_mode {
        InputMode::Editing { .. } => {
            " Enter: confirm | Esc: cancel ".to_string()
        }
        InputMode::Normal => match app.active_tab {
            Tab::Progress => {
                let panel_help = match app.active_panel {
                    ProgressPanel::InProgress => "d=done",
                    ProgressPanel::Next => "Enter=start",
                    ProgressPanel::Done => "",
                    ProgressPanel::Debt => "",
                };
                if panel_help.is_empty() {
                    " Tab: tabs | h/l: panels | j/k: nav | a: add | q: quit ".to_string()
                } else {
                    format!(
                        " Tab: tabs | h/l: panels | j/k: nav | a: add | {} | q: quit ",
                        panel_help
                    )
                }
            }
            Tab::Workflow => {
                " Tab: tabs | n/→: next phase | p/←: prev phase | q: quit ".to_string()
            }
            Tab::Info => " Tab: tabs | q: quit ".to_string(),
        },
    };

    let footer = Paragraph::new(help)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    frame.render_widget(footer, area);
}

/// Render an input popup overlay
fn render_input_popup(frame: &mut Frame, title: &str, buffer: &str) {
    let area = frame.area();
    // Center popup: 60 cols wide, 5 rows tall
    let popup_width = 60u16.min(area.width.saturating_sub(4));
    let popup_height = 5u16;
    let x = (area.width.saturating_sub(popup_width)) / 2;
    let y = (area.height.saturating_sub(popup_height)) / 2;
    let popup_area = Rect::new(x, y, popup_width, popup_height);

    // Clear the area behind the popup
    frame.render_widget(Clear, popup_area);

    let input_text = format!(" > {}█", buffer);
    let popup = Paragraph::new(input_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" {} ", title))
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .style(Style::default().fg(Color::White));
    frame.render_widget(popup, popup_area);
}

/// Create a visual phase progress bar
fn render_phase_bar(current: Phase) -> String {
    let phases = Phase::all();
    let mut parts = Vec::new();

    for (i, phase) in phases.iter().enumerate() {
        if *phase == current {
            parts.push(format!("[{}]", phase.display_name()));
        } else if phase.index() < current.index() {
            parts.push(format!("({})", phase.display_name()));
        } else {
            parts.push(format!(" {} ", phase.display_name()));
        }
        if i < phases.len() - 1 {
            parts.push(" → ".to_string());
        }
    }

    format!("\n{}", parts.concat())
}
