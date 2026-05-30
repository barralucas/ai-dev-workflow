# AI Dev Workflow

> Um **workflow opinativo, agnóstico de stack** para construir software com auxílio de **agentes de IA** (GitHub Copilot, Claude, Cursor, Codex CLI etc.) **sem perder contexto, qualidade ou rastreabilidade**.

Este repositório é uma **biblioteca de instruções, templates e prompts** que você copia (ou submodula) para qualquer novo projeto — frontend, backend, mobile, dados, infraestrutura — e ganha imediatamente:

- Um **fluxo de 7 fases** (Context → Design → Plan → Execute → Verify → Document → Handoff) que o agente segue rigorosamente.
- Uma camada de **Spec-Driven Development** para mudanças relevantes: `spec.md`, `plan.md`, `tasks.md`, contratos e quickstart versionados em `specs/<id>-<slug>/`.
- **Harnesses automatizados** para validar CLI, templates, workflow e contratos antes de confiar em automação agentic.
- Um **painel vivo de progresso** (`PROGRESS.md`) que atravessa sessões, evita "perda de memória" do agente e serve de onboarding em < 10 min.
- **ADRs**, **feature docs**, **risk register**, **postmortems**, **spikes** e **decisions log** prontos.
- Padrões de **qualidade, segurança (OWASP), testes, git e documentação** que o agente é obrigado a respeitar.
- Pontos de extensão por **stack** (Next.js, Node backend, Python, mobile, data) sem reescrever o núcleo.

> **Para quem é**: devs que trabalham com agentes de IA e querem que cada sessão produza código sustentável, decisões registradas e contexto preservado — em vez de "vibe coding" que ninguém entende uma semana depois.

---

## Sumário

