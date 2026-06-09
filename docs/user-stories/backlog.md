# Backlog - AI Dev Workflow

## Em Andamento

### SDD-003 - Simplificar update do workflow instalado
Como pessoa que ja usa o AI Dev Workflow, quero rerodar o comando de instalacao para atualizar skills e saber qual versao foi aplicada, para manter o projeto atualizado sem passos manuais complexos.

#### Criterios de Aceite
- Existe `VERSION`.
- Scripts e CLI exibem a versao atual.
- Projetos consumidores recebem `.aidw-version`.
- Rerodar instalacao/adocao atualiza `skills/`.
- Update significativo mostra aviso no terminal.

### SDD-002 - Criar skill atlas e distribuir skills agnosticas de agente
Como pessoa usando o AI Dev Workflow, quero chamar apenas a skill `atlas` para qualquer tarefa, para que o agente escolha automaticamente o fluxo correto e as skills especializadas necessarias.

#### Criterios de Aceite
- Existe `skills/atlas/SKILL.md`.
- Scripts `bootstrap.sh` e `adopt.sh` distribuem `skills/`.
- `aidw init` e `aidw adopt` criam `skills/atlas/SKILL.md`.
- Harness cobre embedding e distribuicao de skills.
- README e getting-started recomendam `atlas` como entrada principal.

### SDD-001 - Adotar SDD e harness no proprio repositorio
Como mantenedor do AI Dev Workflow, quero que o proprio repositorio use specs versionadas e harnesses automatizados para que o toolkit seja validavel, rastreavel e confiavel para projetos consumidores.

#### Criterios de Aceite
- Existe contexto vivo minimo em `docs/progress/PROGRESS.md`, arquitetura, riscos e ADR.
- Existe spec SDD para a migracao em `specs/001-sdd-and-harness/`.
- Existem harnesses Rust para templates/workflow/CLI basica.
- README e skills relevantes documentam SDD e harness.
- `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` e `cargo build --workspace` passam.

## Proximo
- CLI-002 - Expandir `aidw doctor` para validar estrutura SDD.
- TEST-003 - Adicionar CI multi-plataforma.
