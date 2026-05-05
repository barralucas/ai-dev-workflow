---
mode: 'agent'
description: 'Inicia uma nova feature seguindo o fluxo de 7 fases do AI Dev Workflow.'
---

# Nova Feature

Você vai construir a feature descrita pelo usuário **respeitando rigorosamente o workflow de 7 fases** definido em [`.github/instructions/workflow.instructions.md`](../instructions/workflow.instructions.md).

## Passo a passo

1. **CONTEXT**
   - Leia [`docs/progress/PROGRESS.md`](../../docs/progress/PROGRESS.md).
   - Localize a story no [`docs/user-stories/backlog.md`](../../docs/user-stories/backlog.md). Se não existir, peça os critérios de aceite ao usuário.
   - Confirme **Definition of Ready**: objetivo claro, ator(es), AC, contratos esboçados, sem bloqueios.
   - Resuma em 1 frase para quem é e o que entrega.

2. **DESIGN** (se a feature tem trade-off real ou impacta 2+ módulos)
   - Esboce dados, fluxo e camadas afetadas.
   - Liste **≥ 2 alternativas** com prós/contras.
   - Faça threat modeling lite (STRIDE) das entradas externas.
   - Proponha registrar **ADR** (template em `templates/docs/adr/0000-template.md`).

3. **PLAN**
   - Apresente uma **TODO list** atômica com os arquivos exatos a criar/modificar.
   - Espere confirmação do usuário se a feature for grande.

4. **EXECUTE** (na ordem)
   - Schemas → camada de dados → camada de negócio → apresentação → estados de erro/borda → testes.
   - Commits pequenos no padrão `feat(us-XXX): ...`.

5. **VERIFY**
   - Rode os comandos de quality gate da stack ativa (ex.: `pnpm lint && pnpm typecheck && pnpm test && pnpm build`).
   - Cheque AC um por um, acessibilidade, responsividade, segurança, performance.

6. **DOCUMENT**
   - Crie/atualize `docs/features/<feature>.md`.
   - Atualize `docs/progress/PROGRESS.md` (concluído + próximos).
   - Registre ADR/decisões.

7. **HANDOFF**
   - Marque DoD.
   - Entregue resumo no formato padrão (Concluído / Entregue / Como testar / Docs / Próximo).

## Inputs esperados do usuário

- ID e título da story (ex.: `US-042 - Listar partidas`).
- Critérios de aceite (se não estão no backlog).
- Restrições/preferências específicas (libs, integrações).
