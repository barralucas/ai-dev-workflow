# Spec: Install Update Flow

**ID**: SDD-003
**Status**: Accepted
**Data**: 2026-06-08

## Objetivo

Permitir que pessoas que ja instalaram o AI Dev Workflow atualizem skills e artefatos de workflow rerodando o comando de instalacao/adocao, com versao exibida no terminal e aviso quando a atualizacao for significativa.

## Nao Objetivos

- Nao sobrescrever documentos vivos do projeto consumidor como `PROGRESS.md` e ADRs existentes.
- Nao criar migrador complexo por arquivo nesta fase.
- Nao depender de rede para detectar versao.

## User Story

Como pessoa que ja usa o AI Dev Workflow, quero rerodar o comando de instalacao para atualizar skills e saber qual versao foi aplicada, para manter o projeto atualizado sem passos manuais complexos.

## Requisitos Funcionais

- RF-001: O repositorio deve ter um arquivo `VERSION` versionado.
- RF-002: Scripts `bootstrap.sh` e `adopt.sh` devem mostrar a versao atual no terminal.
- RF-003: Scripts e CLI devem gravar `.aidw-version` no projeto consumidor.
- RF-004: Scripts e CLI devem mostrar a versao instalada anterior quando `.aidw-version` existir.
- RF-005: Scripts e CLI devem avisar update significativo quando major/minor mudar.
- RF-006: Rerodar `bootstrap.sh`, `adopt.sh`, `aidw init` ou `aidw adopt` deve atualizar `skills/`.
- RF-007: Docs vivos do consumidor devem continuar preservados por padrao.
- RF-008: Rerodar `aidw init` deve preservar `.aidw.toml` existente.

## Criterios De Aceite

- CA-001: `VERSION` existe e contem a versao atual.
- CA-002: `scripts/bootstrap.sh` imprime versao e grava `.aidw-version`.
- CA-003: `scripts/adopt.sh` imprime versao, detecta versao anterior e grava `.aidw-version`.
- CA-004: CLI `init`/`adopt` imprimem versao e gravam `.aidw-version`.
- CA-005: Harness cobre update de `skills/` quando `.aidw-version` difere.
- CA-006: README explica como atualizar rerodando instalacao/adocao.
