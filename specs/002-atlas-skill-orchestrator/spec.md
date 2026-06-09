# Spec: Atlas Skill Orchestrator

**ID**: SDD-002
**Status**: Accepted
**Data**: 2026-06-08

## Objetivo

Criar a skill `atlas` como entrada principal agnostica de agente do AI Dev Workflow e distribuir todas as skills do repositorio para projetos consumidores por scripts e CLI.

## Nao Objetivos

- Nao remover nem renomear as skills especializadas existentes.
- Nao esconder nem remover skills especializadas existentes.
- Nao acoplar a distribuicao a um agente especifico.
- Nao criar dependencia nova para empacotar assets.

## Usuarios

- **Pessoa desenvolvedora**: quer instalar o workflow pelo GitHub e usar uma unica skill para comecar qualquer tarefa.
- **Mantenedor do workflow**: precisa evoluir skills sem duplicar fonte da verdade.
- **Agente de IA**: precisa classificar tarefas e aplicar as skills corretas sem depender de escolha manual do usuario.

## User Story

Como pessoa usando o AI Dev Workflow, quero chamar apenas a skill `atlas` para qualquer tarefa, para que o agente escolha automaticamente o fluxo correto e as skills especializadas necessarias.

## Requisitos Funcionais

- RF-001: O repositorio deve conter `skills/atlas/SKILL.md` com frontmatter valido.
- RF-002: A skill `atlas` deve conter matriz de roteamento para todas as skills existentes.
- RF-003: `scripts/bootstrap.sh` deve copiar `skills/` para o projeto alvo.
- RF-004: `scripts/adopt.sh` deve copiar `skills/` sem sobrescrever arquivos existentes.
- RF-005: `aidw init` deve distribuir `skills/`.
- RF-006: `aidw adopt` deve distribuir `skills/` sem sobrescrever customizacoes.
- RF-007: Harness deve validar que `atlas` e skills criticas sao embutidas e criadas por init/adopt.
- RF-008: README/getting-started devem orientar usuarios a comecar pela skill `atlas`.

## Requisitos Nao Funcionais

- RNF-001: As skills devem ter uma unica fonte de verdade em `skills/`.
- RNF-002: A distribuicao deve ser nao destrutiva em projetos existentes.
- RNF-003: Testes devem usar diretorios temporarios e nao depender de rede.
- RNF-004: A mudanca deve preservar o fluxo de instalacao atual por scripts e CLI.

## Criterios De Aceite

- CA-001: `skills/atlas/SKILL.md` existe e usa `name: atlas`.
- CA-002: `bootstrap.sh` copia todas as skills para `skills/` no destino.
- CA-003: `adopt.sh` copia todas as skills para `skills/` no destino sem sobrescrever.
- CA-004: `aidw init` cria `skills/atlas/SKILL.md`.
- CA-005: `aidw adopt` cria `skills/atlas/SKILL.md` e preserva skill existente.
- CA-006: Testes/harness cobrem embedding e escrita das skills.
- CA-007: Documentacao publica recomenda `atlas` como skill principal.
- CA-008: `PROGRESS.md` registra a entrega ou bloqueios.

## Checklist De Revisao

- [x] Escopo define distribuicao, nao apenas criacao local da skill.
- [x] Criterios de aceite sao verificaveis por filesystem e docs.
- [x] Nao objetivos evitam quebrar usuarios que ja chamam skills especializadas.
