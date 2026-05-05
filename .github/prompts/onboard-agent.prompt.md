---
mode: 'agent'
description: 'Onboarding de novo agente em uma sessão — leia o contexto e proponha o próximo passo.'
---

# Onboard Agent

Você está abrindo este projeto pela primeira vez (ou retomando após pausa). **Antes de codar qualquer coisa**, faça onboarding.

## Passo a passo

1. **Leia em ordem**:
   - [`AGENTS.md`](../../AGENTS.md)
   - [`docs/progress/PROGRESS.md`](../../docs/progress/PROGRESS.md)
   - [`docs/architecture/overview.md`](../../docs/architecture/overview.md) e [`tech-stack.md`](../../docs/architecture/tech-stack.md)
   - ADRs em [`docs/adr/`](../../docs/adr/) (pelo menos os 3 mais recentes + os marcados como Accepted)
   - Última feature em [`docs/features/`](../../docs/features/)
   - [`docs/risks/risk-register.md`](../../docs/risks/risk-register.md) (se existir)

2. **Verifique o repositório**:
   - Branch atual (`git status`, `git log -5`).
   - Há mudanças não-commitadas?
   - Há TODOs pendentes em código?

3. **Resuma para o usuário** (≤ 15 linhas):
   - Estado atual do projeto (release/sprint).
   - O que está em andamento (se há WIP).
   - Bloqueios conhecidos.
   - 1-3 próximos passos sugeridos com base em `PROGRESS.md`.

4. **Confirme com o usuário**:
   - "Vamos continuar em X (em andamento) ou pegar Y (próximo)?"
   - Não inicie nenhuma execução sem essa confirmação.

## Anti-padrões

- ❌ Pular leitura do `PROGRESS.md`.
- ❌ Começar a codar baseado só no que aparece nos arquivos abertos no editor.
- ❌ Assumir prioridade sem confirmar.
