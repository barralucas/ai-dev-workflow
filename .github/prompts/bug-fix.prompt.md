---
mode: 'agent'
description: 'Investiga e corrige um bug com teste de regressão obrigatório.'
---

# Bug Fix

Siga o fluxo abaixo. **Toda correção exige teste de regressão**.

## Passo a passo

1. **Reproduza** o bug com passos claros. Se o usuário não forneceu, peça.
2. **Localize** a causa-raiz (não o sintoma). Use logs, debugger, busca no código.
3. **Escreva o teste que falha** demonstrando o bug — antes de corrigir.
4. **Corrija** o código. O teste deve passar.
5. **Verifique regressões**: rode pipeline completo (`lint && typecheck && test && build`).
6. **Documente**:
   - Atualize `docs/progress/PROGRESS.md` (✅ + breve causa-raiz).
   - Se o bug indica risco recorrente, adicione em `docs/risks/risk-register.md`.
   - Se foi incidente em produção, crie `docs/postmortem/YYYY-MM-DD-titulo.md`.
7. **Commit**: `fix(escopo): descrição curta` referenciando issue/story se houver.

## Template de mensagem de commit

```
fix(<escopo>): <descrição curta>

Causa-raiz: <1-2 frases>
Regressão coberta por: <arquivo de teste>

Closes #<issue> (se aplicável)
```

## Inputs esperados

- Descrição do bug + passos para reproduzir.
- Comportamento esperado vs. observado.
- Ambiente (versão, browser, OS) se relevante.
