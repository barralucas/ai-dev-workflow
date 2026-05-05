# GitHub Copilot — Instruções globais do projeto

> Este projeto adota o **AI Dev Workflow**. Antes de qualquer ação, leia [`AGENTS.md`](../AGENTS.md) e [`docs/progress/PROGRESS.md`](../docs/progress/PROGRESS.md).

## Regras gerais (não-negociáveis)

1. **Contexto primeiro**: leia `docs/progress/PROGRESS.md` no início de cada sessão; atualize-o no final.
2. **7 fases**: siga `Context → Design → Plan → Execute → Verify → Document → Handoff`. Veja [`instructions/workflow.instructions.md`](instructions/workflow.instructions.md).
3. **Quality gates**: nada vai para handoff sem `lint && typecheck && test && build` verde (ou equivalentes da stack).
4. **Decisões viram ADR**: trade-off real ou impacto em 2+ módulos → arquivo em `docs/adr/`.
5. **Validação na fronteira**: toda entrada externa (HTTP, FS, env, fila) passa por schema (zod/pydantic/etc.).
6. **Sem segredos no repo**: use `.env.example`. Nunca comite `.env*` reais.
7. **Sem `any` (ou equivalente)**, sem `catch` silencioso, sem suprimir regra de lint sem comentário justificando.
8. **Conventional Commits**: `feat(escopo): ...`, `fix:`, `chore:`, `docs:`, `refactor:`, `test:`.
9. **Idioma**: código em inglês; docs e mensagens ao usuário em PT-BR (ou idioma definido no projeto).
10. **Pergunte se houver ambiguidade** sobre escopo, regra de negócio ou prioridade.

## Onde encontrar regras específicas

- Fluxo de execução: [`instructions/workflow.instructions.md`](instructions/workflow.instructions.md)
- Qualidade de código: [`instructions/code-quality.instructions.md`](instructions/code-quality.instructions.md)
- Segurança (OWASP): [`instructions/security.instructions.md`](instructions/security.instructions.md)
- Testes: [`instructions/testing.instructions.md`](instructions/testing.instructions.md)
- Documentação: [`instructions/documentation.instructions.md`](instructions/documentation.instructions.md)
- Git/commits: [`instructions/git-workflow.instructions.md`](instructions/git-workflow.instructions.md)
- Padrões da stack: [`instructions/stacks/`](instructions/stacks/)

## Anti-padrões (não fazer)

- ❌ Codar sem ler `PROGRESS.md`.
- ❌ "Vou documentar depois".
- ❌ Commits gigantes que misturam feature + refactor + fix.
- ❌ Adicionar dependência sem justificativa (avalie: nativo? Web API? lib do framework?).
- ❌ Decisão arquitetural sem ADR.
- ❌ Pular `Verify` "porque tem certeza que funciona".
