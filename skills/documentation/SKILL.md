---
name: documentation
description: Use ao documentar features, decisões arquiteturais (ADR), atualizar PROGRESS.md, criar feature docs ou escrever README. Cobre onde mora cada tipo de doc, quando atualizar, formato de ADR, estrutura de feature doc e padrões de comentários.
---

# Documentation

> Princípio: documentação é parte da feature. **"Vou documentar depois" = não documenta**.

## 1. Onde Mora Cada Coisa

| Tipo de info                                     | Local                                  |
| ------------------------------------------------ | -------------------------------------- |
| Estado atual do projeto                          | `docs/progress/PROGRESS.md`            |
| Decisão arquitetural relevante                   | `docs/adr/NNNN-titulo.md`              |
| Micro-decisão (não vira ADR)                     | `docs/progress/decisions-log.md`       |
| Como uma feature funciona                        | `docs/features/<feature>.md`           |
| Visão geral do sistema (C4 1-2)                  | `docs/architecture/overview.md`        |
| Stack + versões                                  | `docs/architecture/tech-stack.md`      |
| Modelo de dados                                  | `docs/architecture/data-model.md`      |
| Riscos identificados                             | `docs/risks/risk-register.md`          |
| Incidente / retrospectiva                        | `docs/postmortem/YYYY-MM-DD-titulo.md` |
| Investigação time-boxed                          | `docs/spikes/YYYY-MM-DD-titulo.md`     |
| Backlog de stories                               | `docs/user-stories/backlog.md`         |
| Como rodar o projeto / env vars / rotas públicas | `README.md`                            |

## 2. Quando Atualizar (regras gatilho)

- **Toda feature concluída** → atualiza `PROGRESS.md` + cria/atualiza `docs/features/<feature>.md`.
- **Decisão com trade-off real** ou **impacto em 2+ módulos** → ADR.
- **Risco descoberto** → `risk-register.md`.
- **Incidente em prod** → postmortem + ação preventiva no backlog.
- **Nova env var, comando ou rota pública** → atualiza `README.md`.

## 3. ADR (formato mínimo)

Use o template em `templates/docs/adr/0000-template.md`. Sempre inclua:

- **Status** (`Proposed | Accepted | Superseded by ADR-XXXX`).
- **Contexto** (o problema; restrições).
- **Decisão** (o que foi escolhido).
- **Alternativas consideradas** (≥ 2, com prós/contras).
- **Consequências** (positivas, negativas, impacto futuro).

Numeração sequencial: `0001-stack-inicial.md`, `0002-autenticacao.md`, etc.

## 4. Feature Doc — O Que Cobrir

1. **Objetivo** em uma frase.
2. **Atores** envolvidos.
3. **Rotas/endpoints/comandos** entregues.
4. **Contratos** (entrada/saída) — link para schema.
5. **Decisões importantes** (link para ADR se houver).
6. **Como rodar/testar localmente** (passo a passo).
7. **Estados de erro** tratados.
8. **Pendências / próximos passos** (se houver).

## 5. README.md — O Essencial

- O que é o projeto (1 parágrafo).
- Pré-requisitos (versões de runtime, gerenciador de pacotes).
- Como instalar / rodar dev / build / testar.
- Variáveis de ambiente (apontar para `.env.example`).
- Estrutura de pastas (alto nível).
- Link para `docs/progress/PROGRESS.md` e `docs/architecture/overview.md`.

## 6. Comentários no Código

- Auto-explicativo > comentário.
- Comente o **porquê**, não o **o quê**.
- TODOs com referência: `// TODO(us-042): ...`.
- Para algoritmos complexos: explique a intuição + cite fonte se houver.

## 7. Diagramas

- Prefira **Mermaid** (versionável em markdown).
- C4 nível 1 (contexto) e 2 (containers) são suficientes para a maioria.
- Sequência para fluxos com múltiplos atores/sistemas.

## 8. PROGRESS.md — Estrutura Sugerida

```markdown
# Progress — <Projeto>

## Status Geral
<frase resumindo o estado atual>

## ✅ Concluído
- US-001: <titulo> — <data>

## 🚧 Em Andamento
- US-002: <titulo> — onde parei: <ponto exato>

## 📋 Próximo
- US-003: <titulo>

## 🔴 Bloqueios
- <descrição do bloqueio e quem resolve>

## Decisões Recentes
- <data>: <decisão resumida> → ver ADR-NNNN ou decisions-log.md
```

## Anti-padrões

- ❌ Documentar em chat/Slack/Notion sem refletir no repo.
- ❌ Doc desatualizada — pior que não ter.
- ❌ Copiar código no README (vai desatualizar; aponte para o arquivo).
- ❌ ADR genérica sem trade-off real.
- ❌ Feature doc só com "implementei X" sem o porquê e o como testar.
- ❌ `PROGRESS.md` que não reflete o estado real.
