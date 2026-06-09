---
name: bug-fix
description: Use ao corrigir um bug. Cobre reprodução, localização da causa-raiz (não do sintoma), escrita do teste de regressão ANTES do fix, pipeline de verificação, documentação e commit padronizado. Teste de regressão é obrigatório.
---

# Bug Fix — Fluxo de Correção

> Regra de ouro: **escreva o teste que falha primeiro, depois corrija o código**. Bug sem teste de regressão = bug que volta.

---

## Inputs esperados do usuário

- Descrição do bug + passos para reproduzir.
- Comportamento esperado vs. observado.
- Ambiente (dev/staging/prod) e versão, se relevante.

Se os passos de reprodução estiverem incompletos, **pergunte antes de investigar**.

---

## Passo 1 — Reproduzir

1. Reproduza localmente com os passos informados.
2. Confirme o comportamento incorreto (screenshot, log, output).
3. Se não conseguir reproduzir: peça mais contexto ao usuário.

---

## Passo 2 — Localizar a causa-raiz (não o sintoma)

- Use logs, debugger, `git log --oneline`, `git blame` para rastrear origem.
- Aplique **5 Whys** se o bug parecer sintoma de problema mais profundo:
  1. Por que <sintoma>? Porque <causa imediata>.
  2. Por que <causa imediata>? ...
  5. **Causa-raiz**: `<descrição>`
- Verifique se o bug é sistêmico (afeta outros fluxos) — se sim, adicionar ao `risk-register.md`.

---

## Passo 3 — Escrever o teste de regressão (ANTES do fix)

```
// Arrange — cenário que reproduz o bug
// Act — ação que deveria funcionar
// Assert — comportamento esperado (que vai FALHAR antes do fix)
```

- Nome do teste: `it('US-XXX: <descreve o bug corrigido>', ...)` ou referencia o issue.
- O teste **deve falhar** neste momento. Se não falhar, o teste não está testando o problema correto.

---

## Passo 4 — Corrigir o código

- Corrija a **causa-raiz**, não o sintoma.
- Mantenha o escopo mínimo — não refatore código não relacionado no mesmo commit.
- O teste de regressão **deve passar** após o fix.

---

## Passo 5 — Verificar (pipeline completo)

```
lint && typecheck && test && build
```

- [ ] Teste de regressão verde.
- [ ] Nenhum teste existente quebrado.
- [ ] Edge cases relacionados ainda funcionam.
- [ ] Sem regressões visuais (se UI).

Se qualquer item falhar → voltar para o Passo 4.

---

## Passo 6 — Documentar

- [ ] Atualizar `docs/progress/PROGRESS.md` (registrar o fix como concluído).
- [ ] Se bug sistêmico ou de prod → adicionar/atualizar `docs/risks/risk-register.md`.
- [ ] Se incidente de produção → criar postmortem em `docs/postmortem/YYYY-MM-DD-titulo.md`.
- [ ] Micro-decisão de design na correção → `docs/progress/decisions-log.md`.

---

## Passo 7 — Commit e entrega

Formato de commit:
```
fix(<scope>): <short description of what was fixed>

Root cause: <1-2 sentences>
Regression covered by: <test file:line or test name>
Closes #<issue> (se aplicável)
```

Exemplo:
```
fix(auth): prevent token reuse after logout

Root cause: refresh token was not invalidated on the server on logout.
Regression covered by: src/features/auth/auth.test.ts — "should reject reused refresh token"
Closes #87
```

---

## Anti-padrões

- ❌ Corrigir sem entender a causa-raiz (bug vai voltar).
- ❌ Adicionar `if` de workaround sem comentário explicando por quê.
- ❌ Commitar fix sem teste de regressão.
- ❌ Misturar refactor ou feature no mesmo commit do fix.
- ❌ Silenciar o erro em vez de corrigi-lo.
- ❌ "Funciona na minha máquina" sem reprodução verificável.
