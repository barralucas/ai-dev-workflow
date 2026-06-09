# Tasks: Atlas Skill Orchestrator

## Fase 1 - Skill e Spec

- [x] T001 Criar `skills/atlas/SKILL.md`.
- [x] T002 Criar spec, plano, tasks, contrato e quickstart em `specs/002-atlas-skill-orchestrator/`.

## Fase 2 - Distribuicao

- [x] T003 Atualizar `scripts/bootstrap.sh` para copiar `skills/`.
- [x] T004 Atualizar `scripts/adopt.sh` para copiar `skills/` sem sobrescrever.
- [x] T005 Atualizar `aidw-core` para embutir e escrever skills.
- [x] T006 Atualizar `aidw init` e `aidw adopt` para distribuir skills.

## Fase 3 - Harness

- [x] T007 Adicionar testes de embedding de skills em `aidw-core`.
- [x] T008 Adicionar/atualizar testes de `init` e `adopt` para `atlas`.

## Fase 4 - Documentacao

- [x] T009 Atualizar README com `atlas` como skill principal.
- [x] T010 Atualizar `docs/getting-started.md`.
- [x] T011 Criar `docs/features/atlas-skill-orchestrator.md`.
- [x] T012 Atualizar `docs/progress/PROGRESS.md`.

## Fase 5 - Verificacao

- [ ] T013 Rodar `cargo fmt --check` (bloqueado: `cargo` indisponivel no PATH).
- [ ] T014 Rodar `cargo clippy --workspace --all-targets -- -D warnings` (bloqueado: `cargo` indisponivel no PATH).
- [ ] T015 Rodar `cargo test --workspace` (bloqueado: `cargo` indisponivel no PATH).
- [ ] T016 Rodar `cargo build --workspace` (bloqueado: `cargo` indisponivel no PATH).
