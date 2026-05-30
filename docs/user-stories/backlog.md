# Backlog - AI Dev Workflow

## Em Andamento

### SDD-001 - Adotar SDD e harness no proprio repositorio
Como mantenedor do AI Dev Workflow, quero que o proprio repositorio use specs versionadas e harnesses automatizados para que o toolkit seja validavel, rastreavel e confiavel para projetos consumidores.

#### Criterios de Aceite
- Existe contexto vivo minimo em `docs/progress/PROGRESS.md`, arquitetura, riscos e ADR.
- Existe spec SDD para a migracao em `specs/001-sdd-and-harness/`.
- Existem harnesses Rust para templates/workflow/CLI basica.
- README e skills relevantes documentam SDD e harness.
- `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` e `cargo build --workspace` passam.

## Proximo
- CLI-002 - Expandir `aidw doctor` para validar estrutura SDD.
- TEST-003 - Adicionar CI multi-plataforma.
