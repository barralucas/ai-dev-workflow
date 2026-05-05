---
description: 'Padrões de documentação — quando, onde e como documentar para que outro dev/agente entenda em < 10 minutos.'
applyTo: '**'
---

# Documentation

> Princípio: documentação é parte da feature. **"Vou documentar depois" = não documenta**.

## 1. Onde mora cada coisa

| Tipo de info                                      | Local                                  |
| ------------------------------------------------- | -------------------------------------- |
| Estado atual do projeto                           | `docs/progress/PROGRESS.md`            |
| Decisão arquitetural relevante                    | `docs/adr/NNNN-titulo.md`              |
| Micro-decisão (não vira ADR)                      | `docs/progress/decisions-log.md`       |
| Como uma feature funciona                         | `docs/features/<feature>.md`           |
| Visão geral do sistema (C4 1-2)                   | `docs/architecture/overview.md`        |
| Stack + versões                                   | `docs/architecture/tech-stack.md`      |
| Modelo de dados                                   | `docs/architecture/data-model.md`      |
| Riscos identificados                              | `docs/risks/risk-register.md`          |
| Incidente / retrospectiva                         | `docs/postmortem/YYYY-MM-DD-titulo.md` |
| Investigação time-boxed                           | `docs/spikes/YYYY-MM-DD-titulo.md`     |
| Backlog de stories                                | `docs/user-stories/backlog.md`         |
| Como rodar o projeto / env vars / rotas públicas  | `README.md`                            |

## 2. Quando atualizar (regras gatilho)

- **Toda feature concluída** → atualiza `PROGRESS.md` + cria/atualiza `docs/features/<feature>.md`.
- **Decisão com trade-off real** ou **impacto em 2+ módulos** → ADR.
- **Risco descoberto** → `risk-register.md`.
- **Incidente em prod** → postmortem + ação preventiva no backlog.
- **Nova env var, comando ou rota pública** → atualiza `README.md`.

## 3. ADR (formato mínimo)

Use o template em `templates/docs/adr/0000-template.md`. Sempre tenha:

- **Status** (`Proposed | Accepted | Superseded by ADR-XXXX`).
- **Contexto** (o problema; restrições).
- **Decisão** (o que foi escolhido).
- **Alternativas consideradas** (≥ 2, com prós/contras).
- **Consequências** (positivas, negativas, impacto futuro).

## 4. Feature doc — o que cobrir

1. **Objetivo** em uma frase.
2. **Atores** envolvidos.
3. **Rotas/endpoints/comandos** entregues.
4. **Contratos** (entrada/saída) — link para schema.
5. **Decisões importantes** (link para ADR se houver).
6. **Como rodar/testar localmente** (passo a passo).
7. **Estados de erro** tratados.
8. **Pendências / próximos passos** (se houver).

## 5. README.md — o essencial

- O que é o projeto (1 parágrafo).
- Pré-requisitos (versões de runtime, gerenciador de pacotes).
- Como instalar / rodar dev / build / testar.
- Variáveis de ambiente (apontar para `.env.example`).
- Estrutura de pastas (alto nível).
- Link para `docs/progress/PROGRESS.md` e `docs/architecture/overview.md`.

## 6. Comentários no código

- Auto-explicativo > comentário.
- Comente o **porquê**, não o **o quê**.
- TODOs com referência: `// TODO(us-042): ...`.
- Para algoritmos complexos: explique a intuição + cite fonte se houver.

## 7. Diagramas

- Prefira **Mermaid** (versionável em markdown).
- C4 nível 1 (contexto) e 2 (containers) são suficientes para a maioria.
- Sequência para fluxos com múltiplos atores/sistemas.

## 8. Anti-padrões

- ❌ Documentar em chat/Slack/Notion sem refletir no repo.
- ❌ Doc desatualizada — pior que não ter.
- ❌ Copiar código no README (vai desatualizar; aponte para o arquivo).
- ❌ ADR genérica sem trade-off real.
- ❌ Feature doc só com "implementei X" sem o porquê e o como testar.
