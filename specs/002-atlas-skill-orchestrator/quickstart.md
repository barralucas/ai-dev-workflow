# Quickstart: Atlas Skill Orchestrator

## Validar Conteudo

1. Confirmar que `skills/atlas/SKILL.md` existe.
2. Confirmar que a skill possui frontmatter `name: atlas`.
3. Confirmar que a matriz de roteamento cobre as skills existentes.

## Validar Scripts Manualmente

Em um diretorio temporario fora do repo:

```bash
bash /caminho/para/ai-dev-workflow/scripts/bootstrap.sh --name Fixture --stack none
test -f skills/atlas/SKILL.md
```

Para projeto existente:

```bash
bash /caminho/para/ai-dev-workflow/scripts/adopt.sh --yes --stack none
test -f skills/atlas/SKILL.md
```

## Validar CLI

```bash
cargo test --workspace
```

## Gates Completos

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace
```
