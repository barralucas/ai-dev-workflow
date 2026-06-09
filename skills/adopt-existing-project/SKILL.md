---
name: adopt-existing-project
description: Use ao adotar o AI Dev Workflow em um projeto JÁ EXISTENTE. Cobre as 5 fases de adoção: Discovery (read-only), validação com usuário, população de contexto (tech-stack, overview, ADRs retroativos, PROGRESS.md), verificação e handoff. Nunca inventa histórico nem refatora durante adoção.
---

# Adopt Existing Project

> Missão: **mapear o que já existe** e popular o contexto para sessões futuras. **NÃO** implementar novas features durante a adoção.

---

## Pré-requisito

Execute o script antes de usar este fluxo:
```bash
./scripts/adopt.sh
```

Ele copia os arquivos de instruções e cria o esqueleto mínimo de `docs/` sem sobrescrever nada existente.

---

## Fase 1 — Discovery (somente leitura)

Ler e mapear:

1. **Manifests**: `package.json`, `pyproject.toml`, `Cargo.toml`, `build.gradle`, etc. — identifica stack e versões reais.
2. **Estrutura de pastas**: entender domínios, módulos, camadas.
3. **Configs**: `tsconfig`, `.eslintrc`, `docker-compose`, etc.
4. **Histórico git**: `git log --oneline -50` — identifica features entregues recentemente.
5. **README/docs existentes**: absorver convenções já adotadas.
6. **Testes existentes**: cobertura, frameworks, padrões.

Se houver ambiguidade (monorepo com múltiplos stacks, stack não identificada) → **pergunte antes de avançar**.

---

## Fase 2 — Validação com usuário

Apresentar resumo em ≤ 20 linhas:

```
Stack identificada: <linguagem + framework + versões reais dos manifests>
Domínios/features aparentes: <lista>
Estado aparente: <features prontas vs em andamento>
Gaps óbvios: <sem testes? sem docs? sem CI?>
Decisões retroativas sugeridas para ADR: <2-5 itens>
```

Perguntar explicitamente:
> "Posso popular `PROGRESS.md`, `tech-stack.md` e criar 1-2 ADRs retroativos baseado neste mapeamento?"

**Aguardar aceite antes de escrever qualquer arquivo.**

---

## Fase 3 — Popular contexto (nesta ordem, um commit por passo)

### 3.1 `docs/architecture/tech-stack.md`
- Versões **reais** dos manifests — não inventar.
- Stack, frameworks, libs principais, infra.

### 3.2 `docs/architecture/overview.md`
- Diagrama C4 nível 1 (contexto) e 2 (containers) em Mermaid.
- Baseado no que o código mostra, não em suposições.

### 3.3 `docs/adr/0001-stack-inicial.md`
- ADR retroativa. Status: `Accepted`.
- Contexto: por que esta stack (infira das evidências ou pergunte).
- Alternativas: o que provavelmente foi considerado (ou ask se incerto).

### 3.4 ADRs adicionais (máx 2-3)
- Apenas para trade-offs reais e visíveis no código.
- Ex.: "por que não ORM?" se código usa SQL puro, "por que sem autenticação?" se relevante.
- Não crie ADRs especulativas.

### 3.5 `docs/progress/PROGRESS.md`
- Estado retroativo: ✅ features claramente entregues, 🚧 em andamento, 🚫 bloqueios visíveis.
- Sprint/release: inferir do git history ou perguntar.

### 3.6 `docs/risks/risk-register.md`
- Riscos visíveis: débito técnico óbvio, CVEs em deps, fluxos críticos sem testes.
- Não exagere: apenas riscos reais identificados.

### 3.7 `docs/features/<feature>.md` (opcional)
- **Uma** feature crítica como exemplo retroativo.
- Objetivo: mostrar o padrão para features futuras.

---

## Fase 4 — Verificação

- [ ] `PROGRESS.md` legível em < 10 minutos.
- [ ] `tech-stack.md` bate com os manifests.
- [ ] ADR-0001 coerente com a stack real.
- [ ] Nenhuma informação inventada.
- [ ] Nenhum código alterado (somente docs criados).

---

## Fase 5 — Handoff

```
✅ Adoção concluída: <projeto>

📝 Documentos criados:
- docs/architecture/tech-stack.md
- docs/architecture/overview.md
- docs/adr/0001-stack-inicial.md
- docs/progress/PROGRESS.md
- docs/risks/risk-register.md

⚠️ Gaps identificados:
- <sem testes de integração>
- <deps desatualizadas: X, Y>

🎯 Próxima sessão sugerida:
- Confirmar backlog em docs/user-stories/backlog.md
- Começar com US-001: <primeira feature priorizada>
```

---

## Anti-padrões

- ❌ Inventar histórico, versões ou decisões.
- ❌ Criar 10 ADRs retroativas só para "preencher".
- ❌ Refatorar código durante a adoção.
- ❌ Alterar código existente — esta fase é read + docs apenas.
- ❌ Avançar para Fase 3 sem aceite explícito do usuário na Fase 2.
