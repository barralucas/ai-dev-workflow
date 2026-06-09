---
name: postmortem
description: Use após incidente de produção, bug grave escapado, decisão técnica custosa ou retrospectiva de sprint. Guia o fluxo blameless com timeline factual, 5 Whys, ações acionáveis com dono/prazo, atualização de risk-register e commit do documento.
---

# Postmortem — Blameless

> Princípios: **Blameless** (foco em sistemas), **Factual** (timeline com fontes), **Acionável** (cada lição = tarefa com dono), **Compartilhável**.

---

## Quando criar

- Incidente de produção (qualquer SEV).
- Bug grave que escapou para usuários.
- Decisão técnica com custo alto não antecipado.
- Retrospectiva significativa de sprint.

---

## Passo 1 — Coletade dados (antes de analisar)

Colete fatos **antes** de formular hipóteses:
- Logs, alertas, dashboards do período.
- Mensagens de comunicação (Slack, chat).
- Deploy history (`git log`, CI/CD logs).
- Horários exatos (UTC).

Não filtre os dados pela hipótese. Deixe os fatos falarem primeiro.

---

## Passo 2 — Criar o documento

Copiar `templates/docs/postmortem/_template.md` para:
```
docs/postmortem/YYYY-MM-DD-titulo-curto.md
```

Preencher:

```markdown
# Postmortem: <Título do incidente>

**Data do evento**: YYYY-MM-DD
**Autor(es)**: <nomes>
**Status**: Em análise
**Severidade**: SEV1 | SEV2 | SEV3 | Retrospectiva

## TL;DR
<!-- 2-3 frases: o que aconteceu, impacto, principal ação -->

## Impacto
- Usuários afetados: <#>
- Duração: <início → fim, UTC>
- SLA quebrado?: sim/não
- Dados afetados: <descrição>

## Timeline (UTC)
| Hora  | Evento                              | Fonte        |
|-------|-------------------------------------|--------------|
| HH:MM | <evento factual>                    | <alert/log>  |
| HH:MM | <ação tomada>                       | <quem>       |

## Causa-raiz (5 Whys)
1. Por que <sintoma>? Porque <causa imediata>.
2. Por que <causa imediata>? Porque ...
5. **Causa-raiz**: <descrição>

## Detecção
- Como soubemos? <alerta / cliente / monitoramento manual>
- TTD (time to detect): <minutos>
- TTM (time to mitigate): <minutos>

## Resposta
**O que funcionou**: <bullet>
**O que não funcionou**: <bullet>

## O que correu bem
- <bullet>

## Ações (com dono e prazo)
| # | Ação                                    | Dono | Prazo      | Story/Issue |
|---|-----------------------------------------|------|------------|-------------|
| 1 | Adicionar teste de regressão para X     | @x   | YYYY-MM-DD | US-YYY      |
| 2 | Adicionar alerta de latência p99 > 500ms| @y   | YYYY-MM-DD | —           |
| 3 | Atualizar risk-register.md com R-XXX    | @z   | imediato   | —           |
```

---

## Passo 3 — Análise blameless

- Use linguagem de sistemas: "o processo falhou" > "fulano errou".
- Cada "ação" da análise deve ser um passo do sistema, não uma culpa.
- Identifique **condições latentes** (o que tornou possível o incidente) além da causa imediata.

---

## Passo 4 — Ações acionáveis

Cada lição deve gerar uma **tarefa concreta** com:
- Descrição clara do que fazer.
- Dono (quem vai executar).
- Prazo.
- Story/issue vinculada (ou criar nova).

Evite ações vagas como "melhorar comunicação" ou "ser mais cuidadoso".

---

## Passo 5 — Atualizar documentos

- [ ] Adicionar teste de regressão para a causa-raiz (se bug de código).
- [ ] Atualizar `docs/risks/risk-register.md` (adicionar ou fechar riscos).
- [ ] Adicionar ações ao `docs/user-stories/backlog.md` (priorizadas).
- [ ] Atualizar `docs/progress/PROGRESS.md`.
- [ ] Mudar status do postmortem para `Concluído`.

---

## Passo 6 — Commit

```
docs(postmortem): YYYY-MM-DD <titulo-curto>
```

---

## Anti-padrões

- ❌ Apontar culpados em vez de causas sistêmicas.
- ❌ Timeline sem fontes verificáveis.
- ❌ Ações vagas sem dono e prazo.
- ❌ Não adicionar teste de regressão para causa técnica.
- ❌ Postmortem que ninguém lê (compartilhe com o time).
- ❌ Listar só o que deu errado; ignore o que funcionou.
