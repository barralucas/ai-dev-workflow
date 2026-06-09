# SDD and Harness

## Objetivo
Fazer o proprio AI Dev Workflow usar specs versionadas e harnesses automatizados para reduzir regressao em CLI, templates e workflow.

## Atores
- Mantenedor do AI Dev Workflow.
- Agente de IA que trabalha neste repositorio.
- Projeto consumidor que depende dos templates e instrucoes.

## Comandos Impactados
- `cargo test --workspace`
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo build --workspace`

## Contratos
- Specs vivem em `specs/<id>-<slug>/`.
- Harness inicial fica em testes Rust nos crates existentes.
- Contexto vivo do projeto vive em `docs/progress/PROGRESS.md`.

## Decisoes
- ADR-0001 registra a adocao de SDD leve com `specs/` proprio e harness deterministico.

## Como Testar Localmente
1. Rode `cargo fmt --check`.
2. Rode `cargo clippy --workspace --all-targets -- -D warnings`.
3. Rode `cargo test --workspace`.
4. Rode `cargo build --workspace`.

## Estados De Erro Tratados
- Ausencia de templates obrigatorios deve falhar em teste.
- Ordem incorreta de fases deve falhar em teste.
- Bootstrap/adocao basicos devem falhar em teste se deixarem de criar artefatos esperados.

## Pendencias
- Adicionar CI multi-plataforma.
- Expandir harness para todos os subcomandos da CLI.
