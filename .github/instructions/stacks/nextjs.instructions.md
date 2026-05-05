---
description: 'Adendo de stack — Next.js 15+ (App Router), React 19+, TypeScript estrito, Tailwind CSS.'
applyTo: '**'
---

# Stack — Next.js (App Router)

> Adendo ao [`workflow.instructions.md`](../workflow.instructions.md). Use **apenas** se o projeto for Next.js.

## 1. Stack canônica

| Camada      | Escolha                                                            | Versão |
| ----------- | ------------------------------------------------------------------ | ------ |
| Framework   | Next.js (App Router, Server Components)                            | 15+    |
| UI lib      | React                                                              | 19+    |
| Linguagem   | TypeScript estrito (`strict` + `noUncheckedIndexedAccess`)         | 5.x    |
| Estilo      | Tailwind CSS                                                       | 4.x    |
| Validação   | Zod                                                                | 4.x    |
| Lint/Format | ESLint (config Next + TS) + Prettier (+ plugin Tailwind)           | —      |
| Testes      | Vitest + Testing Library + jsdom                                   | —      |
| Gerenciador | pnpm (via corepack)                                                | 10+    |

## 2. Comandos

```bash
pnpm dev           # desenvolvimento
pnpm build         # build de produção
pnpm start         # rodar build
pnpm lint          # ESLint
pnpm typecheck     # tsc --noEmit
pnpm test          # Vitest
pnpm format        # Prettier --write .
```

**Pipeline VERIFY**:

```bash
pnpm lint && pnpm typecheck && pnpm test && pnpm build
```

## 3. Estrutura de pastas

```
src/
├── app/                  # App Router (rotas, layouts, route handlers)
│   ├── (public)/         # Grupos de rotas públicas
│   ├── (app)/            # Grupos de rotas autenticadas
│   ├── api/              # Route Handlers
│   ├── layout.tsx
│   └── page.tsx
├── components/
│   ├── ui/               # Primitivos (Button, Input, ...)
│   └── features/         # Componentes específicos de feature
├── features/             # Lógica por domínio
│   └── <feature>/
│       ├── components/
│       ├── hooks/
│       ├── server/       # Server actions, queries, repositories
│       └── schemas.ts    # Validação zod
├── lib/                  # Utilitários genéricos (db, auth, fetcher)
├── hooks/                # Hooks reutilizáveis globais
├── styles/               # globals.css e tokens
├── types/                # Tipos compartilhados
└── env.ts                # Validação de envs com zod
```

**Regra**: código específico de feature mora em `src/features/<feature>/`. Apenas extraia para `lib/` ou `components/ui/` quando reutilizado por **2+ features**.

## 4. Padrões fundamentais

- **Server Components por padrão**; `'use client'` **só** com estado, eventos ou APIs de browser.
- **Server Actions** para mutações; **Route Handlers** para APIs públicas/integrações.
- **Data fetching no servidor** (`fetch` + cache nativo); **não** chame API interna a partir de outro Server Component — chame a função diretamente.
- **Forms**: `react-hook-form` + `zodResolver` + Server Action.
- **Imagens**: `next/image`. **Fontes**: `next/font`.
- **Imports**: alias `@/*`. Ordem: libs externas → `@/lib` → `@/components` → `@/features` → relativos.
- **Naming**: componentes `PascalCase`, hooks `useXxx`, arquivos de componente em `PascalCase.tsx`, demais em `kebab-case.ts`.

## 5. Validação na fronteira

- **Toda** server action e route handler valida entrada com **zod**.
- Tipos derivam do schema (`z.infer<typeof Schema>`).
- Envs validados em `src/env.ts` (falha fast no boot).
- DTOs explícitos para o cliente — não vaze modelos de DB.

## 6. Estados de UI obrigatórios

Para cada tela, trate explicitamente:

- **Loading**: `loading.tsx` ou skeletons.
- **Erro**: `error.tsx` com fallback amigável.
- **Vazio**: empty state com CTA quando aplicável.
- **Sucesso**: feedback claro (toast/redirect).

## 7. Performance & SEO

- `metadata` em cada rota pública (title, description, openGraph).
- `next/image` + `priority` somente above-the-fold.
- Avalie cache: `fetch` com `revalidate`/`tags` ou `unstable_cache` para queries pesadas.
- Use `updateTag`/`revalidateTag` em mutações que afetam listas cacheadas.
- Evite `'use client'` em árvores grandes; passe dados serializados Server → Client.

## 8. Acessibilidade

- Labels associados a inputs; `aria-*` quando necessário.
- Navegação por teclado; foco visível.
- Contraste AA mínimo.
- Mobile-first a partir de 360px.

## 9. Testes

- **Unit**: schemas, utils, server actions com mocks de DB.
- **Componente**: render + interação principal (Testing Library).
- **E2E**: Playwright para fluxos críticos (opcional).
- Setup: `vitest.config.ts` + `src/test/setup.ts`.

## 10. Anti-padrões específicos

- ❌ `useEffect` para data fetching em Server Components (não compila — mas em CC, prefira RSC).
- ❌ Server Action mutando sem `revalidateTag`/`revalidatePath`.
- ❌ Importar código `'use server'` em Client Component que não é server action.
- ❌ Usar `next/router` (Pages Router) em App Router — use `next/navigation`.
- ❌ Chamar `/api/...` interna a partir de Server Component — chame a função.
- ❌ `process.env.X` direto no código — use `src/env.ts`.

## 11. Bootstrap (uma vez por projeto)

```bash
pnpm create next-app@latest . --typescript --eslint --tailwind --app --src-dir --import-alias "@/*" --use-pnpm
pnpm add -D prettier prettier-plugin-tailwindcss eslint-config-prettier
pnpm add -D vitest @vitejs/plugin-react @testing-library/react @testing-library/jest-dom jsdom
pnpm add zod
```

Crie `src/env.ts` com validação zod das envs antes da primeira feature.
