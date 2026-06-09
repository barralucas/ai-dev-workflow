---
name: refactor
description: Use ao refatorar código sem mudança de comportamento observável. Cobre justificativa obrigatória, cobertura de testes antes de refatorar, regra dos 3 usos, commits atômicos por passo, pipeline verde a cada etapa e quando NÃO refatorar.
---

# Refactor — Sem Mudança de Comportamento

> Refactor **não altera comportamento observável**. Se alterar → use o fluxo de feature ou bug-fix.

---

## Justificativa obrigatória

Antes de qualquer refactor, confirme que se enquadra em ao menos um destes motivos:

- [ ] **Legibilidade**: código difícil de entender por outro desenvolvedor.
- [ ] **Deduplicação**: mesmo trecho aparece pela **terceira vez** (regra dos 3).
- [ ] **Performance medida**: há benchmark que comprova o ganho.
- [ ] **Preparação para feature**: refactor que desbloqueia US-XXX (documente o porquê).
- [ ] **Débito técnico registrado**: item em `docs/risks/risk-register.md` ou backlog.

Se nenhum destes se aplica → não refatore agora.

---

## Quando NÃO refatorar

- Área sem cobertura de testes (primeiro adicione testes).
- Apenas cosmético (renomear variável sem impacto de legibilidade real).
- Antes da regra dos 3 usos.
- Perto de release crítico.
- Junto com feature ou bug fix (branches separadas).

---

## Passo 1 — Garantir cobertura

1. Verifique se há testes cobrindo o código a ser refatorado.
2. Se não houver → adicione **testes de caracterização** (descrevem o comportamento atual sem julgar se é correto):
   ```
   it('characterization: returns X when given Y', () => {
     expect(fn(Y)).toEqual(X); // este é o comportamento atual
   });
   ```
3. Os testes devem estar **verdes antes do primeiro passo de refactor**.

---

## Passo 2 — Refatorar em passos atômicos

Cada passo de refactor = um commit separado.

```
refactor(<scope>): extract UserRepository from UserService
refactor(<scope>): rename processData to calculateMonthlyInvoice
refactor(<scope>): replace duplicated validation with shared schema
```

Após **cada commit**:
```
lint && typecheck && test
```

Os testes devem permanecer **verdes** em cada etapa. Se ficarem vermelhos → reverter e reavaliar.

---

## Passo 3 — Verificação final

```
lint && typecheck && test && build
```

- [ ] Comportamento externo inalterado (outputs iguais para os mesmos inputs).
- [ ] Todos os testes passando.
- [ ] Sem `any` novo introduzido.
- [ ] Sem código morto deixado para trás.
- [ ] Complexidade ciclomática reduzida ou mantida (não piorou).

---

## Passo 4 — Documentar (se relevante)

- Se o refactor foi preparação para uma feature → registrar em `docs/progress/decisions-log.md`.
- Se envolveu decisão arquitetural (ex.: mudança de padrão de repositório, novo layer de abstração) → ADR.
- Atualizar `docs/progress/PROGRESS.md` se o refactor era item do backlog.

---

## Formato de commit

```
refactor(<scope>): <short imperative description>

Motivo: readability | dedup ≥3 | measured perf | prep US-XXX
Comportamento: inalterado (testes verdes antes e depois)
```

---

## Anti-padrões

- ❌ Misturar refactor com feature ou bug fix no mesmo commit.
- ❌ Refatorar sem testes cobrindo a área.
- ❌ "Deixar mais bonito" sem critério objetivo.
- ❌ Criar abstrações antes de 3 usos reais.
- ❌ Refatorar em uma tacada gigante (commit único com 800+ linhas).
- ❌ Testes vermelhos em qualquer etapa intermediária.
- ❌ Usar `--no-verify` para pular hooks durante refactor.
