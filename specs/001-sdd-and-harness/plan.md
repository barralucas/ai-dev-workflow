# Plan: SDD e Harness

## Constituicao Aplicada
Ver `docs/governance/constitution.md`.

## Abordagem
Implementar SDD leve no proprio repositorio com `docs/` e `specs/`, mantendo compatibilidade com o fluxo existente. Adicionar harness inicial nos crates Rust existentes para validar comportamento essencial sem criar dependencia externa.

## Design
### Alternativa A: Adotar Spec Kit upstream integralmente
Pros: formato conhecido, comandos prontos, comunidade ativa.
Contras: acopla o projeto a outra ferramenta, duplica workflow de 7 fases e exige migracao maior.

### Alternativa B: SDD leve proprio em `specs/`
Pros: menor mudanca, preserva conceitos do AI Dev Workflow, facil de versionar e evoluir.
Contras: precisamos manter templates e convencoes proprias.

### Decisao
Usar Alternativa B nesta fase, documentada na ADR-0001.

## Arquivos Criados/Modificados
- Criar `docs/progress/PROGRESS.md`.
- Criar `docs/progress/decisions-log.md`.
- Criar `docs/architecture/{overview.md,tech-stack.md,data-model.md}`.
- Criar `docs/risks/risk-register.md`.
- Criar `docs/governance/constitution.md`.
- Criar `docs/adr/0001-adotar-sdd-e-harness.md`.
- Criar `docs/user-stories/backlog.md`.
- Criar `docs/features/sdd-and-harness.md`.
- Criar `specs/001-sdd-and-harness/{spec.md,plan.md,tasks.md,quickstart.md}`.
- Criar `specs/001-sdd-and-harness/contracts/harness-contract.md`.
- Atualizar `README.md`.
- Atualizar `skills/new-feature/SKILL.md`.
- Atualizar `skills/testing/SKILL.md`.
- Atualizar crates Rust com testes/harness.

## Harness Planejado
- `aidw-core`: reforcar testes de templates obrigatorios e invariantes de workflow/progress.
- `aidw-cli`: adicionar testes unitarios nos modulos de comando para `init`, `adopt` e `doctor` usando `tempfile`.
- Validar que os comandos criam `.aidw.toml` e docs esperados em diretorio temporario.

## Riscos
- Testes que imprimem muito output podem ficar ruidosos: manter assertivas em filesystem/resultado.
- `verify` usa `sh -c`; nao sera expandido nesta fase para evitar alteracao comportamental sem design dedicado.
- `skills/` esta untracked no workspace; alteracoes serao incluidas porque o usuario pediu atualizar skills.

## Verificacao
```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace
```
