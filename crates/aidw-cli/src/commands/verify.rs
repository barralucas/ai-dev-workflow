//! `aidw verify` — Run quality gate pipeline (lint, typecheck, test, build)

use anyhow::Result;
use std::path::Path;
use std::process::Command;
use std::time::Instant;

use aidw_core::Config;

pub fn run(project_dir: &Path, step: Option<&str>) -> Result<()> {
    let config = Config::load(project_dir).map_err(|_| {
        anyhow::anyhow!("No .aidw.toml found. Run `aidw init` or `aidw adopt` first.")
    })?;

    let gates: Vec<(&str, Option<&String>)> = vec![
        ("lint", config.commands.lint.as_ref()),
        ("typecheck", config.commands.typecheck.as_ref()),
        ("test", config.commands.test.as_ref()),
        ("build", config.commands.build.as_ref()),
    ];

    // If a specific step is requested, run only that one
    if let Some(step_name) = step {
        let gate = gates.iter().find(|(name, _)| *name == step_name);
        match gate {
            Some((name, Some(cmd))) => {
                return run_gate(name, cmd, project_dir);
            }
            Some((name, None)) => {
                println!("  ⚠ {} — not configured in .aidw.toml", name);
                return Ok(());
            }
            None => {
                return Err(anyhow::anyhow!(
                    "Unknown gate '{}'. Valid: lint, typecheck, test, build",
                    step_name
                ));
            }
        }
    }

    // Run all configured gates sequentially
    println!("  Running quality gates...");
    println!();

    let total_start = Instant::now();
    let mut passed = 0;
    let mut skipped = 0;

    for (name, cmd) in &gates {
        match cmd {
            Some(command) => match run_gate(name, command, project_dir) {
                Ok(()) => passed += 1,
                Err(e) => {
                    println!();
                    println!("  ✗ Pipeline FAILED at '{}': {}", name, e);
                    println!();
                    println!("  Fix the issue and run `aidw verify` again.");
                    println!("  Or run just this step: `aidw verify {}`", name);
                    std::process::exit(1);
                }
            },
            None => {
                println!("  ⊘ {} — skipped (not configured)", name);
                skipped += 1;
            }
        }
    }

    let elapsed = total_start.elapsed();
    println!();
    println!(
        "  ✅ All gates passed! ({} passed, {} skipped) [{:.1}s]",
        passed,
        skipped,
        elapsed.as_secs_f64()
    );

    Ok(())
}

fn run_gate(name: &str, command: &str, project_dir: &Path) -> Result<()> {
    let start = Instant::now();
    print!("  ▸ {} — `{}`", name, command);

    // Use shell to run the command (supports pipes, &&, etc.)
    let output = Command::new("sh")
        .args(["-c", command])
        .current_dir(project_dir)
        .output()?;

    let elapsed = start.elapsed();

    if output.status.success() {
        println!(" ✓ [{:.1}s]", elapsed.as_secs_f64());
        Ok(())
    } else {
        println!(" ✗ [{:.1}s]", elapsed.as_secs_f64());
        // Print stderr/stdout for context
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        if !stdout.is_empty() {
            let lines: Vec<&str> = stdout.lines().take(20).collect();
            for line in lines {
                println!("    │ {}", line);
            }
        }
        if !stderr.is_empty() {
            let lines: Vec<&str> = stderr.lines().take(20).collect();
            for line in lines {
                println!("    │ {}", line);
            }
        }
        Err(anyhow::anyhow!("Command `{}` exited with status {}", command, output.status))
    }
}
