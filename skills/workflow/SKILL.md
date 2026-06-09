---
name: workflow
description: Use ao iniciar qualquer tarefa (feature, bug, refactor, spike) ou no início/fim de sessão. Define o fluxo de 7 fases: Context → Design → Plan → Execute → Verify → Document → Handoff, com manutenção do PROGRESS.md e critérios de pronto (DoR/DoD).
---

# Workflow de Execução — 7 Fases

> Princípio guia: nenhuma linha de código sem **contexto** (story/issue), **plano** e **critério de pronto**. Nenhuma feature concluída sem **testes** e **documentação atualizada**.

Fluxo:
```
CONTEXT → DESIGN → PLAN → EXECUTE → VERIFY → DOCUMENT → HANDOFF
```

---

## Fase 1 — CONTEXT (entender o que e por quê)

1. Ler `docs/progress/PROGRESS.md` — entender estado atual.
2. Ler a User Story em `docs/user-stories/backlog.md` (criar/refinar se faltar).
3. Ler ADRs relevantes em `docs/adr/`.
4. Identificar dependências, riscos em `docs/risks/risk-register.md`.
5. Confirmar **Definition of Ready (DoR)**:
   - [ ] Objetivo claro (uma frase) e ator(es) identificados.
   - [ ] Critérios de aceite escritos.
   - [ ] Dados/contratos de entrada/saída esboçados.
   - [ ] Sem dependências bloqueantes não resolvidas.
6. Se houver ambiguidade, **perguntar ao usuário antes de avançar**.

**Gate**: agente sabe responder "o que essa tarefa entrega e para quem"; DoR ✅.

---

## Fase 2 — DESIGN (decidir como)

Aplica-se a features novas ou mudanças arquiteturais. Bugs simples pulam para PLAN.

1. Esboçar modelo de dados (entidades, relações, campos novos).
2. Esboçar fluxo: rotas/endpoints/eventos, camadas, componentes.
3. Avaliar trade-offs de ao menos 2 alternativas.
4. Aplicar: SOLID, Separation of Concerns, YAGNI, DRY (só após 2-3 usos reais).
5. **Threat modeling lite (STRIDE)**: Spoofing? Tampering? Repudiation? Info disclosure? DoS? Elevation of privilege?
6. Se decisão relevante → registrar ADR em `docs/adr/NNNN-titulo.md`.

**Gate**: trade-offs explícitos; segurança, performance, acessibilidade e custo considerados.

---

## Fase 3 — PLAN (quebrar em passos)

1. Criar **TODO list** visível ao usuário (1 item = 1 etapa atômica).
2. Ordenar por dependência; marcar paralelizáveis.
3. Para cada item, definir critério de pronto local.
4. Identificar **arquivos exatos** que serão criados/modificados.
5. Estimar risco/esforço (baixo/médio/alto).

**Gate**: usuário confirma plano (em mudanças grandes).

---

## Fase 4 — EXECUTE (implementar incrementalmente)

Ordem padrão dentro de uma story:

1. **Schemas / contratos** — validação de fronteira (zod, pydantic, joi).
2. **Camada de dados** — migrations + repositórios/queries.
3. **Camada de negócio** — casos de uso / serviços / actions / handlers.
4. **Camada de apresentação** — UI / endpoints / CLI.
5. **Estados de erro/borda** — loading, vazio, erro, timeout, retry, idempotência.
6. **Testes** — unitário + integração (caminho feliz + ao menos 1 caso de erro).

Regras:
- ✅ Commits **pequenos** no padrão Conventional Commits após cada subitem.
- ✅ Marcar TODO in-progress → completed ao terminar.
- ✅ Atualizar `PROGRESS.md` se a sessão for longa.
- ❌ Nunca silenciar erros. Nunca usar `any`/`unknown` sem narrowing. Nunca comitar `.env*` reais.
- ❌ Nunca desabilitar lint/TS sem comentário justificando.

Branch: `feat/us-XXX-titulo-curto`, `fix/...`, `refactor/...`, `docs/...`.

