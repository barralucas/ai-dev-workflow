---
description: 'Estratégia de testes agnóstica de stack — pirâmide de testes, AAA, cobertura mínima, mocks, testes de regressão.'
applyTo: '**'
---

# Testing

> Regra mínima por story: **caminho feliz + 1 caso de erro/borda**. Bug corrigido = teste de regressão obrigatório.

## 1. Pirâmide

```
        /\        E2E (poucos, fluxos críticos)
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
- Mínimo sugerido: **80% em camada de domínio**, **>0% em UI/handlers** (com testes de fluxo principal).
- Cobertura é **piso**, não teto. Teste de fluxo > linhas cobertas.

## 4. Mocks & doubles

- **Prefira testes reais** com fakes em memória (DB SQLite em memória, in-memory bus).
- **Mock só nas fronteiras** verdadeiramente externas (rede, SMS, pagamento).
- **Não mock o sistema sob teste** — se precisa, redesign está pedindo abstração.

## 5. Test doubles & fixtures

- Builders/factories (`makeUser({ ... })`) > literais espalhados.
- Fixtures determinísticas (datas fixas, seeds, IDs previsíveis).
- Reset entre testes — sem ordem implícita.

## 6. Testes de regressão

- **Bug corrigido = teste que falharia antes do fix**. Sem exceção.
- Linke o ID da issue/story no nome do teste: `it('US-042: paginação respeita limite máximo', ...)`.

## 7. Testes de erro/borda (obrigatórios)

Para cada caso de uso, teste:

- Input inválido (schema rejeita).
- Recurso não encontrado.
- Permissão negada.
- Limite/quota excedida.
- Concorrência (mesma operação em paralelo, se aplicável).
- Timeout/falha de dependência externa.

## 8. Testes de UI (front)

- **Testing Library** (ou equivalente) — teste como o usuário usa.
- Não teste detalhes de implementação (state interno, classNames específicos).
- Acessibilidade: query por role/label preferencialmente (`getByRole('button', { name: /salvar/i })`).
- Snapshots: use **com moderação** e revisem (não aprove diff cego).

## 9. Testes de API/handler

- Suba a aplicação real (ou um app mínimo que monte só o handler).
- Teste contratos: status code, body, headers, side effects (DB).
- Teste autz: usuário sem permissão → 403; sem auth → 401.

## 10. E2E

- Playwright/Cypress para web; Maestro/Detox para mobile.
- Apenas **fluxos críticos** ($ que sangra se quebrar): login, checkout, ação principal.
- Roda em CI noturno e/ou pre-deploy — **não** em todo commit (caro).

## 11. Performance & load (quando aplicável)

- k6/Artillery para endpoints críticos.
- Defina SLO antes (ex.: p95 < 300ms a 100 RPS).
- Rode antes de releases grandes; documente em `docs/architecture/`.

## 12. Mutation testing (opcional, alto valor)

- Stryker (JS/TS) ou mutmut (Python) — mede se testes detectam mutações reais.
- Use em camadas de domínio críticas.

## 13. CI/CD

- Todo PR roda: `lint`, `typecheck`, `test` (unit + integração).
- E2E em pipeline separado (mais lento).
- Coverage report em PR (Codecov/Coveralls).
- Falha = bloqueia merge.

## 14. Anti-padrões

- ❌ Teste que não falha quando o código quebra.
- ❌ Teste que depende da ordem de execução.
- ❌ Mock do sistema sob teste.
- ❌ "Vou adicionar teste depois".
- ❌ Snapshot enorme aprovado sem revisão.
- ❌ Suprimir teste flaky em vez de investigar.
