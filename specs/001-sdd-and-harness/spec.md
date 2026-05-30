# Spec: SDD e Harness no Proprio Repositorio

**ID**: SDD-001
**Status**: Accepted
**Data**: 2026-05-29

## Objetivo
Adotar Spec-Driven Development e harnesses automatizados no proprio repositorio AI Dev Workflow para tornar o projeto rastreavel, validavel e consistente com as praticas que recomenda.

## Nao Objetivos
- Nao implementar evals com LLM real nesta fase.
- Nao substituir o workflow de 7 fases por Spec Kit upstream.
- Nao refatorar a arquitetura da CLI/TUI alem do necessario para testes.

## Usuarios
- **Mantenedor**: precisa evoluir templates, prompts e CLI com seguranca.
- **Agente de IA**: precisa encontrar contexto, specs e criterio de pronto sem depender de chat anterior.
- **Projeto consumidor**: precisa receber templates e comandos confiaveis.

## User Story
Como mantenedor do AI Dev Workflow, quero que cada mudanca relevante tenha spec, plano, tasks e harness para que regressões em workflow, templates e CLI sejam detectadas antes de entrega.

## Requisitos Funcionais
- RF-001: O repositorio deve conter `docs/progress/PROGRESS.md` com estado atual.
- RF-002: O repositorio deve conter arquitetura, tech stack, modelo de dados, riscos, backlog e ADR inicial.
- RF-003: A mudanca deve possuir artefatos SDD em `specs/001-sdd-and-harness/`.
- RF-004: O harness deve validar que templates obrigatorios estao embutidos.
- RF-005: O harness deve validar invariantes do workflow de fases.
- RF-006: O harness deve validar bootstrap/adocao basicos de projeto temporario.
- RF-007: README e skills relevantes devem explicar quando criar specs e como pensar em harnesses.

## Requisitos Nao Funcionais
- RNF-001: Testes devem ser deterministas e nao depender de rede.
- RNF-002: Testes devem usar diretorios temporarios para nao modificar o workspace real.
- RNF-003: O projeto deve continuar buildando com `cargo build --workspace`.
- RNF-004: A solucao deve evitar dependencias novas quando `std` e dependencias existentes forem suficientes.

## Criterios De Aceite
- CA-001: `docs/progress/PROGRESS.md` existe e descreve o estado atual.
- CA-002: `docs/adr/0001-adotar-sdd-e-harness.md` existe e registra a decisao.
- CA-003: `specs/001-sdd-and-harness/{spec.md,plan.md,tasks.md,quickstart.md}` existem.
- CA-004: Existe pelo menos um contrato em `specs/001-sdd-and-harness/contracts/`.
- CA-005: Harness de templates cobre lista obrigatoria de docs.
- CA-006: Harness de CLI cobre inicializacao/adocao ou comandos equivalentes sem tocar no workspace real.
- CA-007: README e skills relevantes mencionam SDD/harness.
- CA-008: `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` e `cargo build --workspace` passam.

## Checklist De Revisao
- [x] Escopo define o que e por que, sem entrar direto no codigo.
- [x] Criterios de aceite sao verificaveis.
- [x] Nao objetivos reduzem risco de escopo crescer.
- [x] Requisitos de teste/harness estao claros.
