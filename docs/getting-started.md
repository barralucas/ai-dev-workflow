# Getting Started

Como adotar o **AI Dev Workflow** em um projeto novo ou existente, em 5 minutos.

## Projeto novo

### 1. Rode o bootstrap

```bash
cd /caminho/para/seu/projeto-vazio
bash /caminho/para/ai-dev-workflow/scripts/bootstrap.sh
```

O script vai:

- Copiar `.github/`, `AGENTS.md`, `CLAUDE.md`.
- Copiar `templates/docs/` para `docs/`.
- Perguntar a stack (Next.js, Node backend, Python, mobile, custom).
- Substituir `{{PROJECT_NAME}}` nos templates.
- Sugerir o primeiro commit.

### 2. Preencha o essencial

Edite na ordem:

1. `docs/progress/PROGRESS.md` — defina sprint atual e release alvo.
2. `docs/architecture/tech-stack.md` — preencha versões reais.
3. `docs/adr/0001-stack-inicial.md` — finalize a partir do template.
4. `docs/user-stories/backlog.md` — comece com as primeiras stories.

### 3. Verifique o agente

Abra o projeto no VS Code e peça ao Copilot:

> "Faça onboarding deste projeto e proponha o próximo passo."

Se ele ler `PROGRESS.md` e propor algo coerente, está funcionando.

### 4. Commit inicial

```bash
git add -A
git commit -m "chore: bootstrap ai-dev-workflow"
```

---

## Projeto existente

### 1. Comece pelo essencial

Copie só o mínimo:

```bash
cp -r ai-dev-workflow/.github/instructions/workflow.instructions.md \
      seu-projeto/.github/instructions/
cp ai-dev-workflow/templates/docs/progress/PROGRESS.md \
   seu-projeto/docs/progress/
cp ai-dev-workflow/AGENTS.md seu-projeto/
```

### 2. Inventarie o estado atual

Peça ao agente:

> "Leia este projeto e preencha `docs/progress/PROGRESS.md` com o que já foi entregue (✅), o que está em andamento (🚧) e os próximos sugeridos."

### 3. ADRs retroativas

Adicione 1-3 ADRs explicando **as decisões já tomadas** (stack, padrão de pastas, libs principais). Não precisa ser perfeito — registre o "porquê" do que existe hoje.

### 4. Adote incrementalmente

A cada nova feature, comece a usar o fluxo completo. Não tente "retrofit" tudo.

---

## Sanidade do setup

Faça este teste — peça ao agente:

> "Crie a feature US-001: <título>". Siga o workflow.

Espere ver:

1. Ele lê `PROGRESS.md`.
2. Confirma DoR (ou pergunta o que falta).
3. Apresenta plano com TODO.
4. Executa em commits pequenos.
5. Roda quality gates.
6. Atualiza `PROGRESS.md` + cria feature doc.
7. Entrega resumo no formato padrão.

Se faltar algum passo, **revise as instruções** — provavelmente o `applyTo` está errado ou `AGENTS.md` não está sendo lido.
