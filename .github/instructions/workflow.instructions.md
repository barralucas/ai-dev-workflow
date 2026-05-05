---
description: 'Workflow universal de execução de tarefas para agentes de IA — 7 fases (Context → Design → Plan → Execute → Verify → Document → Handoff) com manutenção contínua de contexto e decisões.'
applyTo: '**'
---

# Workflow de Execução — Núcleo Universal

Este workflow define **como o agente deve executar cada tarefa** (story, bug, refactor, spike) garantindo:

1. **Boas práticas** de engenharia (SOLID, Clean Code, OWASP, testes, acessibilidade).
2. **Decisões arquiteturais** documentadas e rastreáveis (ADRs).
3. **Contexto vivo** — sempre dá para saber o que foi feito, o que está em andamento e o que falta.

> **Princípio guia**: nenhuma linha de código sem **contexto** (story/issue), **plano** e **critério de pronto**. Nenhuma feature concluída sem **testes** e **documentação atualizada**.

> **Stack-agnóstico**: este arquivo descreve o **fluxo**. Comandos, padrões de pasta e idiomas específicos da stack ficam em [`stacks/`](stacks/).

---

## 0. Estrutura de Contexto do Projeto

A estrutura abaixo é a **fonte da verdade** do estado do projeto. Mantenha sempre atualizada.

```
<projeto>/
├── docs/
│   ├── user-stories/
│   │   └── backlog.md              # Backlog completo
│   ├── adr/                        # Architecture Decision Records
│   │   └── NNNN-titulo.md
│   ├── features/                   # 1 doc por feature implementada
│   │   └── <feature>.md
│   ├── architecture/
│   │   ├── overview.md             # Visão geral (C4 nível 1-2)
│   │   ├── data-model.md           # ER + schemas
│   │   └── tech-stack.md           # Stack + versões
│   ├── risks/
│   │   └── risk-register.md        # Catálogo de riscos
│   ├── postmortem/                 # Postmortems de incidentes/retros
│   ├── spikes/                     # Investigações time-boxed
│   └── progress/
│       ├── PROGRESS.md             # ← PRINCIPAL: estado atual do projeto
│       ├── sprint-NN.md            # Notas por sprint (opcional)
│       └── decisions-log.md        # Diário de decisões pequenas (não-ADR)
├── src/                            # Código (ver instruções de stack)
└── .github/
    └── instructions/               # Guias para o agente
```

### 0.1. `docs/progress/PROGRESS.md` — Painel de Controle

Arquivo **vivo** que o agente **lê no início** e **atualiza no fim** de toda sessão. Veja o template em `templates/docs/progress/PROGRESS.md`.

> **Regra de ferro**: o agente **nunca** começa uma tarefa sem ler o `PROGRESS.md`. **Nunca** termina uma tarefa sem atualizá-lo.

---

## 1. Fases do Workflow

```
┌─────────┐   ┌────────┐   ┌─────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌─────────┐
│ CONTEXT │ → │ DESIGN │ → │ PLAN    │ → │ EXECUTE  │ → │ VERIFY   │ → │ DOCUMENT │ → │ HANDOFF │
└─────────┘   └────────┘   └─────────┘   └──────────┘   └──────────┘   └──────────┘   └─────────┘
```

Cada fase tem **entrada**, **saída** e **gate** (critério para avançar).

---

### Fase 1 — CONTEXT (entender o que e por quê)

**Entrada**: pedido do usuário (story, bug, ideia).

**Ações**:

1. Ler `docs/progress/PROGRESS.md` — entender estado atual.
2. Ler a User Story relacionada em `docs/user-stories/backlog.md` (criar/refinar se faltar).
3. Ler ADRs existentes em `docs/adr/` relevantes ao escopo.
4. Identificar **dependências**: outras stories, libs, decisões pendentes, riscos no `risk-register.md`.
5. Confirmar **Definition of Ready (DoR)**:
   - [ ] Objetivo claro (uma frase) e ator(es) identificados.
   - [ ] Critérios de aceite escritos.
   - [ ] Dados/contratos de entrada/saída esboçados.
   - [ ] Sem dependências bloqueantes não resolvidas.
