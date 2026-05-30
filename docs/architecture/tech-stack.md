# Tech Stack - AI Dev Workflow

## Produto
| Camada | Escolha | Observacao |
| --- | --- | --- |
| Linguagem CLI/TUI | Rust 2021 | Workspace Cargo com crates separadas |
| CLI | clap | Parsing de comandos e flags |
| TUI | ratatui + crossterm | Dashboard terminal |
| Config | TOML via `toml` + serde | `.aidw.toml` em projetos consumidores |
| Templates | rust-embed | Templates embutidos no binario |
| Erros | anyhow + thiserror | `anyhow` em CLI, `thiserror` em core |
| Markdown | pulldown-cmark | Parsing futuro/auxiliar de docs |
| Testes | cargo test + tempfile | Harness deterministico com diretorios temporarios |

## Comandos de Verificacao
```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace
```

## Plataformas Alvo
- Linux/macOS com shell POSIX para scripts `.sh`.
- Windows via PowerShell, WSL ou Git Bash conforme documentacao.
- CLI Rust deve ser testavel em Windows sem exigir bash para fluxos que nao executam comandos configurados pelo usuario.

## Dependencias De Desenvolvimento
- `tempfile` para testes com projetos temporarios.
- Harness futuro pode adicionar `assert_cmd` e `predicates` se testes de binario exigirem assertivas mais ricas.
