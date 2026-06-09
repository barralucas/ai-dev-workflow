# Tasks: Install Update Flow

## Fase 1 - Versao

- [x] T001 Criar `VERSION`.
- [x] T002 Adicionar helpers de versao em `aidw-core`.

## Fase 2 - Instalacao/Update

- [x] T003 Atualizar `scripts/bootstrap.sh` para exibir versao, atualizar `skills/` e gravar `.aidw-version`.
- [x] T004 Atualizar `scripts/adopt.sh` para exibir versao, atualizar `skills/`, detectar versao anterior e gravar `.aidw-version`.
- [x] T005 Atualizar `aidw init` e `aidw adopt` para exibir versao e gravar `.aidw-version`.

## Fase 3 - Harness

- [x] T006 Cobrir update de `skills/` quando `.aidw-version` difere.
- [x] T007 Cobrir deteccao de update significativo.

## Fase 4 - Docs

- [x] T008 Atualizar README.
- [x] T009 Criar feature doc.
- [x] T010 Atualizar PROGRESS.

## Fase 5 - Verificacao

- [ ] T011 Rodar `cargo fmt --check` (bloqueado: `cargo` indisponivel no PATH).
- [ ] T012 Rodar `cargo clippy --workspace --all-targets -- -D warnings` (bloqueado: `cargo` indisponivel no PATH).
- [ ] T013 Rodar `cargo test --workspace` (bloqueado: `cargo` indisponivel no PATH).
- [ ] T014 Rodar `cargo build --workspace` (bloqueado: `cargo` indisponivel no PATH).
- [ ] T015 Rodar `bash -n scripts/bootstrap.sh` e `bash -n scripts/adopt.sh` (bloqueado: WSL sem distribuicao instalada neste ambiente).
