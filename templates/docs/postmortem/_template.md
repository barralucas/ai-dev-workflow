# Postmortem: <Título do incidente / retro>

**Data do evento**: YYYY-MM-DD
**Autor(es) do postmortem**: <nomes>
**Status**: Em análise | Concluído
**Severidade**: SEV1 | SEV2 | SEV3 | Retrospectiva (sem incidente)

---

## TL;DR

<!-- 2-3 frases: o que aconteceu, impacto, ação principal. -->

## Impacto

- **Usuários afetados**: <#>
- **Duração**: <início → fim, em UTC>
- **SLA quebrado?**: sim/não
- **Perda financeira estimada**: <$ ou n/a>
- **Dados afetados**: <descrição>

## Timeline (UTC)

| Hora  | Evento                                      | Fonte             |
| ----- | ------------------------------------------- | ----------------- |
| 14:02 | Alerta `error_rate > 5%` disparou           | Datadog           |
| 14:05 | On-call reconheceu                          | PagerDuty         |
| 14:12 | Identificada query nova introduzida em #432 | git blame         |
| 14:18 | Rollback aplicado                           | CI deploy log     |
| 14:21 | Métricas voltaram ao normal                 | Datadog           |

## Causa-raiz (5 Whys)

1. Por que <sintoma>? Porque <causa imediata>.
2. Por que <causa imediata>? Porque <...>
3. ...
5. **Causa-raiz**: <descrição>

## Detecção

- Como soubemos? <alerta / cliente / monitoramento manual>
- Tempo até detectar (TTD): <minutos>
- Tempo até mitigar (TTM): <minutos>

## Resposta

**O que funcionou**

- <bullet>

**O que não funcionou**

- <bullet>

## O que correu bem

<!-- Sempre tem algo. Reforça boas práticas. -->

- <bullet>

## Ações (acionáveis com dono e prazo)

| #   | Ação                                                | Dono | Prazo      | Story/Issue |
| --- | --------------------------------------------------- | ---- | ---------- | ----------- |
| 1   | Adicionar teste de regressão para `<cenário>`       | @x   | YYYY-MM-DD | US-YYY      |
| 2   | Adicionar alerta de latência p99 > 500ms            | @y   | YYYY-MM-DD | —           |
| 3   | Atualizar `risk-register.md` com R-XXX              | @z   | imediato   | —           |

## Princípios do postmortem

- **Blameless**: foco em sistemas e processos.
- **Factual**: timeline com fontes verificáveis.
- **Acionável**: cada lição vira tarefa.
