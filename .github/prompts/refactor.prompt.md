---
mode: 'agent'
description: 'Refatoração sem mudança de comportamento — testes existentes devem continuar verdes.'
---

# Refactor

> **Regra de ouro**: refactor **não altera comportamento observável**. Se altera, é feature/fix — use o prompt apropriado.

## Passo a passo

1. **Justifique** o refactor: que problema resolve? (legibilidade, duplicação ≥3, performance medida, preparação para feature futura).
2. **Garanta cobertura prévia**: a área a refatorar tem testes? Se não, **adicione testes de caracterização** primeiro.
3. **Refatore em passos pequenos**, commit por passo.
4. **Rode testes a cada passo** — devem permanecer verdes.
5. **Não misture** com nova feature ou correção de bug — branches separadas.
6. **Documente** decisão se for arquitetural (ADR ou `decisions-log.md`).

## Sinais de que NÃO é hora de refatorar

- Sem testes na área afetada.
- Para "deixar mais bonito" sem critério objetivo.
- Antes da segunda/terceira repetição (regra dos 3).
- Próximo a release crítico.

## Commit

```
refactor(<escopo>): <descrição>

Motivo: <legibilidade | dedup ≥3 | perf medida | prep para US-XXX>
Comportamento: inalterado (testes verdes pré e pós)
```
