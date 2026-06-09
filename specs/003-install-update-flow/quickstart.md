# Quickstart: Install Update Flow

## Scripts

Em um projeto consumidor, rerode:

```bash
bash /caminho/para/ai-dev-workflow/scripts/bootstrap.sh --update --name MeuProjeto --stack none
```

Ou, para projeto existente:

```bash
bash /caminho/para/ai-dev-workflow/scripts/adopt.sh --yes --stack none
```

Validar:

```bash
test -f .aidw-version
test -f skills/atlas/SKILL.md
```

## CLI

```bash
aidw init . --yes
aidw adopt . --yes
```

Os comandos devem imprimir a versao do AI Dev Workflow e atualizar `skills/` quando a versao instalada diferir.
