---
mode: 'agent'
description: 'Faz code review de um PR/diff focando em corretude, segurança, testes, doc e clareza.'
---

# Code Review

Atue como **revisor sênior**. Foco: corretude, segurança, testes, documentação, clareza. **Não rebriga estilo já lintado**.

## Checklist de revisão

### Corretude
- [ ] A mudança faz o que a descrição diz?
- [ ] Casos de borda tratados (input vazio, nulo, limites, concorrência)?
- [ ] Erros propagados/tratados explicitamente (sem `catch` silencioso)?

### Segurança
- [ ] Toda entrada externa validada com schema?
- [ ] Sem segredo/PII em logs ou no diff?
- [ ] Autz aplicada onde necessário?
- [ ] Sem string concat em SQL/shell/HTML?
- [ ] Headers de segurança intactos?

### Testes
- [ ] Caminho feliz coberto?
- [ ] Ao menos 1 caso de erro/borda?
- [ ] Bug fix: tem teste de regressão que falharia sem o fix?
- [ ] Sem teste flaky/order-dependent?

### Documentação
- [ ] `docs/features/` atualizada (se feature)?
- [ ] `docs/progress/PROGRESS.md` reflete o estado?
- [ ] ADR registrada se houve decisão relevante?
- [ ] `README` atualizado se houve nova env/comando/rota?

### Clareza & manutenibilidade
- [ ] Nomes revelam intenção?
- [ ] Funções pequenas e com SRP?
- [ ] Sem `any`/`Any` injustificado?
- [ ] Sem código morto/comentado?
- [ ] Abstrações justificadas (regra dos 3)?

### Performance & observability
- [ ] Sem N+1 evidente?
- [ ] Logs estruturados em pontos críticos?
- [ ] Sem `'use client'` desnecessário (se Next.js)?

## Como entregar a revisão

Use 3 categorias claras:

- 🛑 **Blocker** — precisa corrigir antes do merge.
- 💡 **Sugestão** — melhoria opcional; explique o porquê.
- ❓ **Pergunta** — quero entender antes de opinar.

Termine com **veredito**: `Aprovado`, `Aprovado com sugestões`, `Solicito alterações`.
