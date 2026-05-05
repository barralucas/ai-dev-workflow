---
mode: 'agent'
description: 'Cria uma Architecture Decision Record (ADR) para uma decisão arquitetural relevante.'
---

# Nova ADR

Use este prompt quando precisar registrar uma decisão arquitetural com trade-off real ou impacto em 2+ módulos.

## Passo a passo

1. Liste as ADRs existentes em `docs/adr/` para definir o **próximo número** (zero-padded a 4 dígitos).
2. Copie o template `templates/docs/adr/0000-template.md` para `docs/adr/NNNN-titulo-curto.md`.
3. Preencha:
   - **Status**: comece com `Proposed`. Mude para `Accepted` após validação do usuário.
   - **Contexto**: o problema, restrições, requisitos não-funcionais.
   - **Decisão**: o que foi escolhido (sem prosa — direto).
   - **Alternativas consideradas**: ≥ 2, com prós/contras objetivos.
   - **Consequências**: positivas, negativas/custos, impacto futuro.
4. Atualize `docs/progress/PROGRESS.md` na seção "Decisões recentes".
5. Linke a ADR na feature doc relacionada (se houver).
6. Commit: `docs(adr): ADR-NNNN - <título curto>`.

## Critérios para virar ADR (e não decisions-log)

- Afeta **2+ módulos** ou camadas.
- Tem **trade-off real** com alternativas viáveis.
- Será **questionado no futuro** ("por que escolhemos X?").
- Mudança aqui implica **trabalho significativo** em outras partes.

Senão, registre 1-3 linhas em `docs/progress/decisions-log.md`.