6. Se houver **ambiguidade**, perguntar ao usuário **antes** de avançar.

**Saída**: contexto consolidado em ≤ 10 linhas no chat (ou no plano).

**Gate**: agente sabe responder em uma frase "o que essa tarefa entrega e para quem"; DoR ✅.

---

### Fase 2 — DESIGN (decidir como)

**Quando aplicar**: features novas, mudanças arquiteturais ou que impactem ≥ 2 módulos. Bugs simples pulam para PLAN.

**Ações**:

1. Esboçar **modelo de dados** (entidades, relações, campos novos).
2. Esboçar **fluxo**: rotas/endpoints/eventos, camadas, componentes principais.
3. Avaliar **trade-offs** de no mínimo 2 alternativas para decisões relevantes.
4. Aplicar princípios:
   - **SOLID** (especialmente SRP nos módulos de feature).
   - **Separation of Concerns**: UI ≠ regra de negócio ≠ acesso a dados.
   - **YAGNI**: não construir o que não tem story.
   - **DRY com cuidado**: só extraia abstração após 2-3 usos reais.
5. **Threat modeling lite**: para qualquer entrada externa, responder STRIDE-light:
   - Spoofing/Identity? Tampering? Repudiation? Info disclosure? DoS? Elevation of privilege?
6. Se a decisão for relevante, registrar **ADR** em `docs/adr/NNNN-titulo.md` (template em `templates/docs/adr/0000-template.md`).

**Saída**: seção "Design" no plano + ADR (se aplicável).

**Gate**: trade-offs explícitos; segurança (OWASP), performance, acessibilidade e custo considerados.

---

### Fase 3 — PLAN (quebrar em passos)

**Ações**:

1. Criar **TODO list** visível ao usuário (1 item = 1 etapa atômica).
2. Ordenar por dependência; marcar paralelizáveis.
3. Para cada item, definir o **critério de pronto** local (ex.: "schema validado por testes").
4. Identificar **arquivos exatos** que serão criados/modificados.
5. Estimar risco/esforço (baixo/médio/alto).

**Saída**: TODO list + lista de arquivos.

**Gate**: usuário confirma plano (em mudanças grandes) **OU** plano é trivialmente óbvio.

---

### Fase 4 — EXECUTE (implementar incrementalmente)

> Siga as **fases por feature** descritas no adendo de stack ativo (`stacks/<sua-stack>.instructions.md`).

**Ordem padrão dentro de uma story** (independente de stack):

1. **Schemas / contratos** — validação de fronteira (zod, pydantic, joi, JSON Schema).
2. **Camada de dados** — migrations + repositórios/queries.
3. **Camada de negócio** — casos de uso / serviços / actions / handlers.
4. **Camada de apresentação** — UI / endpoints / CLI / etc.
5. **Estados de erro/borda** — loading, vazio, erro, timeout, retry, idempotência.
6. **Testes** — unitário + integração (caminho feliz + ao menos 1 caso de erro).

**Regras durante execução**:

- ✅ Commits **pequenos** no padrão Conventional Commits após cada subitem da TODO.
- ✅ Marcar item da TODO como **in-progress → completed** assim que terminar.
- ✅ Atualizar `PROGRESS.md` em "🚧 Em andamento" se a sessão for longa.
- ❌ Nunca silenciar erros. Nunca usar `any`/`unknown` sem narrowing. Nunca comitar `.env*` reais.
- ❌ Nunca desabilitar lint/TS sem comentário justificando.

**Branch e commits**:

- Branch: `feat/us-XXX-titulo-curto`, `fix/...`, `refactor/...`, `docs/...`.
- Commit: `feat(us-005): adicionar action joinMatch` — referencie a story.

---

### Fase 5 — VERIFY (provar que funciona)