- [Por que este workflow existe](#por-que-este-workflow-existe)
- [Princípios](#princípios)
- [Estrutura do repositório](#estrutura-do-repositório)
- [Como usar em um novo projeto](#como-usar-em-um-novo-projeto)
- [Como usar em um projeto existente](#como-usar-em-um-projeto-existente)
- [Como usar no dia a dia](#como-usar-no-dia-a-dia)
- [O fluxo em 7 fases (resumo)](#o-fluxo-em-7-fases-resumo)
- [Spec-Driven Development e harnesses](#spec-driven-development-e-harnesses)
- [Arquivos que o agente lê automaticamente](#arquivos-que-o-agente-lê-automaticamente)
- [Customizando por stack](#customizando-por-stack)
- [Convenções de pastas em projetos consumidores](#convenções-de-pastas-em-projetos-consumidores)
- [Compatibilidade com agentes](#compatibilidade-com-agentes)
- [Atualizando o workflow](#atualizando-o-workflow)
- [FAQ](#faq)
- [Contribuindo](#contribuindo)
- [Licença](#licença)

---

## Por que este workflow existe

Trabalhar com agentes de IA tem três falhas recorrentes:

1. **Perda de contexto entre sessões** — o agente esquece o que foi feito, refaz coisas, contradiz decisões anteriores.
2. **Decisões implícitas** — escolhas arquiteturais somem no histórico do chat e ninguém reconstrói o "porquê".
3. **Qualidade inconsistente** — sem gates explícitos, o agente entrega código sem testes, sem validação de segurança, sem doc.

Este workflow resolve as três:

| Problema             | Solução                                                              |
| -------------------- | -------------------------------------------------------------------- |
| Perda de contexto    | `PROGRESS.md` lido no início e atualizado no fim de toda sessão      |
| Decisões implícitas  | **ADRs** obrigatórias para escolhas relevantes + `decisions-log.md`  |
| Qualidade inconsistente | **Quality gates** (lint, typecheck, test, build) antes de qualquer handoff |

---

## Princípios

1. **Contexto é código de primeira classe** — `docs/progress/PROGRESS.md` é tão importante quanto `package.json`.
2. **Decisão sem ADR não existiu** — se vai impactar 2+ módulos ou tem trade-off real, vira ADR.
3. **Pequenos commits, mensagens claras** — Conventional Commits referenciando a story.
4. **Testes antes do handoff** — caminho feliz + ao menos um caso de erro por story.
5. **Documentação junto com código** — "documento depois" = não documenta.
6. **Server-first / fronteira validada** — toda entrada externa é validada com schema (zod, pydantic, etc.).
7. **Defesa em profundidade** — segurança não é uma camada, são várias.
8. **YAGNI > DRY > clever code** — não construa o que não tem story; só extraia abstração após 2-3 usos reais.
9. **Stack-agnóstico no núcleo, plugável nas pontas** — o fluxo é o mesmo; o que muda são os comandos e padrões da stack.
10. **Specs vivas para mudanças relevantes** — intenção, plano, tasks e contratos devem ficar versionados, não apenas no chat.
11. **Harness antes de confiança** — automações, templates e comandos precisam de testes determinísticos antes de evals agentic.
12. **Idioma**: código e identificadores em **inglês**; docs e mensagens ao usuário no **idioma do projeto** (default PT-BR — ajuste no template).

---

## Estrutura do repositório

```
ai-dev-workflow/
├── README.md                          ← você está aqui
├── CHANGELOG.md                       ← versionamento do workflow
├── LICENSE
│
├── AGENTS.md                          ← entrada universal para agentes (Codex, etc.)
├── CLAUDE.md                          ← reexporta AGENTS.md para Claude
│
├── .github/
│   ├── copilot-instructions.md        ← instruções globais para Copilot
│   ├── instructions/                  ← regras carregadas por padrão de path
│   │   ├── workflow.instructions.md   ← ★ as 7 fases (núcleo)
│   │   ├── code-quality.instructions.md
│   │   ├── security.instructions.md   ← OWASP + threat modeling lite
│   │   ├── testing.instructions.md
│   │   ├── documentation.instructions.md
│   │   ├── git-workflow.instructions.md
│   │   └── stacks/                    ← adendos por stack (opte por 1+ no projeto)
│   │       ├── README.md
│   │       ├── nextjs.instructions.md
│   │       ├── node-backend.instructions.md
│   │       ├── python.instructions.md
│   │       └── mobile.instructions.md
│   ├── prompts/                       ← prompts reutilizáveis (slash-commands)
│   │   ├── new-feature.prompt.md
│   │   ├── bug-fix.prompt.md
│   │   ├── refactor.prompt.md
│   │   ├── adr.prompt.md
│   │   ├── code-review.prompt.md
│   │   ├── postmortem.prompt.md
│   │   ├── onboard-agent.prompt.md
│   │   └── adopt-existing-project.prompt.md  ← adoção em projeto que já tem código
│   └── chatmodes/
│       └── architect.chatmode.md      ← modo "arquiteto" (read-only + ADR)
│
├── templates/                         ← copie isto para `docs/` do projeto novo
│   ├── README.template.md
│   └── docs/
│       ├── progress/
│       │   ├── PROGRESS.md            ← ★ painel vivo
│       │   └── decisions-log.md
│       ├── adr/
│       │   ├── 0000-template.md
│       │   └── 0001-stack-inicial.md  ← exemplo preenchido
│       ├── architecture/
│       │   ├── overview.md            ← C4 nível 1-2
│       │   ├── tech-stack.md
│       │   └── data-model.md
│       ├── features/
│       │   └── _template.md
│       ├── user-stories/
│       │   └── backlog.md
│       ├── risks/
│       │   └── risk-register.md       ← lacuna comum: catálogo de riscos
│       ├── postmortem/
│       │   └── _template.md           ← incidentes & retrospectivas
│       └── spikes/
│           └── _template.md           ← investigações time-boxed
│
├── docs/                              ← documentação DESTE workflow
│   ├── getting-started.md
│   ├── customizing-for-your-stack.md
│   ├── glossary.md
│   └── faq.md
│
├── specs/                             ← specs vivas DESTE workflow
│   └── 001-sdd-and-harness/
│       ├── spec.md                    ← o que/por que
│       ├── plan.md                    ← como
│       ├── tasks.md                   ← trabalho atômico
│       ├── contracts/                 ← contratos verificáveis
│       └── quickstart.md              ← validação local
│
└── scripts/
    ├── bootstrap.sh                   ← projeto novo (zera estrutura)
    └── adopt.sh                       ← projeto existente (preserva tudo, infere stack)
```

---

## Como usar em um novo projeto

### Opção 1 — Script de bootstrap (recomendada)

**Linux / macOS**
```bash
# A partir do diretório do seu novo projeto vazio:
bash /caminho/para/ai-dev-workflow/scripts/bootstrap.sh
```

**Windows — WSL (recomendado)** · [Como instalar o WSL](https://aka.ms/wslstore)
```powershell
# No PowerShell, navegue até o projeto e rode via WSL:
cd C:\caminho\para\seu\projeto-vazio
wsl bash /mnt/c/caminho/para/ai-dev-workflow/scripts/bootstrap.sh
```

**Windows — Git Bash** · [Git for Windows](https://gitforwindows.org/)
```bash
# Abra o Git Bash e execute:
bash /c/caminho/para/ai-dev-workflow/scripts/bootstrap.sh
```

O script:

1. Copia `.github/`, `AGENTS.md`, `CLAUDE.md` para a raiz do projeto.
2. Copia `templates/docs/` para `docs/` no projeto.
3. Pergunta qual(is) stack(s) habilitar e remove as instruções de stack não usadas.
4. Preenche placeholders básicos (`{{PROJECT_NAME}}`, `{{LANGUAGE}}`).
5. Cria commit inicial: `chore: bootstrap ai-dev-workflow`.

### Opção 2 — Cópia manual

**Linux / macOS**
```bash
cp -r ai-dev-workflow/.github SEU_PROJETO/
cp ai-dev-workflow/AGENTS.md ai-dev-workflow/CLAUDE.md SEU_PROJETO/
cp -r ai-dev-workflow/templates/docs SEU_PROJETO/
```

**Windows — PowerShell**
```powershell
Copy-Item -Recurse ai-dev-workflow\.github SEU_PROJETO\
Copy-Item ai-dev-workflow\AGENTS.md, ai-dev-workflow\CLAUDE.md SEU_PROJETO\
Copy-Item -Recurse ai-dev-workflow\templates\docs SEU_PROJETO\
```

Depois:

- Edite `docs/progress/PROGRESS.md` (defina sprint atual).
- Edite `docs/architecture/tech-stack.md` (cole sua stack).
- Crie `docs/adr/0001-stack-inicial.md` a partir do exemplo.
- Habilite **uma** stack em `.github/instructions/stacks/` (mantenha apenas o `.instructions.md` que se aplica) **ou** ajuste o `applyTo` para múltiplas.
- Commit: `chore: bootstrap ai-dev-workflow`.

### Opção 3 — Submódulo Git (para puxar updates)

**Linux / macOS**
```bash
git submodule add https://github.com/SEU_USUARIO/ai-dev-workflow .ai-workflow
ln -s .ai-workflow/.github .github
ln -s .ai-workflow/templates/docs docs
```

**Windows — PowerShell (execute como Administrador)**
```powershell
git submodule add https://github.com/SEU_USUARIO/ai-dev-workflow .ai-workflow
# Use junctions no lugar de symlinks:
cmd /c mklink /J .github .ai-workflow\.github
cmd /c mklink /J docs .ai-workflow\templates\docs
```

> Útil para times grandes que querem propagar melhorias de processo automaticamente.

---

## Como usar em um projeto existente

> O fluxo abaixo **não toca código** existente. Ele só **mapeia o que já existe** e cria o esqueleto mínimo de contexto. Refatorações ficam para próximas sessões, sob o fluxo completo.

### Opção 1 — Script `adopt.sh` (recomendada)

**Linux / macOS**
```bash
cd /caminho/para/seu/projeto-existente
bash /caminho/para/ai-dev-workflow/scripts/adopt.sh
```

**Windows — WSL (recomendado)** · [Como instalar o WSL](https://aka.ms/wslstore)
```powershell
# No PowerShell, navegue até o projeto e rode via WSL:
cd C:\caminho\para\seu\projeto-existente
wsl bash /mnt/c/caminho/para/ai-dev-workflow/scripts/adopt.sh
```

**Windows — Git Bash** · [Git for Windows](https://gitforwindows.org/)
```bash
# Abra o Git Bash e execute:
bash /c/caminho/para/ai-dev-workflow/scripts/adopt.sh
```

O script:

1. Faz **inventário** do projeto (manifests, git, docs existentes).
2. **Detecta a stack automaticamente** (Next.js, Node backend, Python, mobile) a partir de `package.json`/`pyproject.toml`/`app.json`.
3. Copia `.github/` **sem sobrescrever** nada existente (`cp -n`).
4. Cria apenas o **esqueleto mínimo** de `docs/` (PROGRESS.md, decisions-log.md, adr/template). Preserva tudo que já existe.
5. **Não toca** em `README.md`, `.gitignore`, `.env.example` se já existirem.

Flags úteis:

- `--dry-run` — mostra o que faria sem escrever.
- `--minimal` — instala só `workflow.instructions.md` + `PROGRESS.md` + `AGENTS.md`.
- `--stack X` — força stack (`nextjs|node-backend|python|mobile|none`).
- `--yes` — pula confirmações.

Depois do script, **rode o prompt** [`/adopt-existing-project`](.github/prompts/adopt-existing-project.prompt.md) com seu agente. Ele:

- Inventaria o código (read-only).
- Pede sua validação do que encontrou.
- Popula `PROGRESS.md` retroativo, `tech-stack.md`, `overview.md`.
- Cria ADR-0001 retroativa da stack atual.
- Cataloga riscos visíveis.

### Opção 2 — Manual (se preferir controle total)

**Linux / macOS**
```bash
cp ai-dev-workflow/.github/instructions/workflow.instructions.md \
   seu-projeto/.github/instructions/
cp ai-dev-workflow/templates/docs/progress/PROGRESS.md \
   seu-projeto/docs/progress/
cp ai-dev-workflow/AGENTS.md seu-projeto/
```

**Windows — PowerShell**
```powershell
Copy-Item ai-dev-workflow\.github\instructions\workflow.instructions.md `
          seu-projeto\.github\instructions\
Copy-Item ai-dev-workflow\templates\docs\progress\PROGRESS.md `
          seu-projeto\docs\progress\
Copy-Item ai-dev-workflow\AGENTS.md seu-projeto\
```

Depois peça ao agente:

> "Siga o prompt `adopt-existing-project` para popular o contexto a partir do código atual."

### Princípios da adoção

- **Não refatore durante a adoção.** Adoção = ler + documentar.
- **ADRs retroativas** registram o "porquê" do que existe (não precisa ser perfeito).
- **Adote incrementalmente**: o fluxo completo passa a valer das próximas features em diante.

---

## Como usar no dia a dia

> Esta seção mostra **exatamente o que você digita** no chat do agente para cada cenário comum. Os prompts vivem em [`.github/prompts/`](.github/prompts/) e funcionam como **slash commands** no Copilot Chat (VS Code) — em outros agentes, basta citar o nome do prompt.

### O ciclo de uma sessão (sempre o mesmo)

```
1. Abrir o projeto
2. Pedir ao agente: "continue de onde paramos"  → ele lê PROGRESS.md
3. Escolher uma tarefa (feature, bug, refactor, ADR…)
4. Disparar o prompt correspondente (/new-feature, /bug-fix, etc.)
5. Acompanhar as 7 fases: revisar plano antes de Execute, conferir gates antes de Handoff
6. Antes de fechar o editor: "atualize o PROGRESS.md e me dê o resumo"
```

> **Regra de ouro**: comece toda sessão com **"leia o `PROGRESS.md` e me diga onde paramos"** e termine com **"atualize o `PROGRESS.md` antes do handoff"**. Isso sozinho elimina 80% da "perda de memória" do agente.

### Slash commands disponíveis

No VS Code com Copilot Chat, digite `/` e selecione. Em outros agentes, peça pelo nome.

| Comando                       | Quando usar                                              | O que ele faz                                                                  |
| ----------------------------- | -------------------------------------------------------- | ------------------------------------------------------------------------------ |
| `/onboard-agent`              | Primeira sessão num projeto / agente novo                | Lê `PROGRESS.md` + arquitetura + ADRs e propõe próximo passo coerente          |
| `/new-feature`                | Implementar uma user story do backlog                    | Roda as 7 fases (Context → Handoff) com gates explícitos                       |
| `/bug-fix`                    | Corrigir um bug                                          | Reproduz, escreve teste de regressão **primeiro**, corrige, valida             |
| `/refactor`                   | Mudar estrutura sem mudar comportamento                  | Garante testes verdes antes/depois; sem mistura com feature                    |
| `/adr`                        | Registrar uma decisão arquitetural                       | Cria `docs/adr/NNNN-titulo.md` com contexto, alternativas, consequências       |
| `/code-review`                | Revisar PR ou diff (modo adversarial)                    | Procura segurança, testes faltantes, anti-padrões, doc desatualizada           |
| `/postmortem`                 | Após um incidente em prod                                | Cria `docs/postmortem/YYYY-MM-DD-titulo.md` com causa-raiz e ações             |
| `/adopt-existing-project`     | Primeira vez adotando o workflow num projeto que já existe | Inventaria código (read-only), valida com você, popula contexto retroativo   |

### Receitas prontas (copie e cole no chat)

#### Começar o dia

> Leia o `docs/progress/PROGRESS.md` e me diga: o que está em andamento, o que está bloqueado, e qual é o próximo passo recomendado.

#### Implementar uma story do backlog

> `/new-feature` — quero implementar a **US-005: Entrar em uma partida** descrita em `docs/user-stories/backlog.md`. Siga as 7 fases. Pare antes de Execute para eu aprovar o plano.

#### Corrigir um bug

> `/bug-fix` — usuários relataram que o link mágico expirado mostra erro 500 em vez de mensagem amigável. Reproduza, escreva o teste de regressão antes do fix, e atualize o `risk-register.md` se for falha sistêmica.

#### Tomar uma decisão arquitetural

> `/adr` — preciso decidir entre **Drizzle** e **Prisma** como ORM. Liste prós/contras considerando nosso `tech-stack.md`, recomende uma escolha e gere o ADR-000X em `docs/adr/`.

#### Revisar um PR antes de aprovar

> `/code-review` — analise o diff atual (`git diff main`) em modo adversarial. Foque em: segurança (OWASP), testes faltantes, validação de fronteira, doc desatualizada.

#### Trocar de tarefa no meio da sessão

> Salve o estado atual em `PROGRESS.md` (subseção "🚧 Em andamento") com o próximo passo concreto, faça commit `wip:` e me prepare para começar a US-007.

#### Encerrar a sessão

> Atualize o `PROGRESS.md`: mova o que terminei para ✅, registre o que ficou em 🚧 e qual é o próximo. Confirme que `lint && typecheck && test && build` passam. Me dê o resumo no formato padrão de Handoff.

### O que **você** faz vs. o que o **agente** faz

| Etapa                         | Você                                          | Agente                                          |
| ----------------------------- | --------------------------------------------- | ----------------------------------------------- |
| Definir prioridade da sessão  | ✅ escolhe a story/bug                        | sugere próximos com base no `PROGRESS.md`       |
| Critérios de aceite           | ✅ valida (DoR)                               | propõe se faltarem; pergunta se ambíguo         |
| Design / trade-offs           | ✅ aprova alternativa                         | apresenta ≥ 2 alternativas + ADR draft          |
| Plano (TODO list)             | ✅ aprova antes de Execute                    | gera plano com arquivos previstos               |
| Código                        | revisa diff                                   | ✅ executa em commits pequenos                  |
| Quality gates                 | confere status                                | ✅ roda `lint && typecheck && test && build`    |
| Doc da feature + `PROGRESS.md`| revisa                                        | ✅ atualiza antes de declarar pronto            |
| Merge / push                  | ✅ aprova e executa                           | sugere mensagem de PR no formato padrão         |

### Sinais de que está dando certo

- ✅ Toda sessão começa com um **resumo coerente** do `PROGRESS.md`.
- ✅ O agente **pede aprovação do plano** antes de codar features grandes.
- ✅ Commits são **pequenos** e referenciam stories (`feat(us-005): ...`).
- ✅ O agente **se recusa** a declarar pronto se algum gate falhou.
- ✅ Decisões com trade-off **viram ADR** sem você precisar pedir.
- ✅ Outro dev (ou você daqui a 2 semanas) entende o estado do projeto em **< 10 min**.

### Sinais de que algo está errado (e o que fazer)

| Sintoma                                                    | Causa provável                                    | Correção                                                              |
| ---------------------------------------------------------- | ------------------------------------------------- | --------------------------------------------------------------------- |
| Agente não menciona `PROGRESS.md`                          | `AGENTS.md` ou `copilot-instructions.md` não lido | Confira se os arquivos estão na raiz e reinicie a sessão              |
| Agente vai direto ao código sem plano                      | Falta o prompt `/new-feature`                     | Use o slash command em vez de pedir "implementa X"                    |
| Comandos da stack errados (ex.: `npm` num projeto `pnpm`)  | Stack errada ativa em `instructions/stacks/`      | Mantenha só o `.instructions.md` da sua stack; ajuste `applyTo`       |
| Agente "esquece" decisões anteriores                       | ADRs não estão sendo lidas                        | Garanta que existem em `docs/adr/` e cite-as no prompt se necessário  |
| Sessões longas viram bagunça                               | `PROGRESS.md` desatualizado                       | Force update no meio da sessão; trate-o como código de primeira classe|

---

## O fluxo em 7 fases (resumo)

```
┌─────────┐   ┌────────┐   ┌─────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌─────────┐
│ CONTEXT │ → │ DESIGN │ → │ PLAN    │ → │ EXECUTE  │ → │ VERIFY   │ → │ DOCUMENT │ → │ HANDOFF │
└─────────┘   └────────┘   └─────────┘   └──────────┘   └──────────┘   └──────────┘   └─────────┘
```

| Fase         | Entrada                | Saída                          | Gate (não passa sem isto)               |
| ------------ | ---------------------- | ------------------------------ | --------------------------------------- |
| **Context**  | Pedido do usuário      | Resumo de 1 frase do escopo    | `PROGRESS.md` lido + DoR atendido       |
| **Design**   | Contexto consolidado   | Trade-offs + ADR (se relevante)| Alternativas explícitas; segurança e perf consideradas |
| **Plan**     | Design aprovado        | TODO list + arquivos previstos | Plano confirmado pelo usuário (se grande) |
| **Execute**  | TODO list              | Código incremental             | Commits pequenos; sem `any`/silenciar erros |
| **Verify**   | Código pronto          | Pipeline verde                 | `lint && typecheck && test && build` ✅ |
| **Document** | Feature funcionando    | `PROGRESS.md` + feature doc    | Outro dev entende em < 10 min           |
| **Handoff**  | Tudo verde + documentado | Resumo padrão + próximo passo | DoD 100% atendido                       |

Detalhes completos em [`.github/instructions/workflow.instructions.md`](.github/instructions/workflow.instructions.md).

---

## Spec-Driven Development e harnesses

Para mudanças relevantes neste próprio repositório e em projetos consumidores que adotarem essa prática, use `specs/<id>-<slug>/` como fonte de verdade do intento.

Estrutura recomendada:

```
specs/001-minha-feature/
├── spec.md          # o que, por que, usuários, requisitos e critérios de aceite
├── plan.md          # arquitetura, alternativas, trade-offs e arquivos afetados
├── tasks.md         # tarefas atômicas, ordenadas e verificáveis
├── contracts/       # CLI/API/schema/output esperado
└── quickstart.md    # como validar localmente
```

Use specs quando a mudança:

- Afeta 2+ módulos, templates ou comandos.
- Introduz ou altera comportamento público de CLI/API/UI.
- Exige decisão com trade-off, contrato ou migração.
- Precisa ser retomada por outro agente sem depender do histórico do chat.

Harness engineering neste projeto significa transformar comportamento esperado em testes reprodutíveis antes de depender de revisão manual ou evals com LLM:

- **CLI harness**: comandos como `init`, `adopt`, `doctor` e `verify` rodam contra diretórios temporários.
- **Template harness**: templates obrigatórios são embutidos e geram a árvore esperada.
- **Workflow harness**: invariantes como ordem das fases e transições são testados.
- **Contract harness**: contratos em `specs/*/contracts/` guiam testes e outputs esperados.

Pipeline local recomendado para este repositório:

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace
```

---

## Arquivos que o agente lê automaticamente

Cada agente carrega arquivos diferentes. Esta tabela mostra qual arquivo do workflow é lido por padrão por qual ferramenta:

| Arquivo                                | GitHub Copilot | Claude (CLI/Code) | Cursor | Codex CLI |
| -------------------------------------- | :------------: | :---------------: | :----: | :-------: |
| `.github/copilot-instructions.md`      | ✅             | —                 | ✅¹    | —         |
| `.github/instructions/*.md`            | ✅ (por `applyTo`) | —             | parcial | —      |
| `AGENTS.md`                            | —              | via `CLAUDE.md`   | —      | ✅        |
| `CLAUDE.md`                            | —              | ✅                | —      | —         |
| `.cursorrules` (se você gerar)         | —              | —                 | ✅     | —         |

¹ Cursor lê `.github/copilot-instructions.md` em projetos que o tenham; ajuste conforme sua versão.

> **Dica**: mantenha `AGENTS.md` curto e que aponte para os arquivos de detalhe — assim você não duplica regras.

---

## Customizando por stack

O **núcleo é igual para todo projeto**. O que muda é a stack: comandos (`pnpm` vs `cargo` vs `pytest`), padrões de pasta, frameworks de teste, política de cache.

Em `.github/instructions/stacks/` há adendos para:

- `nextjs.instructions.md` — Next.js 15+ App Router, RSC, Server Actions.
- `node-backend.instructions.md` — Express/Fastify/Nest + TypeScript.
- `python.instructions.md` — FastAPI/Django + ruff + pytest.
- `mobile.instructions.md` — React Native / Expo (ou nativo).

**Como ativar uma stack**:

1. No projeto consumidor, mantenha **apenas** o(s) arquivo(s) da(s) stack(s) usada(s) em `.github/instructions/stacks/`.
2. Edite o frontmatter `applyTo` para casar com seus diretórios (ex.: `applyTo: 'src/api/**'`).
3. Adapte comandos no `Quick commands` do arquivo.

**Não tem sua stack?** Copie o template `stacks/_template.instructions.md` (em breve) e descreva: comandos, estrutura de pastas, validação de fronteira, framework de testes, padrão de erro.

---

## Convenções de pastas em projetos consumidores

Após o bootstrap, espere ver no projeto:

```
SEU_PROJETO/
├── AGENTS.md                          ← entrypoint para agentes
├── CLAUDE.md                          ← @AGENTS.md
├── .github/
│   ├── copilot-instructions.md
│   ├── instructions/
│   └── prompts/
├── docs/
│   ├── progress/PROGRESS.md           ← ★ painel
│   ├── adr/                           ← decisões arquiteturais
│   ├── architecture/                  ← visão, stack, dados
│   ├── features/                      ← 1 doc por feature entregue
│   ├── user-stories/backlog.md
│   ├── risks/risk-register.md
│   ├── postmortem/
│   └── spikes/
└── src/                               ← seu código
```

---

## Compatibilidade com agentes

Testado/projetado para:

- **GitHub Copilot Chat** (VS Code) — usa `.github/copilot-instructions.md` + `.github/instructions/*` + `.github/prompts/*`.
- **Claude Code / Claude CLI** — usa `CLAUDE.md` (que reexporta `AGENTS.md`).
- **Codex CLI / OpenAI Agents** — usam `AGENTS.md`.
- **Cursor** — usa `.github/copilot-instructions.md` (ou `.cursorrules` se você gerar a partir dele).

Princípio: **um único `AGENTS.md` curto que aponta para o resto** evita duplicação. `CLAUDE.md` é apenas `@AGENTS.md`.

---

## Atualizando o workflow

- **Versionamento semântico** em `CHANGELOG.md`. Bump:
  - `MAJOR` quando uma fase do fluxo muda (quebra padrões existentes).
  - `MINOR` quando uma nova instrução/template/prompt é adicionado.
  - `PATCH` quando há correção/clareza.
- Projetos que usam **submódulo** dão `git submodule update --remote` para puxar.
- Projetos que **copiaram** podem rodar `scripts/bootstrap.sh --update` (apenas arquivos não modificados).

---

## FAQ

**Por que `PROGRESS.md` em vez de issues do GitHub?**
Issues são ótimas, mas o agente não as lê automaticamente. `PROGRESS.md` mora no repo, é versionado, e é a primeira coisa que o agente vê. Use issues como fonte primária se preferir, mas mantenha o `PROGRESS.md` espelhando o estado atual.

**Posso usar sem agente de IA?**
Sim. O workflow é boa engenharia independente de quem executa.

**Não é overengineering pra projeto pequeno?**
Use o subset mínimo: `PROGRESS.md` + `workflow.instructions.md` + ADRs. O resto entra quando precisa.

**Idioma misto incomoda?**
Recomendamos código em inglês (universal) e docs no idioma do time. Se seu time é 100% EN, mude o template.

**Tenho que usar Conventional Commits?**
Não obrigatoriamente, mas o changelog automático (semantic-release etc.) depende disso. Se trocar, atualize `git-workflow.instructions.md`.

Mais respostas em [`docs/faq.md`](docs/faq.md).

---

## Contribuindo

PRs bem-vindos. Antes:

1. Atualize o `CHANGELOG.md` (categoria `Added` / `Changed` / `Fixed` / `Deprecated`).
2. Se mudar uma fase do fluxo, registre uma ADR neste próprio repo (`docs/adr/`).
3. Teste em ao menos 1 projeto real antes de propor.

---

## Licença

[MIT](LICENSE)
