# Feature: <Nome da Feature>

**Status**: Em andamento | Entregue | Deprecated
**Story(s)**: US-XXX, US-YYY
**Última atualização**: YYYY-MM-DD por @agente

---

## Objetivo

<!-- Em 1-2 frases. Para quem e o quê. -->

## Atores

- <Usuário X>
- <Sistema externo Y>

## Rotas / Endpoints / Comandos entregues

| Tipo | Caminho | Método | Auth | Descrição |
| ---- | ------- | ------ | ---- | --------- |
| Page | `/x`    | GET    | Sim  |           |
| API  | `/api/x`| POST   | Sim  |           |

## Contratos (entrada/saída)

- Schema: [`src/features/<feature>/schemas.ts`](../../src/features/<feature>/schemas.ts)

```ts
// Exemplo
const InputSchema = z.object({
  name: z.string().min(1).max(120),
});
```

## Decisões importantes

- ADR-NNNN: <título> — `<motivo curto>`.
- Decisões menores: ver `docs/progress/decisions-log.md`.

## Como rodar/testar localmente

1. `pnpm dev` (ou comando da stack).
2. Acessar `<rota>`.
3. Preencher `<dados>` e validar `<resultado esperado>`.

## Estados de erro tratados

- Input inválido → 400 com mensagem.
- Recurso não encontrado → 404.
- Permissão negada → 403.
- _(adicione conforme aplicável)_

## Pendências / próximos passos

- [ ] _(se houver)_
