---
name: spike
description: Use ao iniciar uma investigação time-boxed (spike) para reduzir incerteza técnica antes de comprometer com uma feature ou decisão. Cobre definição da pergunta, time-box, critérios de conclusão, documentação do resultado e recomendação (promover feature, ADR, novo spike ou descartar).
---

# Spike — Investigação Time-Boxed

> Spike é uma investigação **time-boxed** para reduzir incerteza. O resultado é **conhecimento**, não código de produção.

---

## Quando fazer um spike (vs. começar a feature diretamente)

Faça um spike quando:
- Não sabe se a abordagem técnica escolhida é viável.
- Há dúvida sobre performance, integração com sistema externo ou lib desconhecida.
- A estimativa da feature varia muito (depende de algo incerto).
- Existe decisão arquitetural bloqueada por falta de dados.

Não faça spike quando:
- A incerteza é sobre requisito de negócio (pergunte ao usuário/PO).
- Já há experiência comprovada da equipe com a abordagem.
- O time-box seria maior que a implementação da feature em si.

---

## Passo 1 — Definir a pergunta

A pergunta deve ser:
- **Específica**: "Conseguimos processar 10k eventos/s com BullMQ no hardware atual?" (não "BullMQ é bom?")
- **Resposta verificável**: você saberá quando a pergunta foi respondida.
- **Com critério de "concluído"**: "PoC com benchmark > 10k/s **ou** evidência de que não é possível"

---

## Passo 2 — Definir o time-box

Exemplos típicos:
- Investigação de lib/framework: **4h-1 dia**
- PoC de integração com sistema externo: **1-2 dias**
- Benchmark de performance: **meio dia**
- Investigação de arquitetura complexa: **2-3 dias**

Se o time-box se esgotar sem resposta → documente o que foi descoberto e decida: estender (com novo time-box definido) ou descartar.

---

## Passo 3 — Criar o documento

Copiar `templates/docs/spikes/_template.md` para:
```
docs/spikes/YYYY-MM-DD-titulo-curto.md
```

Preencher:
```markdown
# Spike: <Título>

**Time-box**: <ex.: 1 dia>
**Status**: Em andamento
**Autor**: <nome>
**Data**: YYYY-MM-DD

## Pergunta a responder
<!-- Específica, com critério para "respondida" -->

## Hipótese inicial
<!-- O que você acha que vai descobrir -->

## Critérios de "spike concluído"
- [ ] Pergunta respondida com evidência (PoC, benchmark, doc do fornecedor)
- [ ] Trade-offs documentados
- [ ] Recomendação clara

## Investigação
### Experimento 1: <nome>
- Setup: ...
- Resultado: ...

## Conclusão
<!-- Resposta à pergunta -->

## Recomendação
- [ ] Promover para feature (criar US-XXX)
- [ ] Registrar ADR-NNNN
- [ ] Novo spike sobre <aspecto restante>
- [ ] Descartar (motivo: ...)

## Artefatos
- <link para PoC, branch throwaway, doc>
```

---

## Passo 4 — Executar a investigação

- Código de PoC vai em branch throwaway (`spike/tema`) — **não mergear para main**.
- Documente cada experimento com setup e resultado (mesmo negativos).
- Se descobrir algo que invalida a hipótese → mude de direção rapidamente (é o ponto do time-box).

---

## Passo 5 — Concluir e recomendar

Ao fim do time-box, preencher Conclusão e Recomendação:

| Resultado | Próxima ação |
|---|---|
| Abordagem viável com trade-offs claros | Criar US-XXX + ADR-NNNN |
| Decisão arquitetural impactada | Criar ADR (status: Proposed) |
| Aspecto ainda incerto | Novo spike com time-box menor |
| Abordagem inviável | Descartar + documentar porquê |

---

## Passo 6 — Registrar e commitar

- [ ] Mudar status do doc para `Concluído`.
- [ ] Atualizar `docs/progress/PROGRESS.md` (decisões recentes ou próximos passos).
- [ ] Criar US/issue se a recomendação for promover para feature.
- [ ] Criar ADR se houver decisão arquitetural.

```
docs(spikes): YYYY-MM-DD <titulo-curto> — <recomendação resumida>
```

---

## Anti-padrões

- ❌ Spike sem pergunta específica ("investigar X" não é pergunta).
- ❌ Spike sem time-box (vira pesquisa infinita).
- ❌ Código de spike que vai para produção sem revisão.
- ❌ Mergar branch de spike para main.
- ❌ Não documentar experimentos negativos (muito valiosos).
- ❌ Spike que substitui a decisão do usuário/PO sobre requisito de negócio.
