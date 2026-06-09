---
name: git-workflow
description: Use ao criar branches, escrever commits, abrir PRs ou revisar código. Cobre padrão Conventional Commits, nomenclatura de branches, tamanho de PR, checklist de code review e anti-padrões de git.
---

# Git Workflow

## 1. Branches

- `main` (ou `master`): sempre deployável.
- `feat/us-XXX-titulo-curto`: nova feature — referencia story.
- `fix/issue-XXX-descricao`: correção de bug.
- `refactor/escopo-curto`: refatoração sem mudança de comportamento.
- `docs/escopo-curto`: apenas documentação.
- `chore/escopo-curto`: build, deps, configs.
- `spike/tema`: investigação time-boxed (descartável).

## 2. Conventional Commits

Formato:
```
<tipo>(<escopo>): <descrição imperativa em minúsculas>

[corpo opcional explicando o "porquê"]

[footer opcional: BREAKING CHANGE / Closes #XXX]
```

**Tipos**:
- `feat`: nova funcionalidade.
- `fix`: correção de bug.
- `docs`: só documentação.
- `style`: formatação, sem mudança de código.
- `refactor`: mudança que não altera comportamento.
- `perf`: melhoria de performance.
- `test`: adicionar/corrigir testes.
- `chore`: build, deps, config, scripts.
- `ci`: pipelines.
- `revert`: reverte commit anterior.
- `wip`: trabalho em andamento (apenas em branches; squash antes de merge).

**Escopo**: opcional, mas recomendado. Use `us-XXX` para referenciar story.

**Exemplos**:
```
feat(us-005): add joinMatch server action
fix(us-007): handle expired magic-link gracefully
docs(adr): record decision to use Drizzle over Prisma
refactor(matches): extract repository from action
test(players): cover auto-join when next=/m/[id]
chore(deps): bump zod to 4.0.2
```

## 3. Commits Pequenos

- 1 commit = 1 mudança lógica.
- Se você precisa de "e" ou "também" para descrever, são 2 commits.
- Refactor preparatório vai **antes** da feature, em commit separado.

## 4. Pull Requests

### Tamanho
- Alvo: < 400 linhas adicionadas líquidas.
- Acima de 800 linhas: divida ou justifique.

### Descrição mínima
```markdown
## O que muda
<bullets>

## Por quê
<contexto / link para story / ADR>

## Como testar
1. <passo>
2. <passo>

## Checklist
- [ ] Critérios de aceite atendidos
- [ ] Testes adicionados (unit + integração)
- [ ] Doc da feature atualizada
- [ ] PROGRESS.md atualizado
- [ ] ADR registrada (se aplicável)
- [ ] Sem segredos comitados
- [ ] Quality gates verdes
```

### Code Review (do revisor)
- Foco em: corretude, segurança, clareza, testes, doc.
- Não rebriga estilo já lintado.
- Sugira; não imponha. Dúvida → pergunta, não asserção.
- Aprovação requer: gates verdes + checklist completo + ao menos 1 olho humano (em times).

## 5. Histórico

- **Squash merge** quando a branch tem ruído (vários `wip`, fixups).
- **Merge commit** preserva histórico real (preferido para features grandes bem-commitadas).
- **Rebase** para atualizar branch local antes de PR — nunca em branch compartilhada.
- `git push --force-with-lease` em branches próprias; **nunca** `--force` em `main`.

## 6. Tags & Releases

- `vMAJOR.MINOR.PATCH` (semver).
- Tag a partir de `main` após merge.
- Release notes geradas a partir de Conventional Commits.

## 7. Hooks Sugeridos

Husky/Lefthook:
- `pre-commit`: lint-staged (lint + format dos arquivos staged).
- `commit-msg`: commitlint (valida Conventional Commits).
- `pre-push`: typecheck + test (rápidos).

## Anti-padrões

- ❌ `git push --force` em branch compartilhada.
- ❌ Commit "WIP" no `main`.
- ❌ PR sem descrição.
- ❌ Commits gigantes misturando feature + refactor + fix.
- ❌ Mensagem `update`, `fix bug`, `changes`.
- ❌ Comitar `node_modules/`, `.env*` reais, builds, IDE configs pessoais.
- ❌ Usar `--no-verify` para pular hooks sem justificativa.
