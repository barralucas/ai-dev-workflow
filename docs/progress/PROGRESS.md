# Progress - AI Dev Workflow

## Status Geral
O projeto passou a dogfoodear Spec-Driven Development, possui harness inicial para CLI/templates/workflow e agora inclui `skills/atlas` como entrada principal agnostica de agente. A verificacao local completa esta bloqueada neste ambiente porque `cargo` nao esta instalado/disponivel no PATH.

## Concluido
- Bootstrap inicial do AI Dev Workflow como biblioteca de instrucoes, templates e prompts.
- CLI Rust inicial com comandos `init`, `adopt`, `doctor`, `status`, `verify`, `progress`, `phase`, `adr` e `session`.
- TUI Rust inicial para acompanhar progresso e fases do workflow.
- SDD-001: Adotar SDD e harness no proprio repositorio.
  - Criados `docs/` vivos, ADR-0001, backlog, feature doc e `specs/001-sdd-and-harness/`.
  - Adicionados harnesses Rust para templates obrigatorios, ordem das fases, `init`, `adopt` e `doctor`.
  - Atualizados README e skills `new-feature`/`testing` com SDD e harness engineering.
- SDD-002: Criar skill `atlas` e distribuir skills agnosticas de agente.
  - Criada `skills/atlas/SKILL.md` como orquestradora principal.
  - Scripts `bootstrap.sh`/`adopt.sh` passam a copiar `skills/`.
  - CLI passa a embutir `skills/` e distribuir skills em `init`/`adopt`.
  - README/getting-started e feature doc atualizados para orientar uso da `atlas`.

## Em Andamento
- Verificacao local dos gates Rust.
  - Bloqueio: `cargo` nao encontrado no PowerShell e WSL nao tem distribuicao instalada.

## Proximo
- Expandir fixtures e cenarios de harness para comandos `progress`, `phase`, `adr`, `status` e `session`.
- Adicionar CI multi-plataforma para `cargo fmt`, `cargo clippy`, `cargo test` e `cargo build`.

## Bloqueios
- `cargo` indisponivel no ambiente atual; gates precisam ser rodados em maquina com Rust toolchain.

## Decisoes Recentes
- 2026-05-29: O proprio repositorio passa a usar `docs/` e `specs/` como fontes versionadas de contexto e intencao.
- 2026-05-29: Harness inicial foca em testes Rust deterministas antes de evals com LLM real.
- 2026-06-08: `atlas` passa a ser a skill principal recomendada; as demais skills continuam como blocos especializados agnosticos de agente distribuidos junto do toolkit.
