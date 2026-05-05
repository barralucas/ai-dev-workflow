---
description: 'Adendo de stack — Node.js backend (Fastify/Express/Nest) + TypeScript estrito.'
applyTo: '**'
---

# Stack — Node.js Backend

> Adendo ao [`workflow.instructions.md`](../workflow.instructions.md). Use para serviços HTTP/jobs em Node.

## 1. Stack canônica

| Camada      | Escolha                                                | Versão |
| ----------- | ------------------------------------------------------ | ------ |
| Runtime     | Node.js LTS                                            | 22+    |
| Framework   | Fastify (default) **ou** Nest **ou** Express          | recent |
| Linguagem   | TypeScript estrito                                     | 5.x    |
| Validação   | Zod (ou TypeBox se Fastify nativo)                     | 4.x    |
| ORM/DB      | Drizzle **ou** Prisma                                  | recent |
| Logs        | pino (estruturado, JSON)                               | —      |
| Testes      | Vitest **ou** Jest + supertest                         | —      |
| Gerenciador | pnpm                                                   | 10+    |

## 2. Comandos

```bash
pnpm dev           # tsx watch ou nodemon
pnpm build         # tsc → dist/
pnpm start         # node dist/main.js
pnpm lint          # ESLint
pnpm typecheck     # tsc --noEmit
pnpm test          # Vitest
pnpm db:migrate    # drizzle-kit ou prisma migrate
```

**Pipeline VERIFY**: `pnpm lint && pnpm typecheck && pnpm test && pnpm build`.

## 3. Estrutura de pastas

```
src/
├── main.ts                          # entrypoint
├── app.ts                           # bootstrap do framework
├── env.ts                           # validação de envs
├── modules/                         # 1 pasta por domínio
│   └── <domain>/
│       ├── <domain>.routes.ts       # ou controller.ts (Nest)
│       ├── <domain>.service.ts      # casos de uso
│       ├── <domain>.repository.ts   # acesso a dados
│       ├── <domain>.schemas.ts      # zod
│       └── <domain>.test.ts
├── lib/                             # cross-cutting (db client, logger, errors)
├── middleware/                      # auth, rate-limit, cors, error-handler
└── types/
```

## 4. Validação na fronteira

- **Toda** rota valida `body`, `query`, `params`, `headers` com zod.
- Use schema único como fonte da verdade dos tipos.
- Envs em `src/env.ts` com falha fast no boot.

## 5. Padrão de erro

- Crie hierarquia de erros de domínio: `DomainError` → `NotFoundError`, `ConflictError`, `UnauthorizedError`.
- Middleware de erro central converte `DomainError` → resposta HTTP apropriada.
- **Nunca** retorne stack trace em produção. Logue server-side com correlation ID.
- Use **RFC 7807** (Problem Details) para respostas de erro JSON.

## 6. Logs & observability

- **pino** com formato JSON; level via env.
- Correlation ID por request (header `x-request-id` ou gerado).
- Logue: método, path, status, duração, userId (não logue body cru).
- Métricas: `prom-client` (Prometheus) — request count, duration histogram, errors.
- Tracing: OpenTelemetry quando o sistema crescer.

## 7. Auth

- JWT curto (≤ 15min) + refresh token rotativo (httpOnly cookie).
- Hash de senha com argon2id.
- Rate limit em `/login` (ex.: 5 tentativas / 15min / IP).
- Middleware de autz por rota; teste de "usuário X não acessa Y".

## 8. Testes

- **Unit**: services puros, schemas.
- **Integração**: routes + DB de teste (containers via testcontainers ou SQLite em memória).
- **Contrato**: snapshot do OpenAPI/JSON Schema gerado.
- supertest para HTTP-level.

## 9. Performance

- **Pool de conexões** dimensionado (DB + HTTP outbound).
- Evite N+1 — use batch loaders (DataLoader) quando aplicável.
- Cache em camada apropriada (Redis para shared, in-process para single-instance).
- Health check (`/health`) e readiness (`/ready`) separados.

## 10. Background jobs

- BullMQ (Redis) ou pg-boss (Postgres) — não use `setInterval`.
- Idempotência por `jobId` natural quando possível.
- Retry com backoff exponencial; DLQ.

## 11. Anti-padrões específicos

- ❌ `try { ... } catch (e) { console.log(e) }` — use logger estruturado e re-lance ou trate.
- ❌ Throw `string` ou `number` — sempre `Error` (subclasse).
- ❌ `process.env.X` direto — use `src/env.ts`.
- ❌ Mutar `req`/`res` sem necessidade.
- ❌ Logar body cru (PII/segredos).
- ❌ String concat em SQL — sempre prepared statements / ORM.
- ❌ `JSON.parse` sem `try` em input externo.

## 12. Bootstrap (uma vez)

```bash
pnpm init
pnpm add fastify zod pino
pnpm add -D typescript tsx vitest @types/node eslint prettier
npx tsc --init --strict
```

Configure `src/env.ts` antes de qualquer feature.
