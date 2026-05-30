# Constitution - AI Dev Workflow

## Principios
1. **Intento antes de codigo**: mudancas relevantes comecam em `specs/<id>/spec.md` antes de implementacao.
2. **Plano executavel**: cada spec relevante deve ter `plan.md` e `tasks.md` com arquivos e criterios claros.
3. **Contexto vivo**: `docs/progress/PROGRESS.md` e docs de arquitetura devem refletir o estado real.
4. **Harness antes de confianca**: comportamento de CLI, templates e workflow deve ser coberto por testes deterministas.
5. **Sem dependencias invisiveis**: comandos, shells, paths e pre-requisitos precisam estar documentados e testados quando possivel.
6. **Decisoes rastreaveis**: decisoes com trade-off real viram ADR; micro-decisoes ficam em `decisions-log.md`.
7. **Simplicidade operacional**: prefira harness local rapido antes de frameworks/evals complexos.

## Gates
- Nenhuma feature relevante deve ir para handoff sem spec, plan, tasks, docs atualizadas e `cargo test --workspace` verde.
- Mudanca em template deve ter teste que prove a arvore/placeholder esperado.
- Mudanca em comando CLI deve ter pelo menos um teste de caminho feliz e um caso de erro/borda, quando aplicavel.

## Governanca
- Esta constituicao e referenciada por specs e planos.
- Alteracoes nestes principios exigem ADR.
- Excecoes devem ser registradas em `docs/progress/decisions-log.md` com motivo e prazo de revisao.
