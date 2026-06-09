---
name: testing
description: Use ao escrever testes ou definir estratégia de testes para qualquer stack. Cobre pirâmide de testes, padrão AAA, cobertura mínima, mocks, testes de regressão, testes de UI, integração e E2E.
---

# Testing

> Regra mínima por story: **caminho feliz + 1 caso de erro/borda**. Bug corrigido = teste de regressão obrigatório.
> Para ferramentas, agentes, templates e CLIs, trate harness engineering como parte do teste: comportamento esperado precisa ser reprodutivel localmente.

## 1. Pirâmide de Testes

```
        /\        E2E (poucos — fluxos críticos)
       /  \
      /────\      Integração (camadas reais; DB de teste)
     /      \
    /────────\    Unit (muitos; rápidos; sem IO real)
```

- **Unit**: funções puras, schemas, regras de negócio. Zero IO. ms.
- **Integração**: handler + DB + repositório real (test DB). Subseg.
- **E2E**: navegador/cliente real percorrendo fluxo completo. Caro — só fluxos críticos.

## 2. Estrutura AAA

```
// Arrange
const input = ...;

// Act
const result = fn(input);

// Assert
expect(result).toEqual(...);
```

- **Um conceito por teste**. Múltiplos asserts ok se descrevem o mesmo conceito.
- Nome do teste descreve **comportamento**: `should reject signup when email already exists`.

## 3. Cobertura

- **Não persiga 100%**. Persiga **caminhos críticos** cobertos.
- Mínimo sugerido: **80% em camada de domínio**, **>0% em UI/handlers**.
- Cobertura é **piso**, não teto. Teste de fluxo > linhas cobertas.

## 4. Mocks & Doubles

- **Prefira testes reais** com fakes em memória (DB SQLite em memória, in-memory bus).
- **Mock só nas fronteiras** verdadeiramente externas (rede, SMS, pagamento).
- **Não mock o sistema sob teste** — se precisa, o design está pedindo abstração.

## 5. Fixtures & Factories

- Builders/factories (`makeUser({ ... })`) > literais espalhados.
- Fixtures determinísticas (datas fixas, seeds, IDs previsíveis).
- Reset entre testes — sem ordem implícita.

## 6. Testes de Regressão

- **Bug corrigido = teste que falharia antes do fix**. Sem exceção.
- Linke o ID da issue/story no nome do teste: `it('US-042: paginação respeita limite máximo', ...)`.

## 7. Testes de Erro/Borda (obrigatórios)

Para cada caso de uso, teste:
- Input inválido (schema rejeita).
- Recurso não encontrado.
- Permissão negada.
- Limite/quota excedida.
- Concorrência (mesma operação em paralelo, se aplicável).
- Timeout/falha de dependência externa.

## 8. Testes de UI

- **Testing Library** (ou equivalente) — teste como o usuário usa.
- Não teste detalhes de implementação (state interno, classNames específicos).
- Acessibilidade: query por role/label preferencialmente (`getByRole('button', { name: /salvar/i })`).
- Snapshots: use **com moderação** e revise (não aprove diff cego).

## 9. Testes de API/Handler

- Suba a aplicação real (ou app mínimo que monte só o handler).
- Teste contratos: status code, body, headers, side effects (DB).
- Teste autz: sem permissão → 403; sem auth → 401.

## 10. E2E

- Playwright/Cypress para web; Maestro/Detox para mobile.
- Apenas **fluxos críticos** ($ que sangra se quebrar): login, checkout, ação principal.
- Roda em CI noturno e/ou pre-deploy — **não** em todo commit.

## 11. Performance & Load (quando aplicável)

- k6/Artillery para endpoints críticos.
- Defina SLO antes (ex.: p95 < 300ms a 100 RPS).
- Rode antes de releases grandes; documente em `docs/architecture/`.

## 12. CI/CD

- Todo PR roda: `lint`, `typecheck`, `test` (unit + integração).
- E2E em pipeline separado (mais lento).
- Coverage report em PR.
- Falha = bloqueia merge.

## 13. Harness Engineering

Use harness quando o sistema sob teste e uma ferramenta, workflow, template, prompt ou integracao agentic.

Tipos recomendados:
- **CLI harness**: execute comandos contra diretorios temporarios; valide arquivos criados, exit status e mensagens essenciais.
- **Template harness**: valide que templates obrigatorios existem, placeholders foram substituidos e a arvore gerada e esperada.
- **Workflow harness**: valide invariantes de estado, ordem de fases, transicoes e gates.
- **Contract harness**: derive testes de `specs/<id>/contracts/` para garantir que outputs publicos nao mudem sem intencao.
- **Agent/eval harness**: comece com checks deterministicos; use LLM real apenas como eval opt-in quando houver dados, custo e tolerancia a flakiness definidos.

Regras:
- Harness nao deve tocar o workspace real; use temp dirs/fixtures.
- Fixtures devem ser pequenas, deterministicas e nomeadas pelo comportamento.
- Golden/snapshot so deve ser usado para output estavel e revisavel.
- Um harness deve falhar antes de uma regressao chegar no usuario.

## Anti-padrões

- ❌ Teste que não falha quando o código quebra.
- ❌ Teste que depende da ordem de execução.
- ❌ Mock do sistema sob teste.
- ❌ "Vou adicionar teste depois".
- ❌ Snapshot enorme aprovado sem revisão.
- ❌ Suprimir teste flaky em vez de investigar.
- ❌ Bug corrigido sem teste de regressão.
- ❌ Testar CLI/template diretamente no repo real quando um temp dir resolver.
- ❌ Eval com LLM real como unico gate de comportamento deterministico.
