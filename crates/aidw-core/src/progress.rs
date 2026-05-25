//! PROGRESS.md parsing and manipulation.
//!
//! Parses the structured PROGRESS.md format into Rust types,
//! allowing programmatic reading and writing.

use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProgressError {
    #[error("PROGRESS.md not found at {0}")]
    NotFound(String),
    #[error("Failed to read file: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Failed to parse PROGRESS.md: {0}")]
    ParseError(String),
}

/// Represents the full PROGRESS.md content
#[derive(Debug, Clone, Default)]
pub struct Progress {
    /// Project name (from header)
    pub project_name: String,
    /// Completed items
    pub done: Vec<ProgressItem>,
    /// In-progress items
    pub in_progress: Vec<ProgressItem>,
    /// Next up items
    pub next: Vec<ProgressItem>,
    /// Tech debt items
    pub tech_debt: Vec<ProgressItem>,
    /// Recent decisions
    pub decisions: Vec<String>,
    /// Raw content (for sections we don't parse)
    pub raw_content: String,
}

/// A single progress item (bullet point)
#[derive(Debug, Clone)]
pub struct ProgressItem {
    pub text: String,
    pub sub_items: Vec<String>,
}

impl Progress {
    /// Load and parse PROGRESS.md from a file path
    pub fn load(path: &Path) -> Result<Self, ProgressError> {
        if !path.exists() {
            return Err(ProgressError::NotFound(path.display().to_string()));
        }
        let content = std::fs::read_to_string(path)?;
        Self::parse(&content)
    }

    /// Parse PROGRESS.md content from a string
    pub fn parse(content: &str) -> Result<Self, ProgressError> {
        let mut progress = Progress {
            raw_content: content.to_string(),
            ..Default::default()
        };

        let mut current_section = Section::None;

        for line in content.lines() {
            let trimmed = line.trim();

            // Parse top-level header
            if trimmed.starts_with("# ") && !trimmed.starts_with("## ") {
                progress.project_name = trimmed[2..].to_string();
                continue;
            }

            // Detect section headers (## or ###)
            if trimmed.starts_with("## ") || trimmed.starts_with("### ") {
                let header = trimmed
                    .trim_start_matches('#')
                    .trim()
                    .to_lowercase();

                current_section = detect_section(&header);
                continue;
            }

            // Sub-items (indented lines starting with - or *) — check BEFORE top-level
            let indent = line.len() - line.trim_start().len();
            if indent >= 2 && (trimmed.starts_with("- ") || trimmed.starts_with("* ")) {
                let text = trimmed[2..].to_string();
                let items = match current_section {
                    Section::Done => &mut progress.done,
                    Section::InProgress => &mut progress.in_progress,
                    Section::Next => &mut progress.next,
                    Section::TechDebt => &mut progress.tech_debt,
                    _ => continue,
                };
                if let Some(last) = items.last_mut() {
                    last.sub_items.push(text);
                }
                continue;
            }

            // Top-level bullet items (no indentation)
            if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
                let text = trimmed[2..].to_string();
                let item = ProgressItem {
                    text,
                    sub_items: Vec::new(),
                };

                match current_section {
                    Section::Done => progress.done.push(item),
                    Section::InProgress => progress.in_progress.push(item),
                    Section::Next => progress.next.push(item),
                    Section::TechDebt => progress.tech_debt.push(item),
                    Section::Decisions => progress.decisions.push(item.text),
                    Section::None => {}
                }
            }
        }