**Pipeline obrigatório** antes de declarar a feature pronta (substitua pelos comandos da sua stack):

```
<lint> && <typecheck> && <test> && <build>
```

**Verificações adicionais**:

- [ ] Critérios de aceite da story atendidos (cheque um por um).
- [ ] Estados de erro/borda testados manualmente.
- [ ] **Acessibilidade** (se UI): navegação por teclado, foco visível, labels, contraste AA.
- [ ] **Responsividade** (se web): mobile-first a partir de 360px.
- [ ] **Segurança** (OWASP): validação de entrada, sem PII em logs, sem stack ao cliente, autz/autn aplicada.
- [ ] **Performance**: sem N+1; payloads/queries observadas; sem `use client` desnecessário (web).
- [ ] **Observabilidade**: logs estruturados nos pontos críticos; métricas/traces se aplicável.

**Se algo falhar**: voltar para EXECUTE — **nunca** prosseguir com falhas.

---

### Fase 6 — DOCUMENT (deixar rastro)

Atualize **antes do handoff**:

1. **`docs/features/<feature>.md`** — documentação da feature: objetivo, contratos (entrada/saída), decisões, como rodar/testar localmente.
2. **`docs/adr/`** — finalizar ADRs (status `Accepted`) se houve.
3. **`docs/progress/PROGRESS.md`** — mover story de "🚧" para "✅", atualizar próximos.
4. **`docs/progress/decisions-log.md`** — micro-decisões que não viraram ADR (1-3 linhas cada).
5. **`docs/risks/risk-register.md`** — adicionar/atualizar/encerrar riscos descobertos.
6. **`README.md`** — somente se houver nova env var, comando ou rota pública.

**Regra de ouro**: se outro dev/agente abrir o projeto agora, ele consegue entender em **< 10 minutos** lendo apenas `PROGRESS.md` + a doc da feature.

---

### Fase 7 — HANDOFF (entregar)

**Definition of Done (DoD)**: marque cada item antes de declarar pronto.

- [ ] AC da story 100% atendidos.
- [ ] Quality gates verdes (lint, typecheck, test, build).
- [ ] Testes adicionados (unit + integração mínimos).
- [ ] Doc da feature criada/atualizada.
- [ ] `PROGRESS.md` atualizado.
- [ ] ADR registrada se houve decisão relevante.
- [ ] Commits no padrão Conventional Commits.
- [ ] Sem segredos comitados.
- [ ] PR criado (se for fluxo de PR) com descrição linkando story e ADR.

**Resumo final ao usuário** (formato fixo):

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

---

## 2. Manutenção de Contexto Entre Sessões

### 2.1. Início de cada sessão (checklist do agente)

- [ ] Ler `docs/progress/PROGRESS.md`.
- [ ] Verificar branch atual (`git status`, `git log -5`).
- [ ] Conferir TODO list pendente (se houver).
- [ ] Confirmar com o usuário: "vamos continuar em X ou trocar para Y?".

### 2.2. Fim de cada sessão (checklist do agente)

- [ ] Atualizar `PROGRESS.md` (concluído + em andamento + próximo + bloqueios).
- [ ] Commit (mesmo que WIP — usar `wip:` ou `git stash` com nota).
- [ ] Se houve decisão importante, registrar ADR ou linha em `decisions-log.md`.
- [ ] Resumo ao usuário com link para `PROGRESS.md`.

### 2.3. Quando trocar de tarefa no meio

- Salvar estado atual em `PROGRESS.md` (subseção "🚧 Em andamento").
- Commit WIP **OU** `git stash` com mensagem.
- Documentar **onde parou** e **próximo passo concreto**.

---

## 3. Princípios de Arquitetura (resumo aplicado)

