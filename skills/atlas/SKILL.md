---
name: atlas
description: Use como skill principal/orquestradora para qualquer tarefa no AI Dev Workflow. Classifica o pedido do usuario e aplica as skills especializadas corretas: workflow, onboarding, feature, bug-fix, refactor, architect, security, testing, documentation, git, stacks e distribuicao.
---

# Atlas - Skill Principal do AI Dev Workflow

Atlas e a porta de entrada unica para usar o AI Dev Workflow em qualquer agente com suporte a skills ou instrucoes reutilizaveis.

O usuario nao precisa escolher entre `new-feature`, `bug-fix`, `refactor`, `testing`, `security` ou stacks. Classifique o pedido, carregue/aplique as skills corretas e execute o fluxo mantendo contexto, qualidade e documentacao.

---

## Regra De Ouro

1. Comece por contexto: leia `AGENTS.md` e `docs/progress/PROGRESS.md` quando existirem.
2. Classifique a intencao antes de agir.
3. Use `workflow` como base para qualquer tarefa executavel.
4. Combine skills especializadas quando houver mais de uma dimensao relevante.
5. Se a classificacao ou escopo estiver ambiguo, faca uma pergunta curta antes de executar.
6. Nunca declare pronto sem verificar e documentar conforme o workflow.

---

## Saida Inicial Esperada

Antes de mudancas relevantes, informe de forma curta:

```text
Classificacao: <feature|bug|refactor|review|docs|arquitetura|spike|adocao|incidente|git|teste|sessao>
Skills aplicadas: <lista>
Proximo passo: <acao imediata>
```

Para tarefas simples, nao burocratize: aplique a classificacao internamente e execute.

---

## Matriz De Roteamento

| Pedido do usuario | Skills a aplicar |
| --- | --- |
| Inicio de sessao, entender projeto, continuar trabalho | `onboard-agent`, `workflow` |
| Nova feature, user story, comportamento novo | `new-feature`, `workflow`, `testing`, `documentation`, stack ativa |
| Bug, erro, regressao, comportamento inesperado | `bug-fix`, `testing`, `code-quality` |
| Refactor sem mudanca observavel | `refactor`, `testing`, `code-quality` |
| Arquitetura, trade-off, diagrama, decisao tecnica | `architect`, `adr`, `documentation` |
| Criar ou revisar ADR | `adr`, `architect`, `documentation` |
| Review de PR, diff ou branch | `code-review`, `testing`, `security` quando aplicavel |
| Testes, cobertura, harness, estrategia de QA | `testing`, `code-quality` |
| Documentacao, README, PROGRESS, feature docs | `documentation`, `workflow` |
| Git, branch, commit, PR, release | `git-workflow` |
| Adotar workflow em projeto existente | `adopt-existing-project`, `documentation` |
| Investigacao time-boxed, prova de conceito | `spike`, `documentation` |
| Incidente, bug grave escapado, retrospectiva | `postmortem`, `bug-fix`, `documentation` |
| Configurar runtime do agente, skills, agentes, plugins | skill de customizacao do agente quando existir, `documentation` |

---

## Modificadores Obrigatorios

Adicione estas skills quando o contexto pedir:

| Condicao | Skill adicional |
| --- | --- |
| Autenticacao, autorizacao, dados sensiveis, entrada externa, uploads, webhooks | `security` |
| Next.js App Router | `stack-nextjs` |
| Backend Node.js com TypeScript | `stack-node` |
| Python FastAPI/Django | `stack-python` |
| React Native / Expo | `stack-mobile` |
| Mudanca com decisao duradoura ou trade-off real | `adr` |
| Mudanca relevante do proprio AI Dev Workflow | `new-feature`, `testing`, `documentation` e spec em `specs/<id>-<slug>/` |

---

## Fluxo De Execucao

### 1. Contexto

Leia, nesta ordem, quando existirem:

1. `AGENTS.md`
2. `docs/progress/PROGRESS.md`
3. `docs/architecture/overview.md`
4. `docs/architecture/tech-stack.md`
5. ADRs relevantes em `docs/adr/`
6. Specs relevantes em `specs/`
7. Riscos em `docs/risks/risk-register.md`

### 2. Classificacao

Determine a intencao principal e os modificadores. Se houver conflito, priorize seguranca e preservacao de trabalho existente.

### 3. Plano

Para mudancas com 3+ passos, crie TODO list atomica. Para mudancas grandes, pare antes de executar e aguarde confirmacao.

### 4. Execucao

Execute incrementalmente usando as skills roteadas. Preserve mudancas do usuario e mantenha escopo minimo.

### 5. Verificacao

Rode os gates da stack:

```bash
lint && typecheck && test && build
```

Se algum comando nao existir ou estiver bloqueado, registre o bloqueio com evidencia.

### 6. Documentacao E Handoff

Atualize `PROGRESS.md` antes de encerrar. Atualize docs, ADRs, specs e riscos quando aplicavel.

---

## Regras De Distribuicao Do AI Dev Workflow

Quando a tarefa for evoluir este repositorio:

1. Trate `skills/` como artefato oficial agnostico de agente do produto.
2. Mudancas relevantes em skills devem ser distribuiveis por scripts e CLI.
3. Adicione harness para garantir que assets criticos sao copiados/embutidos.
4. Atualize README e getting-started quando mudar a experiencia de instalacao ou uso.
5. Registre progresso em `docs/progress/PROGRESS.md`.

---

## Anti-padroes

- Pedir ao usuario para escolher outra skill quando `atlas` consegue rotear.
- Aplicar uma skill de stack sem detectar ou confirmar a stack.
- Corrigir bug sem teste de regressao.
- Refatorar junto com feature ou bug fix.
- Criar ADR sem alternativas e consequencias.
- Encerrar sessao sem atualizar `PROGRESS.md`.
- Declarar pronto com gates pendentes sem explicar o bloqueio.
