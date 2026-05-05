---
mode: 'agent'
description: 'Conduz um postmortem blameless de incidente ou retrospectiva de sprint.'
---

# Postmortem

Postmortem é **blameless**: foco em sistemas e processos, não em pessoas.

## Quando criar

- Incidente em produção (downtime, perda de dados, vazamento).
- Bug grave que escapou para usuários.
- Decisão técnica que custou caro (retrabalho, migração).
- Retrospectiva de sprint/release com lições significativas.

## Passo a passo

1. Copie `templates/docs/postmortem/_template.md` para `docs/postmortem/YYYY-MM-DD-titulo-curto.md`.
2. Reúna a timeline factual (logs, alertas, mensagens) **antes** de analisar.
3. Identifique:
   - **O que aconteceu** (factual, sem julgamento).
   - **Impacto** (usuários, $, SLA).
   - **Causa-raiz** (use 5 Whys; vai além do sintoma).
   - **Detecção** (como soubemos; quanto demorou).
   - **Resposta** (o que foi feito; o que funcionou; o que não).
   - **O que correu bem** (sim, sempre tem algo).
   - **O que precisa mudar** (ações com dono e prazo).
4. Adicione regressão em testes para a causa-raiz (se aplicável).
5. Atualize `docs/risks/risk-register.md` com o risco identificado.
6. Adicione ações ao `docs/user-stories/backlog.md` (priorizadas).
7. Atualize `docs/progress/PROGRESS.md`.

## Princípios

- **Blameless**: pessoas tomam decisões razoáveis com a info que tinham.
- **Factual**: timeline com timestamps e fontes.
- **Acionável**: cada lição vira tarefa com dono.
- **Compartilhável**: outros times/projetos aprendem.