| Princípio                  | Aplicação prática                                                  |
| -------------------------- | ------------------------------------------------------------------ |
| **Separation of Concerns** | Camadas isoladas: apresentação ≠ negócio ≠ dados                   |
| **Domain-Driven (light)**  | Cada feature/módulo = um domínio coeso                             |
| **Server-first** (web)     | Server Components/Actions por padrão; cliente só quando necessário |
| **Validação na fronteira** | Toda entrada externa passa por schema (zod/pydantic/etc.)          |
| **Imutabilidade**          | Não mutar props/estado; preferir map/filter/reduce                 |
| **Fail fast**              | Erros explícitos no servidor; mensagens amigáveis ao usuário       |
| **Defense in depth**       | Validação no client (UX) + server (segurança)                      |
| **Cost-aware**             | Avaliar libs antes de adicionar; evitar over-engineering           |
| **Observability-ready**    | Logs estruturados, IDs de correlação, métricas em pontos críticos  |

---

## 4. Quality Gates (resumo)

| Gate              | Quando            | Como                              |
| ----------------- | ----------------- | --------------------------------- |
| **DoR ✅**        | Antes de Design   | Checklist da Fase 1 atendida      |
| **Plan approved** | Antes de Execute  | TODO list aprovada                |
| **Code quality**  | Antes de Verify   | `lint && typecheck` ✅            |
| **Tests pass**    | Antes de Document | `test` ✅ + AC cobertos           |
| **Build OK**      | Antes de Handoff  | `build` ✅                        |
| **Docs synced**   | Antes de Handoff  | `PROGRESS.md` + feature doc       |
| **DoD ✅**        | Handoff           | Checklist da Fase 7 atendida      |

---

## 5. Anti-padrões (não fazer)

- ❌ Iniciar código sem ler `PROGRESS.md`.
- ❌ Encerrar sessão sem atualizar `PROGRESS.md`.
- ❌ Commits gigantes que misturam refactor + feature + fix.
- ❌ "Vou documentar depois" — documente junto.
- ❌ Suprimir testes ou regras para "ganhar tempo".
- ❌ Tomar decisão arquitetural sem registrar (ADR ou decisions-log).
- ❌ Pular fase VERIFY quando "tem certeza que funciona".
- ❌ Misturar dois domínios em um mesmo módulo de feature.
- ❌ Adicionar dependência sem justificativa documentada.

---

## 6. Bootstrap (primeira execução do projeto)

Antes da primeira story, criar:

1. `docs/progress/PROGRESS.md` (com release/sprint atual).
2. `docs/progress/decisions-log.md` (vazio, com cabeçalho).
3. `docs/architecture/overview.md`.
4. `docs/architecture/tech-stack.md`.
5. `docs/architecture/data-model.md` (mesmo que vazio inicialmente).
6. `docs/risks/risk-register.md`.
7. `docs/adr/0001-stack-inicial.md`.

Commit: `docs: bootstrap project context (progress, architecture, adr-0001)`.

---

## 7. Fluxograma (referência rápida)

```
       ┌─────────────────────┐
       │ Pedido do usuário   │
       └──────────┬──────────┘
                  ▼
           [1] CONTEXT  ── lê PROGRESS.md + backlog + ADRs + DoR
                  ▼
           [2] DESIGN   ── (se necessário) trade-offs + ADR + threat model
                  ▼
           [3] PLAN     ── TODO list + arquivos + AC
                  ▼
           [4] EXECUTE  ── schemas → dados → negócio → UI → testes
                  ▼
           [5] VERIFY   ── lint + typecheck + test + build + AC + a11y + sec
                  ▼               │
                  │ falhou ───────┘ (volta para EXECUTE)
                  ▼ ok
           [6] DOCUMENT ── feature doc + PROGRESS.md + ADR + risks
                  ▼
           [7] HANDOFF  ── DoD ✅ + resumo + próximo sugerido
```

---

**Resumo executivo**: Toda tarefa segue 7 fases (Context → Design → Plan → Execute → Verify → Document → Handoff). O `PROGRESS.md` é o cérebro do projeto — leia no início, atualize no fim. Decisões importantes viram ADR. Nada vai para HANDOFF sem `lint + typecheck + test + build` verde + DoD atendido.
