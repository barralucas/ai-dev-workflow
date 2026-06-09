# Install Update Flow

## Objetivo

Permitir update simples do AI Dev Workflow em projetos consumidores rerodando os comandos de instalacao/adocao, com versao exibida no terminal e aviso de update significativo.

## Contratos

- `VERSION` define a versao atual do toolkit.
- `.aidw-version` registra a versao instalada no projeto consumidor.
- `skills/` e atualizado quando a versao instalada difere da versao atual.
- Docs vivos do consumidor permanecem preservados por padrao.
- `.aidw.toml` existente e preservado ao rerodar `aidw init`.

## Comandos Impactados

- `scripts/bootstrap.sh`
- `scripts/adopt.sh`
- `aidw init`
- `aidw adopt`

## Como Testar

1. Rode `cargo fmt --check`.
2. Rode `cargo clippy --workspace --all-targets -- -D warnings`.
3. Rode `cargo test --workspace`.
4. Rode `cargo build --workspace`.

## Pendencias

- Rodar gates Rust em ambiente com `cargo` disponivel.
