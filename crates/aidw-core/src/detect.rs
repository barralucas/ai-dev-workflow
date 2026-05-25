//! Stack detection engine.
//!
//! Analyzes a project directory to determine which technology stack is in use.

use std::path::Path;

/// Supported technology stacks
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stack {
    NextJs,
    NodeBackend,
    Python,
    Mobile,
    Rust,
    Go,
    Unknown,
}

impl Stack {
    /// Get the string identifier used in config files
    pub fn as_str(&self) -> &'static str {
        match self {
            Stack::NextJs => "nextjs",
            Stack::NodeBackend => "node-backend",
            Stack::Python => "python",
            Stack::Mobile => "mobile",
            Stack::Rust => "rust",
            Stack::Go => "go",
            Stack::Unknown => "none",
        }
    }

    /// Get a human-readable display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Stack::NextJs => "Next.js",
            Stack::NodeBackend => "Node.js Backend",
            Stack::Python => "Python",
            Stack::Mobile => "Mobile (React Native/Expo)",
            Stack::Rust => "Rust",
            Stack::Go => "Go",
            Stack::Unknown => "Unknown / Multi-stack",
        }
    }

    /// Parse from string identifier
    pub fn parse(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "nextjs" | "next" => Stack::NextJs,
            "node-backend" | "node" | "express" | "fastify" | "nest" => Stack::NodeBackend,
            "python" | "fastapi" | "django" | "flask" => Stack::Python,
            "mobile" | "react-native" | "expo" => Stack::Mobile,
            "rust" => Stack::Rust,
            "go" | "golang" => Stack::Go,
            _ => Stack::Unknown,
        }
    }

    /// Get all known stacks for display
    pub fn all() -> &'static [Stack] {
        &[
            Stack::NextJs,
            Stack::NodeBackend,
            Stack::Python,
            Stack::Mobile,
            Stack::Rust,
            Stack::Go,
            Stack::Unknown,
        ]
    }
}

impl std::fmt::Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Stack detection logic
pub struct StackDetector;

impl StackDetector {
    /// Detect the primary stack of a project directory
    pub fn detect(dir: &Path) -> Stack {
        // Next.js detection (highest priority for JS projects)
        if dir.join("next.config.js").exists()
            || dir.join("next.config.mjs").exists()
            || dir.join("next.config.ts").exists()
        {
            return Stack::NextJs;
        }

        // Mobile / Expo detection
        if let Ok(content) = std::fs::read_to_string(dir.join("app.json")) {
            if content.contains("\"expo\"") {
                return Stack::Mobile;
            }
        }

        // Rust detection
        if dir.join("Cargo.toml").exists() {
            return Stack::Rust;
        }

        // Go detection
        if dir.join("go.mod").exists() {
            return Stack::Go;
        }

        // Python detection
        if dir.join("pyproject.toml").exists()
            || dir.join("requirements.txt").exists()
            || dir.join("setup.py").exists()
        {
            return Stack::Python;
        }

        // Node.js detection (check package.json for specifics)
        if dir.join("package.json").exists() {
            if let Ok(content) = std::fs::read_to_string(dir.join("package.json")) {
                if content.contains("\"next\"") {
                    return Stack::NextJs;
                }
                if content.contains("\"fastify\"")
                    || content.contains("\"express\"")
                    || content.contains("\"@nestjs/core\"")
                    || content.contains("\"hono\"")
                {
                    return Stack::NodeBackend;
                }
                // Default Node project
                return Stack::NodeBackend;
            }
        }

        Stack::Unknown
    }

    /// Get suggested commands for a stack
    pub fn suggested_commands(stack: Stack) -> StackCommands {
        match stack {
            Stack::NextJs => StackCommands {
                lint: Some("pnpm lint".to_string()),
                typecheck: Some("pnpm tsc --noEmit".to_string()),
                test: Some("pnpm test".to_string()),
                build: Some("pnpm build".to_string()),
            },
            Stack::NodeBackend => StackCommands {
                lint: Some("pnpm lint".to_string()),
                typecheck: Some("pnpm tsc --noEmit".to_string()),
                test: Some("pnpm test".to_string()),
                build: Some("pnpm build".to_string()),
            },
            Stack::Python => StackCommands {
                lint: Some("ruff check .".to_string()),
                typecheck: Some("mypy .".to_string()),
                test: Some("pytest".to_string()),
                build: None,
            },
            Stack::Mobile => StackCommands {
                lint: Some("pnpm lint".to_string()),
                typecheck: Some("pnpm tsc --noEmit".to_string()),
                test: Some("pnpm test".to_string()),
                build: Some("eas build --profile preview".to_string()),
            },
            Stack::Rust => StackCommands {
                lint: Some("cargo clippy -- -D warnings".to_string()),
                typecheck: Some("cargo check".to_string()),
                test: Some("cargo test".to_string()),
                build: Some("cargo build --release".to_string()),
            },
            Stack::Go => StackCommands {
                lint: Some("golangci-lint run".to_string()),
                typecheck: Some("go vet ./...".to_string()),
                test: Some("go test ./...".to_string()),
                build: Some("go build ./...".to_string()),
            },
            Stack::Unknown => StackCommands {
                lint: None,
                typecheck: None,
                test: None,
                build: None,
            },
        }
    }
}

/// Suggested commands for a detected stack
#[derive(Debug, Clone)]
pub struct StackCommands {
    pub lint: Option<String>,
    pub typecheck: Option<String>,
    pub test: Option<String>,
    pub build: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_detect_nextjs() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("next.config.js"), "module.exports = {}").unwrap();
        assert_eq!(StackDetector::detect(dir.path()), Stack::NextJs);
    }

    #[test]
    fn test_detect_rust() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("Cargo.toml"), "[package]\nname = \"test\"").unwrap();
        assert_eq!(StackDetector::detect(dir.path()), Stack::Rust);
    }

    #[test]
    fn test_detect_python() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("pyproject.toml"), "[project]\nname = \"test\"").unwrap();
        assert_eq!(StackDetector::detect(dir.path()), Stack::Python);
    }

    #[test]
    fn test_detect_go() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("go.mod"), "module example.com/test").unwrap();
        assert_eq!(StackDetector::detect(dir.path()), Stack::Go);
    }

    #[test]
    fn test_detect_unknown() {
        let dir = tempdir().unwrap();
        assert_eq!(StackDetector::detect(dir.path()), Stack::Unknown);
    }

    #[test]
    fn test_stack_from_str() {
        assert_eq!(Stack::parse("nextjs"), Stack::NextJs);
        assert_eq!(Stack::parse("python"), Stack::Python);
        assert_eq!(Stack::parse("rust"), Stack::Rust);
        assert_eq!(Stack::parse("unknown"), Stack::Unknown);
    }
}
