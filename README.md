# AI Dev Workflow

> Um **workflow opinativo, agnóstico de stack** para construir software com auxílio de **agentes de IA** (GitHub Copilot, Claude, Cursor, Codex CLI etc.) **sem perder contexto, qualidade ou rastreabilidade**.

Este repositório é uma **biblioteca de instruções, templates e prompts** que você copia (ou submodula) para qualquer novo projeto — frontend, backend, mobile, dados, infraestrutura — e ganha imediatamente:

- Um **fluxo de 7 fases** (Context → Design → Plan → Execute → Verify → Document → Handoff) que o agente segue rigorosamente.
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
- [O fluxo em 7 fases (resumo)](#o-fluxo-em-7-fases-resumo)
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
10. **Idioma**: código e identificadores em **inglês**; docs e mensagens ao usuário no **idioma do projeto** (default PT-BR — ajuste no template).

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
└── scripts/
    ├── bootstrap.sh                   ← projeto novo (zera estrutura)
    └── adopt.sh                       ← projeto existente (preserva tudo, infere stack)
```

---

## Como usar em um novo projeto

### Opção 1 — Script de bootstrap (recomendada)

```bash
# A partir do diretório do seu novo projeto vazio:
bash /caminho/para/ai-dev-workflow/scripts/bootstrap.sh
```

O script:

1. Copia `.github/`, `AGENTS.md`, `CLAUDE.md` para a raiz do projeto.
2. Copia `templates/docs/` para `docs/` no projeto.
3. Pergunta qual(is) stack(s) habilitar e remove as instruções de stack não usadas.
4. Preenche placeholders básicos (`{{PROJECT_NAME}}`, `{{LANGUAGE}}`).
5. Cria commit inicial: `chore: bootstrap ai-dev-workflow`.

### Opção 2 — Cópia manual

```bash
cp -r ai-dev-workflow/.github SEU_PROJETO/
cp ai-dev-workflow/AGENTS.md ai-dev-workflow/CLAUDE.md SEU_PROJETO/
cp -r ai-dev-workflow/templates/docs SEU_PROJETO/
```

Depois:

- Edite `docs/progress/PROGRESS.md` (defina sprint atual).
- Edite `docs/architecture/tech-stack.md` (cole sua stack).
- Crie `docs/adr/0001-stack-inicial.md` a partir do exemplo.
- Habilite **uma** stack em `.github/instructions/stacks/` (mantenha apenas o `.instructions.md` que se aplica) **ou** ajuste o `applyTo` para múltiplas.
- Commit: `chore: bootstrap ai-dev-workflow`.

### Opção 3 — Submódulo Git (para puxar updates)

```bash
git submodule add https://github.com/SEU_USUARIO/ai-dev-workflow .ai-workflow
ln -s .ai-workflow/.github .github
ln -s .ai-workflow/templates/docs docs
```

> Útil para times grandes que querem propagar melhorias de processo automaticamente.

---

## Como usar em um projeto existente

> O fluxo abaixo **não toca código** existente. Ele só **mapeia o que já existe** e cria o esqueleto mínimo de contexto. Refatorações ficam para próximas sessões, sob o fluxo completo.

### Opção 1 — Script `adopt.sh` (recomendada)

```bash
cd /caminho/para/seu/projeto-existente
bash /caminho/para/ai-dev-workflow/scripts/adopt.sh
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

```bash
cp ai-dev-workflow/.github/instructions/workflow.instructions.md \
   seu-projeto/.github/instructions/
cp ai-dev-workflow/templates/docs/progress/PROGRESS.md \
   seu-projeto/docs/progress/
cp ai-dev-workflow/AGENTS.md seu-projeto/
```

Depois peça ao agente:

> "Siga o prompt `adopt-existing-project` para popular o contexto a partir do código atual."

### Princípios da adoção

- **Não refatore durante a adoção.** Adoção = ler + documentar.
- **ADRs retroativas** registram o "porquê" do que existe (não precisa ser perfeito).
- **Adote incrementalmente**: o fluxo completo passa a valer das próximas features em diante.

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
