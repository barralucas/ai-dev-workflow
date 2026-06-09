# Getting Started

Como adotar o **AI Dev Workflow** em um projeto novo ou existente, em 5 minutos.

> **Pré-requisitos por sistema operacional**
> - **Linux / macOS**: bash já está disponível por padrão. Nenhuma instalação extra necessária.
> - **Windows**: os scripts são arquivos `.sh` (bash). Você precisa de **WSL** (recomendado) ou **Git Bash**.
>   - WSL: [instale pelo Microsoft Store](https://aka.ms/wslstore) e use `wsl bash script.sh`
>   - Git Bash: instale o [Git for Windows](https://gitforwindows.org/) e execute os comandos no terminal Git Bash.
> - **Comandos `git`**: funcionam igual em todos os sistemas operacionais.

## Projeto novo

### 1. Rode o bootstrap

**Linux / macOS**
```bash
cd /caminho/para/seu/projeto-vazio
bash /caminho/para/ai-dev-workflow/scripts/bootstrap.sh
```

**Windows — WSL (recomendado)**
```powershell
# No PowerShell, navegue até o projeto e rode via WSL:
cd C:\caminho\para\seu\projeto-vazio
wsl bash /mnt/c/caminho/para/ai-dev-workflow/scripts/bootstrap.sh
```

**Windows — Git Bash**
```bash
# Abra o Git Bash e execute:
cd /c/caminho/para/seu/projeto-vazio
bash /c/caminho/para/ai-dev-workflow/scripts/bootstrap.sh
```

O script vai:

- Copiar `.github/`, `AGENTS.md`, `CLAUDE.md`.
- Copiar `skills/`, incluindo a skill principal `atlas`.
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

Se seu agente suportar skills, prefira:

> "Use a skill atlas para fazer onboarding deste projeto e propor o próximo passo."

Se ele ler `PROGRESS.md` e propor algo coerente, está funcionando.

### 4. Commit inicial

```bash
# Funciona igual em Linux, macOS e Windows (PowerShell ou Git Bash):
git add -A
git commit -m "chore: bootstrap ai-dev-workflow"
```

---

## Projeto existente

> Diferente do projeto novo: aqui o objetivo é **mapear o que já existe** sem tocar em código. Refactor fica para depois.

### 1. Rode o `adopt.sh`

**Linux / macOS**
```bash
cd /caminho/para/seu/projeto-existente
bash /caminho/para/ai-dev-workflow/scripts/adopt.sh
```

**Windows — WSL (recomendado)**
```powershell
# No PowerShell, navegue até o projeto e rode via WSL:
cd C:\caminho\para\seu\projeto-existente
wsl bash /mnt/c/caminho/para/ai-dev-workflow/scripts/adopt.sh
```

**Windows — Git Bash**
```bash
# Abra o Git Bash e execute:
cd /c/caminho/para/seu/projeto-existente
bash /c/caminho/para/ai-dev-workflow/scripts/adopt.sh
```

O script:

- Detecta a stack (Next.js, Node backend, Python, mobile) a partir dos manifests.
- Copia `.github/` sem sobrescrever nada (`cp -n`).
- Copia `skills/` sem sobrescrever skills existentes, incluindo `atlas`.
- Cria só o esqueleto mínimo de `docs/` — preserva qualquer doc já existente.
- Não toca `README.md`, `.gitignore`, `.env.example` existentes.

> Use `--dry-run` para ver o que seria feito antes; `--minimal` para uma adoção ainda mais enxuta.

### 2. Rode o prompt de adoção com o agente

No editor com Copilot/Claude/Codex, peça:

> "Use a skill atlas para adotar este projeto existente."

Ele vai:

1. **Descobrir** stack, estrutura, padrões de commit (read-only).
2. **Validar** o que encontrou com você antes de escrever.
3. **Popular** `PROGRESS.md`, `tech-stack.md`, `overview.md`, ADR-0001 retroativa, `risk-register.md`.
4. **Resumir** lacunas (sem testes? sem CI? deps com CVE?) como sugestão de próximos.

### 3. Commit

```bash
# Funciona igual em Linux, macOS e Windows (PowerShell ou Git Bash):
git add -A
git commit -m "chore: adopt ai-dev-workflow"
```

### 4. A partir daqui

Use o fluxo completo nas **próximas features**. Não tente retrofittar tudo de uma vez.

---

## Sanidade do setup

Faça este teste — peça ao agente:

> "Use a skill atlas para criar a feature US-001: <título>."

Espere ver:

1. Ele lê `PROGRESS.md`.
2. Confirma DoR (ou pergunta o que falta).
3. Apresenta plano com TODO.
4. Executa em commits pequenos.
5. Roda quality gates.
6. Atualiza `PROGRESS.md` + cria feature doc.
7. Entrega resumo no formato padrão.

Se faltar algum passo, **revise as instruções** — provavelmente o `applyTo` está errado ou `AGENTS.md` não está sendo lido.
