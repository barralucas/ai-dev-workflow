//! `aidw adr` — Architecture Decision Records management

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

use aidw_core::Config;

use crate::AdrAction;

pub fn run(project_dir: PathBuf, action: Option<AdrAction>) -> Result<()> {
    match action {
        None | Some(AdrAction::List) => list(&project_dir),
        Some(AdrAction::New { title }) => create(&project_dir, &title),
    }
}

fn get_adr_dir(project_dir: &Path) -> PathBuf {
    Config::load(project_dir)
        .map(|c| project_dir.join(&c.paths.adr_dir))
        .unwrap_or_else(|_| project_dir.join("docs/adr"))
}

fn list(project_dir: &Path) -> Result<()> {
    let adr_dir = get_adr_dir(project_dir);

    if !adr_dir.exists() {
        println!("  No ADR directory found at: {}", adr_dir.display());
        println!("  Run `aidw init` or create docs/adr/ manually.");
        return Ok(());
    }

    let mut entries: Vec<_> = std::fs::read_dir(&adr_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().extension().is_some_and(|ext| ext == "md")
                && e.file_name().to_string_lossy() != "0000-template.md"
        })
        .collect();

    entries.sort_by_key(|e| e.file_name());

    if entries.is_empty() {
        println!("  No ADRs found. Use `aidw adr new \"<title>\"` to create one.");
        return Ok(());
    }

    println!("  Architecture Decision Records:");
    println!();
    for entry in &entries {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        // Try to extract status from file content
        let status = std::fs::read_to_string(entry.path())
            .ok()
            .and_then(|content| {
                content.lines().find_map(|line| {
                    if line.to_lowercase().contains("status:") || line.to_lowercase().contains("## status") {
                        Some(line.trim().to_string())
                    } else {
                        None
                    }
                })
            })
            .unwrap_or_default();

        println!("  • {} {}", name_str.trim_end_matches(".md"), status);
    }
    println!();

    Ok(())
}

fn create(project_dir: &Path, title: &str) -> Result<()> {
    let adr_dir = get_adr_dir(project_dir);
    std::fs::create_dir_all(&adr_dir)?;

    // Find next number
    let next_num = std::fs::read_dir(&adr_dir)?
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            name.split('-').next()?.parse::<u32>().ok()
        })
        .max()
        .unwrap_or(0)
        + 1;

    // Slugify title
    let slug: String = title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() || c == ' ' { c } else { '-' })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("-");

    let filename = format!("{:04}-{}.md", next_num, slug);
    let filepath = adr_dir.join(&filename);

    let date = chrono::Utc::now().format("%Y-%m-%d").to_string();

    let content = format!(
        r#"# ADR-{:04}: {}

**Data**: {}
**Status**: Proposed

## Contexto

<!-- Qual problema estamos resolvendo? Que restrições existem? -->

## Opções consideradas

### Opção 1: (descreva)

**Prós:**
-

**Contras:**
-

### Opção 2: (descreva)

**Prós:**
-

**Contras:**
-

## Decisão

<!-- Qual opção escolhida e por quê -->

## Consequências

### Positivas
-

### Negativas
-

### Riscos
-
"#,
        next_num, title, date
    );

    std::fs::write(&filepath, content)
        .with_context(|| format!("Failed to write ADR to {}", filepath.display()))?;

    println!("  ✓ Created ADR: {}", filename);
    println!("  Path: {}", filepath.display());
    println!();
    println!("  Edit the file to fill in context, options, and decision.");

    Ok(())
}
