# Risk Register - AI Dev Workflow

| ID | Risco | Impacto | Probabilidade | Mitigacao | Status |
| --- | --- | --- | --- | --- | --- |
| R-001 | Instrucoes apontarem para arquivos inexistentes no proprio repo | Alto | Media | Dogfooding com `docs/` real e `aidw doctor`/harness | Em mitigacao |
| R-002 | Mudancas em templates quebrarem bootstrap/adocao sem deteccao | Alto | Media | Golden/harness de templates e smoke tests de CLI | Em mitigacao |
| R-003 | Comandos documentados funcionarem em Linux mas falharem em Windows | Medio | Media | Harness cross-platform e evitar dependencia implicita de `sh` fora de `verify` configurado | Aberto |
| R-004 | Evals agentic flakies bloquearem desenvolvimento | Medio | Baixa | Comecar com harness deterministico; evals LLM ficam opt-in | Mitigado |
