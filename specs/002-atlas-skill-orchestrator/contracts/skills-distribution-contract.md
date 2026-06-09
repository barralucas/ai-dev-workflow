# Contract: Skills Distribution

## `atlas` Skill

Dado o repositorio AI Dev Workflow, quando as skills forem listadas, entao deve existir `skills/atlas/SKILL.md` com frontmatter `name: atlas`.

## Scripts

Dado um projeto alvo, quando `scripts/bootstrap.sh` executar, entao `skills/atlas/SKILL.md` e as demais skills versionadas devem existir no destino.

Dado um projeto alvo com uma skill customizada existente, quando `scripts/adopt.sh` executar, entao a skill existente deve ser preservada e skills ausentes devem ser copiadas.

## CLI

Dado um diretorio temporario vazio, quando `aidw init` executar, entao `skills/atlas/SKILL.md` deve existir no destino.

Dado um diretorio temporario de projeto existente, quando `aidw adopt` executar, entao `skills/atlas/SKILL.md` deve existir e arquivos de skills ja existentes nao devem ser sobrescritos.
