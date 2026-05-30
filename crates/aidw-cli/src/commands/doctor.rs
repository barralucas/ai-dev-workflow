//! `aidw doctor` — Project health check

use anyhow::Result;
use std::path::Path;
use std::process::Command;

use aidw_core::Config;

struct Check {
    name: &'static str,
    passed: bool,
    message: String,
}

pub fn run(project_dir: &Path) -> Result<()> {
    println!("  Checking project health...");
    println!();

    let mut checks: Vec<Check> = Vec::new();

    // 1. .aidw.toml exists and is valid
    let config = match Config::load(project_dir) {
        Ok(c) => {
            checks.push(Check {
                name: ".aidw.toml",
                passed: true,
                message: format!("Valid (project: {}, stack: {})", c.project.name, c.project.stack),
            });
            Some(c)
        }
        Err(e) => {
            checks.push(Check {
                name: ".aidw.toml",
                passed: false,
                message: format!("{}. Run `aidw init` to create it.", e),
            });
            None
        }
    };

    // 2. PROGRESS.md exists
    let progress_path = config
        .as_ref()
        .map(|c| project_dir.join(&c.paths.progress))
        .unwrap_or_else(|| project_dir.join("docs/progress/PROGRESS.md"));

    if progress_path.exists() {
        checks.push(Check {
            name: "PROGRESS.md",
            passed: true,
            message: format!("Found at {}", progress_path.strip_prefix(project_dir).unwrap_or(&progress_path).display()),
        });
    } else {
        checks.push(Check {
            name: "PROGRESS.md",
            passed: false,
            message: format!("Not found at {}. Run `aidw init` to create it.", progress_path.strip_prefix(project_dir).unwrap_or(&progress_path).display()),
        });
    }

    // 3. ADR directory exists
    let adr_dir = config
        .as_ref()
        .map(|c| project_dir.join(&c.paths.adr_dir))
        .unwrap_or_else(|| project_dir.join("docs/adr"));

    if adr_dir.exists() {
        let adr_count = std::fs::read_dir(&adr_dir)
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .filter(|e| {
                        e.path().extension().is_some_and(|ext| ext == "md")
                            && e.file_name().to_string_lossy() != "0000-template.md"
                    })
                    .count()
            })
            .unwrap_or(0);
        checks.push(Check {
            name: "ADR directory",
            passed: true,
            message: format!("Found ({} ADRs)", adr_count),
        });
    } else {
        checks.push(Check {
            name: "ADR directory",
            passed: false,
            message: "Not found. Run `aidw init` to create docs/adr/".to_string(),
        });
    }

    // 4. Git repository
    if project_dir.join(".git").exists() {
        checks.push(Check {
            name: "Git repository",
            passed: true,
            message: "Initialized".to_string(),
        });
    } else {
        checks.push(Check {
            name: "Git repository",
            passed: false,
            message: "Not a git repo. Run `git init`".to_string(),
        });
    }

    // 5. Check configured commands are available
    if let Some(ref config) = config {
        let cmds = [
            ("lint", &config.commands.lint),
            ("typecheck", &config.commands.typecheck),
            ("test", &config.commands.test),
            ("build", &config.commands.build),
        ];

        for (name, cmd_opt) in &cmds {
            match cmd_opt {
                Some(cmd) => {
                    // Extract the base command (first word)
                    let base_cmd = cmd.split_whitespace().next().unwrap_or(cmd);
                    let available = Command::new("which")
                        .arg(base_cmd)
                        .output()
                        .map(|o| o.status.success())
                        .unwrap_or(false);

                    if available {
                        checks.push(Check {
                            name: Box::leak(format!("cmd:{}", name).into_boxed_str()),
                            passed: true,
                            message: format!("`{}` — available", cmd),
                        });
                    } else {
                        checks.push(Check {
                            name: Box::leak(format!("cmd:{}", name).into_boxed_str()),
                            passed: false,
                            message: format!("`{}` — '{}' not found in PATH", cmd, base_cmd),
                        });
                    }
                }
                None => {
                    checks.push(Check {
                        name: Box::leak(format!("cmd:{}", name).into_boxed_str()),
                        passed: true,
                        message: "Not configured (optional)".to_string(),
                    });
                }
            }
        }
    }

    // 6. AGENTS.md exists
    if project_dir.join("AGENTS.md").exists() {
        checks.push(Check {
            name: "AGENTS.md",
            passed: true,
            message: "Found".to_string(),
        });
    } else {
        checks.push(Check {
            name: "AGENTS.md",
            passed: false,
            message: "Not found. Agents won't have an entry point.".to_string(),
        });
    }

    // Display results
    let total = checks.len();
    let passed = checks.iter().filter(|c| c.passed).count();
    let failed = total - passed;

    for check in &checks {
        let icon = if check.passed { "✓" } else { "✗" };
        println!("  {} {:14} {}", icon, check.name, check.message);
    }

    println!();
    if failed == 0 {
        println!("  ✅ All {} checks passed. Project is healthy!", total);
    } else {
        println!(
            "  ⚠️  {}/{} checks passed, {} issues found.",
            passed, total, failed
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::init;

    #[test]
    fn test_doctor_runs_on_initialized_project() {
        let dir = tempfile::tempdir().unwrap();
        init::run(
            dir.path().to_path_buf(),
            Some("Doctor Harness".to_string()),
            Some("rust".to_string()),
            true,
        )
        .unwrap();

        run(dir.path()).unwrap();
    }
}
