# Stacks

Adendos específicos por stack ao [`workflow.instructions.md`](../workflow.instructions.md) (núcleo).

## Como usar

Em um projeto consumidor, mantenha **apenas** o(s) arquivo(s) que se aplicam à sua stack. Os demais podem ser apagados — eles vivem aqui no repositório do workflow apenas como biblioteca.

Cada arquivo de stack cobre, no mínimo:

1. **Comandos canônicos** (lint, typecheck, test, build, dev, format).
2. **Estrutura de pastas** sugerida.
3. **Validação na fronteira** (que lib usar).
4. **Padrões de teste** (framework + estrutura).
5. **Padrão de erro** (como propagar e expor).
6. **Performance/observability essentials** específicos.
7. **Anti-padrões** específicos.

## Stacks disponíveis

- [`nextjs.instructions.md`](nextjs.instructions.md) — Next.js 15+ (App Router) + React + TypeScript + Tailwind.
- [`node-backend.instructions.md`](node-backend.instructions.md) — Node.js (Fastify/Express/Nest) + TypeScript.
- [`python.instructions.md`](python.instructions.md) — Python 3.12+ (FastAPI/Django) + ruff + pytest.
- [`mobile.instructions.md`](mobile.instructions.md) — React Native / Expo.

## Adicionando uma nova stack

Crie um novo `<stack>.instructions.md` cobrindo as 7 seções acima. Mantenha:

- Frontmatter `applyTo` apropriado (`'**'` se a stack é única no projeto; `'src/api/**'` se é uma camada).
- Referência ao núcleo: "este arquivo é adendo de `workflow.instructions.md`".
- Comandos exatos rodáveis no terminal.
- Não duplique princípios já no núcleo — apenas o que é específico da stack.
