# FAQ

### O agente continua "esquecendo" o contexto. O que fazer?

Verifique:

1. `AGENTS.md` está na raiz?
2. `docs/progress/PROGRESS.md` está atualizado e descritivo?
3. Você está usando o prompt `onboard-agent` no início da sessão?
4. `.github/copilot-instructions.md` ou equivalente da sua ferramenta está presente?

Se sim e ainda esquece, o problema pode ser **`PROGRESS.md` longo demais** — quebre por release ou compacte itens antigos.

### Posso usar com Cursor / Windsurf / outro?

Sim. Eles geralmente lêem `.github/copilot-instructions.md` ou `.cursorrules`. Gere um a partir das instruções existentes ou aponte o agente para `AGENTS.md`.

### Não quero Conventional Commits, posso trocar?

Sim. Edite `.github/instructions/git-workflow.instructions.md`. Lembre que mudará impactos em changelog automático (semantic-release etc.).

### Posso usar issues do GitHub em vez de `PROGRESS.md`?

Sim, com cuidado: o agente **não lê issues automaticamente**. Mantenha `PROGRESS.md` espelhando o estado, mesmo que a fonte primária sejam issues. Ou ensine o agente a buscar issues via MCP/tool.

### O fluxo é overengineering para projetos pequenos?

Use o subset mínimo:

- `PROGRESS.md` (essencial)
- `workflow.instructions.md` (essencial)
- `AGENTS.md` (essencial)
- ADRs (só quando há decisão real)
- O resto entra quando precisa

### Como mantenho o workflow atualizado em vários projetos?

Três opções:

1. **Submódulo Git**: `git submodule add` + symlinks. Permite `git submodule update --remote`.
2. **Cópia**: re-rode `bootstrap.sh --update`.
3. **Catálogo central**: hospede uma versão de referência e revise periodicamente.

### Como pluggar com MCP servers?

MCP é ortogonal. As instruções continuam valendo; o agente apenas ganha mais ferramentas. Se um MCP for crítico para o projeto, mencione em `AGENTS.md`.

### Diferença entre ADR e `decisions-log.md`?

- **ADR**: decisão com trade-off real, impacto em 2+ módulos, será questionada no futuro.
- **decisions-log**: micro-decisão (1-3 linhas), não afeta arquitetura amplamente.

Em dúvida, prefira ADR.

### O que fazer quando uma ADR ficar obsoleta?

Mude o status para `Superseded by ADR-XXXX` e crie a nova ADR referenciando a antiga. **Nunca delete** ADRs aceitas — elas são histórico.

### Posso ter múltiplas stacks ativas no mesmo repo?

Sim (monorepo). Use `applyTo` no frontmatter de cada `stacks/*.instructions.md` apontando para subpastas:

```yaml
applyTo: 'apps/web/**'
```

### O agente está pulando o VERIFY. Como forçar?

- Confirme que a instrução está sendo carregada (verifique `applyTo`).
- Adicione no prompt direto: "antes do handoff, rode `lint && typecheck && test && build` e mostre o output".
- Em casos extremos, configure pre-commit/pre-push hooks que falhem se algum gate quebrar.

### `Definition of Ready` parece burocrático. Vale a pena?

Sim, especialmente com agentes. Sem DoR claro, o agente "inventa" critérios e entrega algo diferente do esperado. 30 segundos de DoR economizam horas de retrabalho.

### Como medir se o workflow está funcionando?

Indicadores:

- Tempo de onboarding de novo dev/agente: < 15 min para entender o estado.
- % de PRs com testes: > 90%.
- % de incidentes com postmortem: 100%.
- Decisões questionadas sem ADR para responder: tendendo a zero.
- Quality gates verdes em PRs: > 95%.