        Ok(progress)
    }

    /// Generate PROGRESS.md content from the struct
    pub fn render(&self) -> String {
        let mut out = String::new();

        out.push_str(&format!("# {}\n\n", self.project_name));
        out.push_str("> Estado atual do projeto. Atualizado a cada sessão.\n\n---\n\n");

        // Done
        out.push_str("## ✅ Concluído\n\n");
        for item in &self.done {
            out.push_str(&format!("- {}\n", item.text));
            for sub in &item.sub_items {
                out.push_str(&format!("  - {}\n", sub));
            }
        }
        if self.done.is_empty() {
            out.push_str("- (nenhum item ainda)\n");
        }
        out.push('\n');

        // In Progress
        out.push_str("## 🚧 Em andamento\n\n");
        for item in &self.in_progress {
            out.push_str(&format!("- {}\n", item.text));
            for sub in &item.sub_items {
                out.push_str(&format!("  - {}\n", sub));
            }
        }
        if self.in_progress.is_empty() {
            out.push_str("- (nenhum item)\n");
        }
        out.push('\n');

        // Next
        out.push_str("## 🎯 Próximo\n\n");
        for item in &self.next {
            out.push_str(&format!("- {}\n", item.text));
            for sub in &item.sub_items {
                out.push_str(&format!("  - {}\n", sub));
            }
        }
        if self.next.is_empty() {
            out.push_str("- (nenhum item)\n");
        }
        out.push('\n');

        // Tech Debt
        out.push_str("## ⚠️ Débitos técnicos\n\n");
        for item in &self.tech_debt {
            out.push_str(&format!("- {}\n", item.text));
            for sub in &item.sub_items {
                out.push_str(&format!("  - {}\n", sub));
            }
        }
        if self.tech_debt.is_empty() {
            out.push_str("- (nenhum)\n");
        }
        out.push('\n');

        // Decisions
        out.push_str("## 📝 Decisões recentes\n\n");
        for d in &self.decisions {
            out.push_str(&format!("- {}\n", d));
        }
        if self.decisions.is_empty() {
            out.push_str("- (nenhuma)\n");
        }
        out.push('\n');

        out
    }

    /// Save rendered progress to a file
    pub fn save(&self, path: &Path) -> Result<(), ProgressError> {
        let content = self.render();
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Add an item to a section
    pub fn add_item(&mut self, section: &str, text: String) {
        let item = ProgressItem {
            text,
            sub_items: Vec::new(),
        };
        match section {
            "done" => self.done.push(item),
            "in_progress" | "in-progress" => self.in_progress.push(item),
            "next" => self.next.push(item),
            "tech_debt" | "debt" => self.tech_debt.push(item),
            "decisions" => self.decisions.push(item.text),
            _ => {}
        }
    }

    /// Move an item from one section to another by text match
    pub fn move_item(&mut self, text: &str, from: &str, to: &str) -> bool {
        let source = match from {
            "done" => &mut self.done,
            "in_progress" | "in-progress" => &mut self.in_progress,
            "next" => &mut self.next,
            "tech_debt" | "debt" => &mut self.tech_debt,
            _ => return false,
        };

        let pos = source.iter().position(|item| item.text.contains(text));
        if let Some(idx) = pos {
            let item = source.remove(idx);
            let target = match to {
                "done" => &mut self.done,
                "in_progress" | "in-progress" => &mut self.in_progress,
                "next" => &mut self.next,
                "tech_debt" | "debt" => &mut self.tech_debt,
                _ => return false,
            };
            target.push(item);
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Section {
    None,
    Done,
    InProgress,
    Next,
    TechDebt,
    Decisions,
}

/// Detect which section a header belongs to.
/// Uses a scoring approach to handle Unicode edge cases with accented chars and emojis.
fn detect_section(header: &str) -> Section {
    // Normalize: strip non-ASCII for keyword matching
    let ascii_lower: String = header
        .chars()
        .filter(|c| c.is_ascii() || *c == ' ')
        .collect::<String>()
        .to_lowercase();

    // Check decisions FIRST (before debt) to avoid false matches
    // "Decisões" → ascii filtered = "decises" which contains "decis"
    if header.contains('\u{1F4DD}') // 📝
        || ascii_lower.contains("decis")
        || ascii_lower.contains("decision")
    {
        return Section::Decisions;
    }

    // Done / Completed
    if header.contains('\u{2705}') // ✅
        || ascii_lower.contains("conclu")
        || ascii_lower.contains("done")
        || ascii_lower.contains("completed")
    {
        return Section::Done;
    }

    // In Progress
    if header.contains('\u{1F6A7}') // 🚧
        || ascii_lower.contains("andamento")
        || ascii_lower.contains("progress")
        || ascii_lower.contains("in progress")
    {
        return Section::InProgress;
    }

    // Next
    if header.contains('\u{1F3AF}') // 🎯
        || ascii_lower.contains("prximo") // próximo without accent
        || ascii_lower.contains("next")
    {
        return Section::Next;
    }

    // Tech Debt
    if header.contains('\u{26A0}') // ⚠
        || ascii_lower.contains("dbit") // débito without accent
        || ascii_lower.contains("debt")
        || ascii_lower.contains("tcnic") // técnico without accent
    {
        return Section::TechDebt;
    }

    Section::None
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_PROGRESS: &str = r#"# My Project

> Estado atual do projeto. Atualizado a cada sessão.

---

## ✅ Concluído

- Setup inicial do projeto
  - Configurou Next.js 15
  - Adicionou Tailwind
- Implementou autenticação

## 🚧 Em andamento

- Sistema de notificações
  - Backend pronto
  - Falta frontend

## 🎯 Próximo

- Dashboard de métricas
- Exportação CSV

## ⚠️ Débitos técnicos

- Migrar de REST para tRPC
- Adicionar testes E2E

## 📝 Decisões recentes

- Usar Drizzle ORM em vez de Prisma (performance)
- Adotar server actions para mutations
"#;

    #[test]
    fn test_parse_progress() {
        let progress = Progress::parse(SAMPLE_PROGRESS).unwrap();
        assert_eq!(progress.project_name, "My Project");
        assert_eq!(progress.done.len(), 2);
        assert_eq!(progress.in_progress.len(), 1);
        assert_eq!(progress.next.len(), 2);
        assert_eq!(progress.tech_debt.len(), 2);
        assert_eq!(progress.decisions.len(), 2);

        // Check sub-items
        assert_eq!(progress.done[0].sub_items.len(), 2);
        assert_eq!(progress.in_progress[0].sub_items.len(), 2);
    }

    #[test]
    fn test_add_item() {
        let mut progress = Progress::parse(SAMPLE_PROGRESS).unwrap();
        progress.add_item("next", "New feature X".to_string());
        assert_eq!(progress.next.len(), 3);
        assert_eq!(progress.next[2].text, "New feature X");
    }

    #[test]
    fn test_move_item() {
        let mut progress = Progress::parse(SAMPLE_PROGRESS).unwrap();
        let moved = progress.move_item("Dashboard", "next", "in_progress");
        assert!(moved);
        assert_eq!(progress.next.len(), 1);
        assert_eq!(progress.in_progress.len(), 2);
    }

    #[test]
    fn test_render_roundtrip() {
        let progress = Progress::parse(SAMPLE_PROGRESS).unwrap();
        let rendered = progress.render();
        let reparsed = Progress::parse(&rendered).unwrap();
        assert_eq!(reparsed.done.len(), progress.done.len());
        assert_eq!(reparsed.in_progress.len(), progress.in_progress.len());
        assert_eq!(reparsed.next.len(), progress.next.len());
    }
}
