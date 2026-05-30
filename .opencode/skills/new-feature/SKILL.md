---
name: new-feature
description: Use ao iniciar uma nova feature. Guia as 7 fases completas (Context → Handoff) com foco em DoR, design de dados, STRIDE, plano atômico de TODO, execução incremental (schemas → dados → negócio → UI → testes) e handoff padronizado.
---

# New Feature — Fluxo Completo

> Siga as 7 fases do workflow. Nunca comece a codar sem contexto e plano confirmado.
> Para mudancas relevantes, a spec versionada e a fonte de verdade; o chat e apenas interface de trabalho.

---

## Inputs esperados do usuário

- ID + título da story (ex.: US-042 — Listagem de partidas)
- Critérios de aceite
- Restrições ou preferências técnicas

Se algum item estiver faltando, **pergunte antes de avançar**.

---

## Fase 1 — CONTEXT

1. Ler `docs/progress/PROGRESS.md` — confirmar estado atual e sprint.
2. Localizar ou criar a story em `docs/user-stories/backlog.md`.
3. Ler ADRs relevantes em `docs/adr/`.
4. Para mudancas relevantes, localizar ou criar `specs/<id>-<slug>/spec.md`.
5. Confirmar **Definition of Ready (DoR)**:
   - [ ] Objetivo claro em uma frase e ator(es) identificados.
   - [ ] Critérios de aceite escritos.
   - [ ] Dados/contratos de entrada/saída esboçados.
   - [ ] Sem dependências bloqueantes não resolvidas.
6. Resumir contexto em ≤ 10 linhas no chat.

**Gate**: sabe responder "o que essa feature entrega e para quem"; DoR ✅.

### Quando Criar Spec SDD

Crie `specs/<id>-<slug>/` quando a feature:
- altera comportamento publico de CLI/API/UI;
- afeta 2+ modulos, templates ou comandos;
- introduz contrato, migracao, risco ou decisao arquitetural;
- precisa ser retomada por outro agente sem contexto do chat.

Estrutura minima:
```
specs/<id>-<slug>/
├── spec.md
├── plan.md
├── tasks.md
├── contracts/
└── quickstart.md
```

---

## Fase 2 — DESIGN

1. Esboçar **modelo de dados**: entidades, campos novos, relações.
2. Esboçar **fluxo**: rotas/endpoints/events, camadas, componentes principais.
3. Avaliar **≥ 2 alternativas** com prós/contras explícitos para decisões relevantes.
4. Aplicar: SRP, Separation of Concerns, YAGNI, DRY (só após 3 usos reais).
5. **STRIDE-light** para cada entrada externa:

| Categoria | Pergunta | Mitigação |
|---|---|---|
| Spoofing | Posso me passar por outro usuário? | Auth forte |
| Tampering | Posso alterar dados em trânsito? | TLS + HMAC |
| Repudiation | Posso negar ação que fiz? | Audit log |
| Info disclosure | Vazio dados indevidos? | Autz por campo |
| DoS | Derrubada por requests? | Rate limit |
| Elevation | Posso virar admin? | Verificação dupla de papel |

6. Se decisão impactar ≥ 2 módulos → registrar ADR em `docs/adr/NNNN-titulo.md`.

**Gate**: trade-offs explícitos; segurança, performance, acessibilidade e custo considerados.

---

## Fase 3 — PLAN

1. Criar **TODO list atômica** (1 item = 1 etapa, verificável).
2. Ordenar por dependência; marcar paralelizáveis.
3. Listar **arquivos exatos** que serão criados/modificados.
4. Estimar risco/esforço (baixo/médio/alto) por item.
5. Persistir plano em `specs/<id>-<slug>/plan.md` e tarefas em `specs/<id>-<slug>/tasks.md` quando houver spec.
6. Para mudanças grandes: **aguardar confirmação do usuário** antes de executar.

---

## Fase 4 — EXECUTE

Ordem padrão:

```
1. Schemas / contratos        (zod, pydantic, joi — validação de fronteira)
2. Camada de dados            (migrations, repositórios, queries)
3. Camada de negócio          (casos de uso, services, actions, handlers)
4. Camada de apresentação     (UI, endpoints, CLI)
5. Estados de erro/borda      (loading, vazio, erro, timeout, retry, idempotência)
6. Testes                     (unit + integração — caminho feliz + ≥ 1 erro)
```

Regras:
- ✅ Commits pequenos: `feat(us-XXX): add <action>` após cada subitem concluído.
- ✅ Marcar TODO in-progress → completed ao terminar cada item.
- ❌ Nunca silenciar erros. Nunca usar `any` sem narrowing. Nunca comitar `.env*` reais.
- ❌ Nunca desabilitar lint/TS sem comentário justificando.

---

## Fase 5 — VERIFY

Pipeline obrigatório:
```
lint && typecheck && test && build
```

Checklist adicional:
- [ ] Cada critério de aceite verificado individualmente.
- [ ] Harness/contrato atualizado para comportamento novo ou alterado.
- [ ] Estados de erro/borda testados manualmente.
- [ ] Acessibilidade (se UI): teclado, foco, labels, contraste AA.
- [ ] Responsividade (se web): mobile-first a partir de 360px.
- [ ] Segurança OWASP: validação, sem PII em logs, sem stack trace ao cliente.
- [ ] Performance: sem N+1 evidente; payloads observadas.

Se qualquer item falhar → voltar para EXECUTE. Nunca prosseguir com falhas.

---

## Fase 6 — DOCUMENT

- [ ] Criar `docs/features/<feature>.md` (use template em `templates/docs/features/_template.md`).
- [ ] Atualizar `specs/<id>-<slug>/` com decisoes finais, contratos e quickstart se houver spec.
- [ ] Finalizar ADR com status `Accepted` (se houver).
- [ ] Atualizar `docs/progress/PROGRESS.md` — mover story de "🚧" para "✅".
- [ ] Registrar micro-decisões em `docs/progress/decisions-log.md`.
- [ ] Atualizar `docs/risks/risk-register.md` se novos riscos descobertos.
- [ ] Atualizar `README.md` se houver nova env var, comando ou rota pública.

---

## Fase 7 — HANDOFF

Confirmar DoD completo e entregar resumo:

```
✅ Concluído: US-XXX <título>

📦 Entregue:
- <bullet 1>
- <bullet 2>

🧪 Como testar:
1. <passo>
2. <passo>

📝 Docs atualizados:
- docs/features/<feature>.md
- docs/progress/PROGRESS.md
- ADR-NNNN (se aplicável)

🎯 Próximo sugerido: US-YYY <título>
```
