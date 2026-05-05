# AGENTS — Entrypoint para Agentes de IA

> Este arquivo é a **porta de entrada** para qualquer agente de IA (Codex CLI, Claude, Copilot, Cursor) trabalhando neste projeto.
> Mantenha-o **curto**. Detalhes ficam nos arquivos referenciados.

---

## 1. O que ler primeiro (nesta ordem)

1. [`docs/progress/PROGRESS.md`](docs/progress/PROGRESS.md) — **estado atual** do projeto. Sempre.
2. [`.github/instructions/workflow.instructions.md`](.github/instructions/workflow.instructions.md) — as **7 fases** do fluxo de execução.
3. [`docs/architecture/overview.md`](docs/architecture/overview.md) e [`tech-stack.md`](docs/architecture/tech-stack.md) — contexto arquitetural.
4. ADRs relevantes em [`docs/adr/`](docs/adr/) ao tópico da tarefa.

## 2. Princípios não-negociáveis

- **Nunca** comece a codar sem ler `PROGRESS.md`.
- **Nunca** termine a sessão sem atualizar `PROGRESS.md`.
- **Nunca** declare uma tarefa pronta sem `lint && typecheck && test && build` verde (adapte aos comandos da stack).
- **Nunca** tome decisão arquitetural sem registrar (ADR ou `decisions-log.md`).
- **Nunca** silencie erros, suprima testes ou desabilite regras de lint sem comentário justificando.
- **Nunca** comite segredos ou `.env*` (exceto `.env.example`).
- **Sempre** valide entradas externas com schema (zod, pydantic, joi, etc.).
- **Sempre** prefira pequenos commits no padrão Conventional Commits.

## 3. Fluxo padrão (resumo)

```
CONTEXT → DESIGN → PLAN → EXECUTE → VERIFY → DOCUMENT → HANDOFF
```

Detalhes em [`.github/instructions/workflow.instructions.md`](.github/instructions/workflow.instructions.md).

## 4. Outras instruções por tema

- **Qualidade de código**: [`.github/instructions/code-quality.instructions.md`](.github/instructions/code-quality.instructions.md)
- **Segurança (OWASP + threat modeling lite)**: [`.github/instructions/security.instructions.md`](.github/instructions/security.instructions.md)
- **Testes**: [`.github/instructions/testing.instructions.md`](.github/instructions/testing.instructions.md)
- **Documentação**: [`.github/instructions/documentation.instructions.md`](.github/instructions/documentation.instructions.md)
- **Git / commits / branches**: [`.github/instructions/git-workflow.instructions.md`](.github/instructions/git-workflow.instructions.md)
- **Padrões da stack ativa**: [`.github/instructions/stacks/`](.github/instructions/stacks/)

## 5. Idioma

- Código, identificadores, nomes de branch, commits: **inglês**.
- Documentação, mensagens ao usuário, comentários explicativos: **idioma do projeto** (default: PT-BR; ajuste aqui se diferente).

## 6. Quando não souber

- Se houver ambiguidade sobre escopo/regra de negócio: **pergunte ao usuário antes de codar**.
- Se a decisão tem trade-off real: **proponha alternativas com prós/contras**, espere o aceite, registre ADR.
- Se um arquivo necessário não existe: **crie a partir do template** correspondente em `docs/`.

---

> Powered by [ai-dev-workflow](https://github.com/lucastrindade/ai-dev-workflow) — workflow opinativo agnóstico de stack para construir software com agentes de IA.
