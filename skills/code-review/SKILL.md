---
name: code-review
description: Use ao revisar código (PR review, pair review ou auto-review antes de merge). Checklist estruturado em 6 dimensões: Corretude, Segurança, Testes, Documentação, Clareza e Performance/Observabilidade. Entrega veredicto padronizado com blockers e sugestões.
---

# Code Review

> Aja como revisor sênior. Foque em corretude, segurança e clareza. Não rebrigue estilo já coberto pelo linter.

---

## Antes de começar

1. Leia a **descrição do PR** (o quê muda e por quê).
2. Leia a **story/issue** referenciada (critérios de aceite).
3. Se não houver descrição → pediu antes de revisar.

---

## Checklist de revisão (6 dimensões)

### 1. Corretude

- [ ] A mudança faz o que a descrição diz?
- [ ] Edge cases estão tratados?
- [ ] Erros são propagados corretamente (sem `catch` silencioso)?
- [ ] Condições de corrida possíveis foram consideradas?
- [ ] Idempotência está garantida onde necessário?

### 2. Segurança

- [ ] Toda entrada externa está sendo validada com schema?
- [ ] Nenhum segredo ou PII no diff (logs, comentários, variáveis hardcoded)?
- [ ] Autorização está aplicada em cada rota/handler novo?
- [ ] Zero concatenação de strings em SQL/queries?
- [ ] Headers de segurança intactos (CSP, CORS, etc.)?
- [ ] Uploads de arquivo validam MIME real e tamanho?

### 3. Testes

- [ ] Caminho feliz coberto?
- [ ] Ao menos 1 caso de erro/borda testado?
- [ ] Bug fix tem teste de regressão que falharia antes do fix?
- [ ] Nenhum teste flaky introduzido (datas sem mock, IDs aleatórios sem seed)?
- [ ] Mocks usados apenas em fronteiras externas reais?

### 4. Documentação

- [ ] `docs/features/<feature>.md` criada ou atualizada?
- [ ] `docs/progress/PROGRESS.md` reflete o estado atual?
- [ ] ADR registrada se houve decisão arquitetural?
- [ ] `README.md` atualizado para nova env var, comando ou rota pública?
- [ ] Comentários no código explicam o **porquê**, não o quê?

### 5. Clareza

- [ ] Nomes revelam intenção? (`calculateTotal` > `processData`)
- [ ] Funções com responsabilidade única (SRP)?
- [ ] Sem `any`/`unknown` não justificado?
- [ ] Sem código morto (console.log, TODO antigo, variáveis não usadas)?
- [ ] Abstrações justificadas (regra dos 3 usos)?
- [ ] Booleanos nomeados: `isLoading`, `hasPermission`, `canDelete`?

### 6. Performance & Observabilidade

- [ ] Sem N+1 evidente em queries/fetches?
- [ ] Logs estruturados nos pontos críticos (sem PII)?
- [ ] Sem `use client` desnecessário em Server Components (Next.js)?
- [ ] Payloads de resposta não incluem campos desnecessários?
- [ ] Cache com invalidação previsível (sem stale eterno)?

---

## Formato de entrega dos comentários

Use prefixos de severidade:

| Prefixo | Significado | Obriga mudança? |
|---|---|---|
| `🛑 Blocker` | Deve ser corrigido antes do merge | Sim |
| `💡 Sugestão` | Melhoria opcional; explique o porquê | Não |
| `❓ Dúvida` | Quer entender antes de opinar | Não |

Exemplo:
```
🛑 Blocker — `userId` vem do body sem validação. Mova para o token JWT (rota autenticada).

💡 Sugestão — `processData` poderia ser `calculateMonthlyRevenue` para revelar intenção.

❓ Dúvida — Este loop pode processar 10k items? Existe paginação planejada?
```

---

## Veredicto final

Encerre com um dos três:

```
✅ Aprovado
```

```
✅ Aprovado com sugestões
(lista de 💡 opcionais; pode mergear)
```

```
🔄 Solicitar mudanças
(lista de 🛑 blockers que devem ser resolvidos antes do merge)
```

---

## Anti-padrões do revisor

- ❌ Rebriger estilo que o linter já cobre (não opine sobre aspas, indentação).
- ❌ Aprovar sem ler os testes.
- ❌ Comentar "por que não X?" sem explicar o benefício real de X.
- ❌ Bloquear PR por preferência pessoal sem justificativa técnica.
- ❌ Aprovar sem verificar os quality gates verdes.
- ❌ Revisar apenas os novos arquivos; ignorar os modificados.
