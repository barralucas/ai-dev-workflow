# Atlas Skill Orchestrator

## Objetivo

Criar `atlas` como skill principal agnostica de agente do AI Dev Workflow e distribuir todas as skills do repositorio para projetos consumidores por scripts e CLI.

## Atores

- Pessoa desenvolvedora instalando o AI Dev Workflow pelo GitHub.
- Mantenedor do AI Dev Workflow.
- Agente de IA usando skills em projetos consumidores.

## Comandos Impactados

- `scripts/bootstrap.sh`
- `scripts/adopt.sh`
- `aidw init`
- `aidw adopt`
- `cargo test --workspace`

## Contratos

- `skills/` e fonte unica das skills distribuiveis.
- `skills/atlas/SKILL.md` e a entrada principal recomendada.
- Scripts copiam skills para projetos consumidores.
- CLI embute as skills e cria `skills/` durante `init` e `adopt`.

## Decisoes

- Nao duplicar skills em `templates/`; a CLI embute `skills/` diretamente.
- Manter skills especializadas existentes para uso direto e para composicao pela `atlas`.
- Distribuicao em projeto existente deve ser nao destrutiva.

## Como Testar Localmente

1. Rode `cargo fmt --check`.
2. Rode `cargo clippy --workspace --all-targets -- -D warnings`.
3. Rode `cargo test --workspace`.
4. Rode `cargo build --workspace`.

## Estados De Erro Tratados

- Ausencia da skill `atlas` deve falhar no harness de assets.
- `aidw init` deve falhar em teste se deixar de criar `skills/atlas/SKILL.md`.
- `aidw adopt` deve preservar skills existentes e falhar em teste se sobrescrever customizacao.

## Pendencias

- Rodar gates Rust em ambiente com `cargo` disponivel.
