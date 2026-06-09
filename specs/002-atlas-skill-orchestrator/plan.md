# Plan: Atlas Skill Orchestrator

## Constituicao Aplicada

Ver `docs/governance/constitution.md`.

## Abordagem

Manter `skills/` como fonte unica agnostica de agente e distribuir a pasta completa por scripts e pela CLI. A CLI passa a embutir esses assets diretamente da pasta `skills/`, evitando duplicacao em `templates/`.

## Design

### Alternativa A: Duplicar skills em `templates/skills/`

Pros: usa o mecanismo atual de templates sem novo embed.
Contras: duplica conteudo, aumenta risco de divergencia entre skill usada no repo e skill distribuida.

### Alternativa B: Embutir `skills/` como assets dedicados

Pros: fonte unica, scripts e CLI distribuem o mesmo conteudo, menor risco de drift.
Contras: exige pequena extensao em `aidw-core` para escrever assets de skills.

### Decisao

Usar Alternativa B.

## Arquivos Criados/Modificados

- Criar `skills/atlas/SKILL.md`.
- Criar `specs/002-atlas-skill-orchestrator/{spec.md,plan.md,tasks.md,quickstart.md}`.
- Criar `specs/002-atlas-skill-orchestrator/contracts/skills-distribution-contract.md`.
- Criar `docs/features/atlas-skill-orchestrator.md`.
- Atualizar `crates/aidw-core/src/templates.rs` para embutir e escrever skills.
- Atualizar `crates/aidw-cli/src/commands/init.rs`.
- Atualizar `crates/aidw-cli/src/commands/adopt.rs`.
- Atualizar `scripts/bootstrap.sh`.
- Atualizar `scripts/adopt.sh`.
- Atualizar `README.md`.
- Atualizar `docs/getting-started.md`.
- Atualizar `docs/progress/PROGRESS.md`.

## Harness Planejado

- `aidw-core`: validar que `atlas` e skills criticas existem nos assets embutidos.
- `aidw-core`: validar `write_skills` em diretorio temporario.
- `aidw-cli init`: validar criacao de `skills/atlas/SKILL.md`.
- `aidw-cli adopt`: validar criacao nao destrutiva de `skills/atlas/SKILL.md`.

## Riscos

- `rust-embed` com pasta fora de `templates/` precisa de caminho relativo correto.
- `skills/` esta untracked no workspace; esta mudanca deve incluir essas skills como artefatos oficiais.
- `cargo` esta indisponivel no ambiente atual, entao gates Rust podem continuar bloqueados localmente.

## Verificacao

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace
```
