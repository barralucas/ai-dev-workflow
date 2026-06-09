---
name: onboard-agent
description: Use ao iniciar uma nova sessão de agente em qualquer projeto. Guia a leitura ordenada de contexto (PROGRESS.md, architecture, ADRs, features, risks), verificação do estado git e resumo padronizado ao usuário antes de executar qualquer tarefa.
---

# Onboard Agent — Início de Sessão

> Nunca execute uma tarefa sem ler o contexto primeiro. O `PROGRESS.md` é o cérebro do projeto.

---

## Passo 1 — Ler contexto (nesta ordem)

1. **`AGENTS.md`** — regras do projeto e onde tudo fica.
2. **`docs/progress/PROGRESS.md`** — estado atual: sprint, WIP, bloqueios, decisões recentes.
3. **`docs/architecture/overview.md`** + **`docs/architecture/tech-stack.md`** — estrutura e stack.
4. **Últimas 3 ADRs relevantes** em `docs/adr/` (mais recentes pelo número).
5. **Última feature entregue** em `docs/features/` (entender padrão estabelecido).
6. **`docs/risks/risk-register.md`** — riscos ativos.

---

## Passo 2 — Verificar estado do repositório

```bash
git status                    # arquivos modificados não commitados
git log --oneline -5          # últimos 5 commits
git branch --show-current     # branch atual
```

- Há trabalho em andamento (WIP) não commitado?
- A branch atual bate com o que está em `PROGRESS.md` como "🚧 Em andamento"?
- Há conflitos ou merge pendente?

---

## Passo 3 — Resumir para o usuário

Entregar resumo em ≤ 15 linhas:

```
📋 Estado do projeto: <Sprint N | Release R1 | fase>

✅ Últimas entregas:
- US-XXX: <título> (concluída em <data>)

🚧 Em andamento:
- US-YYY: <título> — parei em: <ponto exato>

⏭️ Próximos sugeridos (por prioridade):
1. US-ZZZ: <título>
2. US-WWW: <título>

🔴 Bloqueios ativos:
- <descrição ou "nenhum">

🧭 Decisões recentes:
- <ADR-NNNN ou decisions-log entry>
```

---

## Passo 4 — Confirmar com o usuário

**Antes de executar qualquer tarefa**, perguntar:

> "Continuo em [US-YYY — título em andamento] ou prefere começar com [US-ZZZ — próximo sugerido]?"

Aguardar confirmação. **Não iniciar execução sem resposta**.

---

## Anti-padrões

- ❌ Pular a leitura do `PROGRESS.md`.
- ❌ Assumir prioridade sem confirmar com o usuário.
- ❌ Começar a codar baseado apenas nos arquivos abertos no editor.
- ❌ Ignorar WIP não commitado (pode sobrescrever trabalho).
- ❌ Resumir em mais de 15 linhas (poluição de contexto).
- ❌ Executar a "tarefa óbvia" sem confirmar que é a certa agora.
