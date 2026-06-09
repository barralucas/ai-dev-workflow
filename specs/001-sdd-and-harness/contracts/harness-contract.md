# Harness Contract

## Template Harness
O harness deve falhar se qualquer template obrigatorio deixar de estar embutido:
- `docs/progress/PROGRESS.md`
- `docs/progress/decisions-log.md`
- `docs/adr/0000-template.md`
- `docs/adr/0001-stack-inicial.md`
- `docs/architecture/overview.md`
- `docs/architecture/tech-stack.md`
- `docs/architecture/data-model.md`
- `docs/features/_template.md`
- `docs/user-stories/backlog.md`
- `docs/risks/risk-register.md`
- `docs/postmortem/_template.md`
- `docs/spikes/_template.md`

## CLI Harness
### `aidw init`
Dado um diretorio temporario vazio, quando `init::run` executa com nome e stack Rust, entao devem existir:
- `.aidw.toml`
- `docs/progress/PROGRESS.md`
- `docs/architecture/tech-stack.md`
- `docs/adr/0001-stack-inicial.md`

### `aidw adopt --dry-run`
Dado um diretorio temporario com `Cargo.toml`, quando `adopt::run` executa com `dry_run=true`, entao nenhum `.aidw.toml` nem `docs/progress/PROGRESS.md` deve ser criado.

### `aidw adopt`
Dado um diretorio temporario com `Cargo.toml`, quando `adopt::run` executa sem dry-run, entao `.aidw.toml` e docs minimos devem ser criados sem apagar arquivos existentes.

### `aidw doctor`
Dado um projeto inicializado, `doctor::run` deve retornar `Ok(())`.

## Workflow Harness
O harness deve garantir:
- `Phase::all()` retorna exatamente 7 fases.
- `Context` e a primeira fase, `Handoff` e a ultima.
- `next()` e `prev()` sao consistentes para fases adjacentes.
