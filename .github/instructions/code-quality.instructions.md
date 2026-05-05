---
description: 'Padrões de qualidade de código aplicáveis a qualquer linguagem/stack — clean code, SOLID, naming, comentários, dependências.'
applyTo: '**'
---

# Code Quality

Estes padrões valem para **qualquer linguagem**. Adendos por stack ficam em `stacks/`.

## 1. Clean code & SOLID (essencial)

- **SRP**: cada módulo/função/classe tem **uma** responsabilidade clara.
- **Funções pequenas** (regra prática: cabe na tela; ≤ ~40 linhas; ≤ 4 parâmetros).
- **Nomes revelam intenção**: `calculateMonthlyInvoice` > `processData`. Evite abreviações obscuras.
- **Evite flags booleanas em parâmetros** — divida em duas funções.
- **Imutabilidade por padrão**: não mute argumentos, prefira retornar novo valor.
- **Fail fast**: valide pré-condições no início; lance erro específico.
- **Composition > inheritance**.
- **Pure functions** sempre que possível (sem side-effects, mesmo input → mesmo output).

## 2. Tipagem & contratos

- **Tipos estritos** (TS `strict`, mypy `strict`, etc.). Sem `any`/`Any` sem narrowing/justificativa.
- **Validação na fronteira**: toda entrada externa (HTTP, FS, env, fila, CLI) passa por schema. Tipos internos derivam do schema.
- **DTOs explícitos** entre camadas — não vaze modelos de dados para a UI.

## 3. Tratamento de erros

- **Nunca** `catch` silencioso. Logue e/ou re-lance com contexto.
- Diferencie **erro de domínio** (esperado, ex.: `EmailAlreadyTakenError`) de **falha técnica** (ex.: `TimeoutError`).
- Mensagens ao usuário: amigáveis, sem stack trace.
- Sempre tenha **logs estruturados** com correlação (request id, user id quando seguro).

## 4. Comentários

- Código auto-explicativo > comentário. Se precisa comentar **o quê**, renomeie. Comente **o porquê**.
- **TODO/FIXME** com referência: `// TODO(us-042): paginar quando >100`.
- **Não comente código morto** — apague (git lembra).

## 5. Dependências

Antes de adicionar uma lib, responda:

1. **Já tem nativo na linguagem/runtime?**
2. **Já tem no framework atual?**
3. **A lib é mantida?** (último release < 12 meses, issues respondidas).
4. **Tamanho/peso é aceitável?** (bundle size para front, footprint para back).
5. **Licença compatível?** (evite GPL em projetos comerciais sem revisão).

Documente a escolha em ADR ou `decisions-log.md`.

## 6. Reuso & abstração

- **Regra dos 3**: só extraia abstração após o **terceiro** uso real.
- **Helpers/utilitários** vão para `lib/` (ou equivalente) **só** quando usados por 2+ features.
- **Não crie classes/factories/strategies** preventivamente.

## 7. Nomeação (convenções gerais)

- **Booleanos**: `isXxx`, `hasXxx`, `canXxx`, `shouldXxx`.
- **Funções**: verbo no início (`fetchUser`, `parseConfig`).
- **Constantes globais**: `SCREAMING_SNAKE_CASE`.
- **Arquivos**: siga a convenção da linguagem/framework (ex.: `kebab-case.ts`, `PascalCase.tsx` para componente, `snake_case.py`).
- **Branches**: `feat/us-XXX-titulo-curto`, `fix/...`, etc.

## 8. Performance (princípios universais)

- **Mensure antes de otimizar** — sem benchmark, sem otimização.
- **Evite N+1** em qualquer acesso a dados externos.
- **Cache com chave clara e invalidação previsível** — cache sem invalidação é bug.
- **Streams/iteradores** para grandes volumes; não carregue tudo em memória.
- **Concorrência > paralelismo** quando IO-bound.

## 9. Acessibilidade (qualquer interface ao humano)

- Web/mobile: navegação por teclado, foco visível, labels, contraste AA.
- CLI: mensagens claras, `--help`, exit codes corretos, suporte a `--json` quando plausível.
- API: erros padronizados (RFC 7807 ou similar), versionamento explícito.

## 10. Lint, format & gates

- **Lint** + **format** rodam em pre-commit (sugestão: husky/lefthook + lint-staged).
- **Sem desabilitar regra** sem comentário justificando.
- CI roda: `lint`, `typecheck`, `test`, `build` em todo PR.
