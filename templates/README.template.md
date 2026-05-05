# {{PROJECT_NAME}}

> <Descrição em 1-2 frases do que é o projeto.>

## Pré-requisitos

- <Runtime + versão> (ex.: Node 22+ / Python 3.12+)
- <Gerenciador de pacotes> (ex.: pnpm 10+)
- <Outros> (Docker, etc.)

## Setup rápido

```bash
git clone <repo>
cd <repo>
cp .env.example .env.local      # preencher segredos
<comando de install>            # ex.: pnpm install
<comando de dev>                # ex.: pnpm dev
```

## Comandos

```bash
<dev>          # desenvolvimento
<build>        # build de produção
<lint>         # lint
<typecheck>    # checagem de tipos
<test>         # testes
<format>       # formatação
```

## Variáveis de ambiente

Veja `.env.example`. Validação em `<src/env.ts | app/settings.py>`.

## Estrutura

```
<src ou app>/   # código
docs/           # documentação (PROGRESS, ADRs, features, arquitetura)
.github/        # instruções para agentes de IA + workflows CI
```

## Documentação

- [Estado atual do projeto](docs/progress/PROGRESS.md)
- [Arquitetura — Visão Geral](docs/architecture/overview.md)
- [Stack](docs/architecture/tech-stack.md)
- [ADRs](docs/adr/)
- [Backlog](docs/user-stories/backlog.md)

## Para agentes de IA

Este projeto adota o **AI Dev Workflow**. Antes de codar, leia [`AGENTS.md`](AGENTS.md) e siga as instruções em `.github/instructions/`.

## Licença

<MIT | proprietário | ...>
