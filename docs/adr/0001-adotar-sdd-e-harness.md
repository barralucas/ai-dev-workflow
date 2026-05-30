# ADR-0001: Adotar SDD e Harness no Proprio Repositorio

**Status**: Accepted
**Data**: 2026-05-29

## Contexto
O projeto fornece um workflow para agentes de IA, mas o proprio repositorio nao mantinha `docs/progress/PROGRESS.md`, ADRs reais, specs por feature ou harnesses sistematicos para validar CLI/templates. Isso criava risco de as instrucoes recomendadas nao serem praticadas pelo projeto que as distribui.

## Decisao
Adotar Spec-Driven Development no proprio repositorio usando `docs/` para contexto vivo e `specs/` para specs, planos, tasks, contratos e quickstarts por mudanca relevante. Adotar harness deterministico em Rust para validar CLI, templates e workflow antes de introduzir evals com LLM real.

## Alternativas Consideradas
| Alternativa | Pros | Contras |
| --- | --- | --- |
| Manter apenas o workflow atual em `docs/` e prompts | Menor mudanca inicial | Nao cria fonte de verdade executavel por feature; pouca rastreabilidade spec -> teste |
| Adotar Spec Kit integralmente com `.specify/` | Alinha diretamente com ferramenta externa conhecida | Maior acoplamento e mudanca operacional; duplica conceitos ja existentes no AI Dev Workflow |
| Adotar SDD leve com `specs/` proprio | Menor acoplamento; preserva identidade do projeto; facil migrar incrementalmente | Requer manter templates e convencoes proprias |

## Consequencias
### Positivas
- O repositorio passa a dogfoodear seu proprio fluxo.
- Mudancas relevantes ficam rastreaveis de intencao a teste.
- Templates e CLI ganham protecao contra regressao.

### Negativas
- Mais artefatos para manter por mudanca relevante.
- E necessario disciplinar atualizacao de `PROGRESS.md` e specs.

### Acompanhamento
- Expandir harness para CI multi-plataforma.
- Avaliar evals agentic opt-in apos estabilizar testes deterministicos.
