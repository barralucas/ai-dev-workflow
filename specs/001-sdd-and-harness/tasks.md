# Tasks: SDD e Harness

## Fase 1 - Contexto SDD
- [x] T001 Criar docs vivos minimos em `docs/progress`, `docs/architecture`, `docs/risks`, `docs/governance`, `docs/adr`, `docs/user-stories` e `docs/features`.
- [x] T002 Criar spec, plano, tasks, contrato e quickstart em `specs/001-sdd-and-harness/`.

## Fase 2 - Harness Rust
- [x] T003 Adicionar testes de templates obrigatorios em `crates/aidw-core/src/templates.rs`.
- [x] T004 Adicionar testes de invariantes de workflow em `crates/aidw-core/src/workflow.rs`.
- [x] T005 Adicionar harness de `init` em `crates/aidw-cli/src/commands/init.rs`.
- [x] T006 Adicionar harness de `adopt` em `crates/aidw-cli/src/commands/adopt.rs`.
- [x] T007 Adicionar harness de `doctor` em `crates/aidw-cli/src/commands/doctor.rs`.

## Fase 3 - Documentacao e Skills
- [x] T008 Atualizar `README.md` com SDD e harness.
- [x] T009 Atualizar `skills/new-feature/SKILL.md` para persistir specs em mudancas relevantes.
- [x] T010 Atualizar `skills/testing/SKILL.md` com harness engineering.

## Fase 4 - Verificacao
- [ ] T011 Rodar `cargo fmt --check` (bloqueado: `cargo` indisponivel no ambiente atual).
- [ ] T012 Rodar `cargo clippy --workspace --all-targets -- -D warnings` (bloqueado: `cargo` indisponivel no ambiente atual).
- [ ] T013 Rodar `cargo test --workspace` (bloqueado: `cargo` indisponivel no ambiente atual).
- [ ] T014 Rodar `cargo build --workspace` (bloqueado: `cargo` indisponivel no ambiente atual).

## Fase 5 - Handoff
- [x] T015 Atualizar `docs/progress/PROGRESS.md` com resultado final.
- [x] T016 Commitar na branch `develop`.
