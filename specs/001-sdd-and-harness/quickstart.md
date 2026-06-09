# Quickstart: Validar SDD e Harness

## Pre-requisitos
- Rust toolchain instalado.
- Executar comandos na raiz do repositorio.

## Validacao Manual
1. Confirme que a spec existe:
   ```bash
   ls specs/001-sdd-and-harness
   ```

2. Rode formatacao:
   ```bash
   cargo fmt --check
   ```

3. Rode lint/typecheck via clippy:
   ```bash
   cargo clippy --workspace --all-targets -- -D warnings
   ```

4. Rode testes:
   ```bash
   cargo test --workspace
   ```

5. Rode build:
   ```bash
   cargo build --workspace
   ```

## Resultado Esperado
- Todos os comandos passam.
- Testes de template/workflow/CLI aparecem no output de `cargo test`.
- `docs/progress/PROGRESS.md` descreve esta migracao como concluida apos handoff.
