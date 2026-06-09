# Plan: Install Update Flow

## Abordagem

Usar um arquivo `VERSION` no repositorio como fonte simples para scripts e CLI. Projetos consumidores recebem `.aidw-version` para registrar a versao instalada. Ao rerodar instalacao/adocao, `skills/` e atualizado e docs vivos continuam preservados.

## Design

### Alternativa A: Versao apenas no Cargo workspace

Pros: fonte unica para binarios Rust.
Contras: scripts shell precisariam parsear TOML; projetos sem CLI nao teriam caminho simples.

### Alternativa B: Arquivo `VERSION`

Pros: simples para scripts e CLI, facil de exibir no terminal, sem dependencia nova.
Contras: precisa manter sincronizado com Cargo quando houver release.

### Decisao

Usar `VERSION` nesta fase.

## Arquivos Impactados

- `VERSION`
- `scripts/bootstrap.sh`
- `scripts/adopt.sh`
- `crates/aidw-core/src/templates.rs`
- `crates/aidw-cli/src/commands/init.rs`
- `crates/aidw-cli/src/commands/adopt.rs`
- `README.md`
- `docs/features/install-update-flow.md`
- `docs/progress/PROGRESS.md`

## Verificacao

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace
```
