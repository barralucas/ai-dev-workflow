# Data Model - AI Dev Workflow

## Entidades De Configuracao
| Entidade | Origem | Campos principais |
| --- | --- | --- |
| `Config` | `.aidw.toml` | `project`, `commands`, `paths`, `current_task` |
| `ProjectConfig` | `.aidw.toml` | `name`, `stack`, `language` |
| `CommandsConfig` | `.aidw.toml` | `lint`, `typecheck`, `test`, `build` |
| `PathsConfig` | `.aidw.toml` | `progress`, `adr_dir`, `features_dir` |
| `TaskConfig` | `.aidw.toml` | `id`, `title`, `phase`, `started_at` |

## Entidades De Progresso
| Entidade | Origem | Campos principais |
| --- | --- | --- |
| `Progress` | `docs/progress/PROGRESS.md` | `project_name`, `done`, `in_progress`, `next`, `tech_debt`, `decisions` |
| `ProgressItem` | Markdown | `text`, `sub_items` |

## Entidades De Workflow
| Entidade | Origem | Campos principais |
| --- | --- | --- |
| `Phase` | Codigo | `Context`, `Design`, `Plan`, `Execute`, `Verify`, `Document`, `Handoff` |
| `WorkflowState` | Runtime/config futuro | `task_id`, `task_title`, `current_phase`, `started_at`, `phase_history` |
| `PhaseTransition` | Runtime/config futuro | `from`, `to`, `timestamp` |

## Artefatos SDD
| Artefato | Local | Papel |
| --- | --- | --- |
| Constituicao | `docs/governance/constitution.md` | Principios nao-negociaveis do projeto |
| Spec | `specs/<id>/spec.md` | O que e por que construir |
| Plano | `specs/<id>/plan.md` | Como implementar respeitando stack e arquitetura |
| Tasks | `specs/<id>/tasks.md` | Trabalho atomico executavel e verificavel |
| Contratos | `specs/<id>/contracts/` | Interfaces, comandos e outputs esperados |
| Quickstart | `specs/<id>/quickstart.md` | Como validar a mudanca localmente |
