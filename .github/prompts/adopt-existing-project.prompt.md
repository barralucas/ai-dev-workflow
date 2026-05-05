---
mode: 'agent'
description: 'Adoção do AI Dev Workflow em um projeto JÁ EXISTENTE — descoberta, inventário, ADRs retroativas e PROGRESS.md inicial.'
---

# Adopt Existing Project

Você está adotando o **AI Dev Workflow** em um projeto que **já tem código**. Sua missão **não é codar feature nova** — é **mapear o que já existe** e popular o contexto para que as próximas sessões funcionem com o fluxo completo.

> Pré-requisito: o script `scripts/adopt.sh` já foi rodado (estrutura mínima `docs/` + `.github/` em vigor). Se não, rode antes.

---

## Fase 1 — Descoberta (read-only)

**Não escreva nada ainda.** Apenas leia e tome notas.

1. **Manifests da stack**: `package.json`, `pyproject.toml`, `Cargo.toml`, `go.mod`, `app.json`, `Gemfile`, etc. Identifique:
   - Linguagem(ns) e versão(ões) runtime.
   - Framework(s) principal(is).
   - Bibliotecas relevantes (validação, ORM, testes, lint).
   - Scripts de build/dev/test.
2. **Estrutura de pastas** (alto nível, ≤ 2 níveis): `src/`, `app/`, `tests/`, `apps/`, `packages/`, etc.
3. **Configs**: `tsconfig.json`, `eslint.config.*`, `.prettierrc`, `pyproject.toml [tool.*]`, `Dockerfile`, `compose.*`, `.env.example`.
4. **Histórico git** (últimos 30-50 commits): `git --no-pager log --oneline -50`.
   - Padrão de commits? (Conventional? Mensagens livres?)
   - Branches ativas?
5. **README/docs existentes**: leia tudo que houver — não duplique depois.
6. **Testes**: existe pasta? quantos arquivos? framework?

> Se algum dado for ambíguo (ex.: monorepo com múltiplas stacks), **pergunte ao usuário antes de prosseguir**.

---

## Fase 2 — Validação com o usuário

Apresente um resumo (≤ 20 linhas) cobrindo:

- Stack identificada (com versões).
- Domínios/features aparentes (inferidos da estrutura).
- O que parece pronto vs. em andamento.
- Lacunas óbvias (ex.: "não há testes", "sem CI", "sem lint").
- 2-5 decisões arquiteturais já tomadas que merecem ADR retroativa.

Pergunte:

> "Posso popular `PROGRESS.md`, `tech-stack.md` e criar 1-2 ADRs retroativas com base nisso? Quer ajustar algo antes?"

**Aguarde aceite.** Não escreva sem confirmação.

---

## Fase 3 — Popular o contexto (com aceite do usuário)

Faça **na ordem** e em commits pequenos:

### 3.1. `docs/architecture/tech-stack.md`

Preencha com versões reais extraídas dos manifests. Não invente.

### 3.2. `docs/architecture/overview.md`

Esboce C4 nível 1-2 (contexto + containers). Use Mermaid. Mantenha curto (cabe na tela).

### 3.3. `docs/adr/0001-stack-inicial.md`

ADR retroativa registrando a stack atual. Status `Accepted`. Em "Alternativas consideradas", se você não souber o que foi avaliado, escreva honestamente: _"Decisão tomada antes da adoção do workflow; alternativas não documentadas."_

### 3.4. ADRs adicionais (opcional, máx. 2-3)

Apenas para decisões com **trade-off real e visível** no código (ex.: "ORM X em vez de Y", "monorepo com pnpm workspaces", "auth via JWT vs session"). **Não invente trade-offs.**

### 3.5. `docs/progress/PROGRESS.md`

- **Release/sprint atual**: pergunte ao usuário (ou marque "n/a — adoção retroativa").
- **✅ Concluído**: liste features identificadas no código (1 linha cada). Não detalhe — basta "auth", "listagem de X", etc.
- **🚧 Em andamento**: o que estiver em branch não-mergeada ou marcado em código (TODO, FIXME).
- **🎯 Próximos**: deixe vazio ou preencha com o que o usuário indicar.
- **🚫 Bloqueios**: lacunas técnicas óbvias (sem testes, sem CI, dep desatualizada crítica).

### 3.6. `docs/risks/risk-register.md`

Adicione riscos visíveis: dívida técnica explícita, deps com CVE conhecida, pontos sem teste em fluxo crítico.

### 3.7. `docs/features/<feature>.md` (opcional)

Para **uma** feature crítica já existente, crie a feature doc retroativa como exemplo. Não tente cobrir todas — fica por conta das próximas sessões.

---

## Fase 4 — Verificação

- [ ] `PROGRESS.md` permite a outro dev entender o estado em < 10 min.
- [ ] `tech-stack.md` bate com os manifests.
- [ ] ADR-0001 está coerente com a stack real.
- [ ] Nenhum doc inventou informação não-verificável no código.
- [ ] Commits no padrão Conventional Commits, separados por área.

---

## Fase 5 — Handoff

Resumo final no formato:

```
✅ Workflow adotado em <projeto>

📦 Contexto criado:
- docs/progress/PROGRESS.md (estado retroativo)
- docs/architecture/tech-stack.md
- docs/architecture/overview.md
- docs/adr/0001-stack-inicial.md
- docs/risks/risk-register.md (N riscos catalogados)

⚠️ Lacunas identificadas (sugestão de próximos):
- <ex.: sem testes em módulo X>
- <ex.: dep Y com CVE>
- <ex.: sem CI configurado>

🎯 Sugestão de próxima sessão:
- Rodar /new-feature para a próxima story OU
- Criar ADR-0002 sobre <decisão pendente real>
```

---

## Anti-padrões nesta adoção

- ❌ Inventar histórico que não está no código.
- ❌ Criar 10 ADRs retroativas só pra "encher".
- ❌ Documentar features uma a uma agora — faça incrementalmente nas próximas sessões.
- ❌ Refatorar código durante a adoção. **Adoção é read + doc, não refactor.**
- ❌ Sobrescrever README ou docs existentes sem perguntar.
