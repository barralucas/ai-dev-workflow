# Decisions Log - AI Dev Workflow

Micro-decisoes que nao exigem ADR propria.

## 2026-05-29

- `specs/` sera usado para artefatos SDD por mudanca relevante, separado de `docs/features/`, que documenta comportamento entregue.
- Harnesses iniciais ficam em testes Rust convencionais para manter execucao local simples e sem dependencias externas.
- Evals agentic com LLM real ficam fora do escopo inicial para evitar custo, flakiness e dependencia de credenciais.
