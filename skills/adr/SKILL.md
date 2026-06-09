---
name: adr
description: Use ao criar uma Architecture Decision Record (ADR). Cobre critérios para criar ADR vs decisions-log, numeração sequencial, preenchimento do template (Status, Contexto, Decisão, Alternativas, Consequências), commit e linkagem no PROGRESS.md e feature doc.
---

# ADR — Architecture Decision Record

> ADR captura **o porquê** de uma decisão, não apenas o quê. Decisões não documentadas voltam para assombrar.

---

## Quando criar um ADR (vs. decisions-log)

**ADR** quando a decisão:
- Afeta 2+ módulos ou camadas.
- Tem trade-off real com alternativas viáveis.
- Será questionada no futuro ("por que fizemos assim?").
- Mudar depois implica trabalho significativo em outros lugares.

**decisions-log.md** quando:
- Micro-decisão de implementação local (1-3 linhas).
- Alternativas não eram viáveis de fato.
- Impacto restrito a 1 arquivo/função.

Em caso de dúvida: **prefira o ADR**. Custo de criar é baixo; custo de não ter é alto.

---

## Passo 1 — Numerar

1. Listar ADRs existentes em `docs/adr/`.
2. Próximo número = maior existente + 1 (zero-padded 4 dígitos): `0001`, `0002`, etc.
3. Nome do arquivo: `docs/adr/NNNN-titulo-curto-em-kebab-case.md`

---

## Passo 2 — Preencher o template

Template em: `templates/docs/adr/0000-template.md`

```markdown
# ADR-NNNN: <Título direto e curto>

**Status**: Proposed
**Data**: YYYY-MM-DD
**Autores**: <nome / handle>

## Contexto
<!-- Qual o problema? Quais restrições existem?
     Por que precisamos decidir agora? O que acontece se não decidirmos? -->

## Decisão
<!-- O que foi decidido. Direto, sem prosa. Pode incluir tabela. -->

## Alternativas consideradas
<!-- ≥ 2 alternativas com prós e contras objetivos -->
- **A) <nome>** — <descrição curta>
  - ✅ <pro>
  - ❌ <contra>
- **B) <nome>**
  - ✅ <pro>
  - ❌ <contra>

## Consequências
**Positivas**: <bullet>
**Negativas / custos**: <bullet>
**Impacto futuro**: <bullet — o que esta decisão habilita/restringe>

## Referências
- <links, RFCs, issues, ADRs relacionadas>
```

---

## Passo 3 — Status do ciclo de vida

| Status | Quando usar |
|---|---|
| `Proposed` | Rascunho criado, aguardando validação |
| `Accepted` | Aprovado pelo usuário/time |
| `Superseded by ADR-XXXX` | Substituída por decisão posterior |
| `Deprecated` | Não se aplica mais, mas mantida por histórico |

Regra: **não mude de `Proposed` para `Accepted` sem confirmação explícita do usuário**.

---

## Passo 4 — Linkar e registrar

- [ ] Atualizar `docs/progress/PROGRESS.md` → seção "🧭 Decisões recentes".
- [ ] Linkar na feature doc (`docs/features/<feature>.md`) se aplicável.
- [ ] Linkar em ADRs relacionadas (seção "Referências").

---

## Passo 5 — Commit

```
docs(adr): ADR-NNNN - <titulo curto>
```

---

## Checklist de qualidade do ADR

- [ ] Status definido.
- [ ] Contexto explica o problema sem assumir a solução.
- [ ] Decisão é clara e direta (não "vamos considerar X").
- [ ] Pelo menos 2 alternativas com prós/contras objetivos.
- [ ] Consequências incluem custos reais, não apenas benefícios.
- [ ] Linkada no PROGRESS.md.

---

## Exemplo de ADR ruim vs. bom

**Ruim** — contexto vago, sem alternativas, sem consequências:
```
Decidimos usar Drizzle.
```

**Bom** — contexto claro, alternativas documentadas, consequências honestas:
```
## Contexto
Precisamos de ORM para Postgres em um projeto Next.js 15 com Server Actions.
A equipe tem familiaridade com SQL puro. O schema muda com frequência.

## Decisão
Usar Drizzle ORM v0.30+ com migrations geradas.

## Alternativas
- Prisma: ✅ excelente DX / ❌ proxy runtime (Edge overhead), schema.prisma separado
- Kysley: ✅ type-safe SQL / ❌ sem migrations automáticas, menos ecossistema
- SQL puro: ✅ máximo controle / ❌ sem type-safety, migrations manuais

## Consequências
✅ Type-safety, sem proxy runtime, migrations versionadas
❌ API menos conhecida pela equipe (2-3 dias de ramp-up)
Futuro: habilita uso em Edge Functions sem overhead do Prisma
```