---

## Fase 5 — VERIFY (provar que funciona)

Pipeline obrigatório:
```
lint && typecheck && test && build
```

Verificações adicionais:
- [ ] Critérios de aceite da story atendidos (cheque um por um).
- [ ] Estados de erro/borda testados.
- [ ] **Acessibilidade** (se UI): teclado, foco, labels, contraste AA.
- [ ] **Responsividade** (se web): mobile-first a partir de 360px.
- [ ] **Segurança (OWASP)**: validação, sem PII em logs, sem stack ao cliente.
- [ ] **Performance**: sem N+1; payloads observadas.
- [ ] **Observabilidade**: logs estruturados.

Se algo falhar → voltar para EXECUTE. Nunca prosseguir com falhas.

---

## Fase 6 — DOCUMENT (deixar rastro)

1. `docs/features/<feature>.md` — objetivo, contratos, decisões, como testar.
2. `docs/adr/` — finalizar ADRs com status `Accepted`.
3. `docs/progress/PROGRESS.md` — mover story de "🚧" para "✅".
4. `docs/progress/decisions-log.md` — micro-decisões que não viraram ADR.
5. `docs/risks/risk-register.md` — adicionar/atualizar/encerrar riscos.
6. `README.md` — somente se houver nova env var, comando ou rota pública.

---

## Fase 7 — HANDOFF (entregar)

Definition of Done (DoD):
- [ ] AC da story 100% atendidos.
- [ ] Quality gates verdes (lint, typecheck, test, build).
- [ ] Testes adicionados (unit + integração mínimos).
- [ ] Doc da feature criada/atualizada.
- [ ] `PROGRESS.md` atualizado.
- [ ] ADR registrada se houve decisão relevante.
- [ ] Commits no padrão Conventional Commits.
- [ ] Sem segredos comitados.
- [ ] PR criado com descrição linkando story e ADR.

Resumo final ao usuário:
```
✅ Concluído: US-XXX <título>

📦 Entregue:
- <bullet 1>

🧪 Como testar:
1. <passo>

📝 Docs atualizados:
- docs/features/<feature>.md
- docs/progress/PROGRESS.md
- ADR-NNNN (se aplicável)

🎯 Próximo sugerido: US-YYY <título>
```

---

## Manutenção de Contexto Entre Sessões

### Início de sessão
- [ ] Ler `docs/progress/PROGRESS.md`.
- [ ] Verificar branch atual (`git status`, `git log -5`).
- [ ] Confirmar com usuário: "vamos continuar em X ou trocar para Y?".

### Fim de sessão
- [ ] Atualizar `PROGRESS.md` (concluído + em andamento + próximo + bloqueios).
- [ ] Commit (mesmo que WIP).
- [ ] Se houve decisão, registrar ADR ou linha em `decisions-log.md`.

### Ao trocar de tarefa
- Salvar estado em `PROGRESS.md` (subseção "🚧 Em andamento").
- Commit WIP **ou** `git stash` com mensagem descritiva.
- Documentar onde parou e próximo passo concreto.

---

## Bootstrap (primeira execução do projeto)

Antes da primeira story, criar:
1. `docs/progress/PROGRESS.md`
2. `docs/progress/decisions-log.md`
3. `docs/architecture/overview.md`
4. `docs/architecture/tech-stack.md`
5. `docs/architecture/data-model.md`
6. `docs/risks/risk-register.md`
7. `docs/adr/0001-stack-inicial.md`

Commit: `docs: bootstrap project context (progress, architecture, adr-0001)`.

---

## Anti-padrões

- ❌ Iniciar código sem ler `PROGRESS.md`.
- ❌ Encerrar sessão sem atualizar `PROGRESS.md`.
- ❌ Commits gigantes misturando refactor + feature + fix.
- ❌ "Vou documentar depois".
- ❌ Suprimir testes ou regras para "ganhar tempo".
- ❌ Tomar decisão arquitetural sem registrar.
- ❌ Pular fase VERIFY quando "tem certeza que funciona".
