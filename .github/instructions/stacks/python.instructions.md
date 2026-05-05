---
description: 'Adendo de stack — Python 3.12+ (FastAPI/Django) + ruff + pytest + uv/poetry.'
applyTo: '**'
---

# Stack — Python

> Adendo ao [`workflow.instructions.md`](../workflow.instructions.md). Use para serviços/APIs em Python.

## 1. Stack canônica

| Camada       | Escolha                                       | Versão |
| ------------ | --------------------------------------------- | ------ |
| Runtime      | Python                                        | 3.12+  |
| Framework    | FastAPI (default) **ou** Django               | recent |
| Validação    | Pydantic v2                                   | 2.x    |
| ORM          | SQLAlchemy 2 **ou** Django ORM                | 2.x    |
| Migrations   | Alembic **ou** Django migrations              | —      |
| Logs         | structlog (JSON em prod)                      | —      |
| Lint/Format  | ruff (lint + format)                          | —      |
| Type check   | mypy (strict) **ou** pyright                  | —      |
| Testes       | pytest + pytest-asyncio + httpx               | —      |
| Gerenciador  | uv (preferido) **ou** poetry                  | recent |

## 2. Comandos

```bash
uv sync                  # instalar deps
uv run uvicorn app.main:app --reload   # dev (FastAPI)
uv run ruff check .      # lint
uv run ruff format .     # format
uv run mypy app          # typecheck
uv run pytest            # testes
uv run alembic upgrade head            # migrations
```

**Pipeline VERIFY**: `ruff check . && ruff format --check . && mypy app && pytest`.

> Não há "build" tradicional; em produção use container/wheel.

## 3. Estrutura de pastas

```
app/
├── main.py                          # entrypoint (FastAPI app)
├── settings.py                      # config (pydantic-settings)
├── domains/                         # 1 pasta por domínio
│   └── <domain>/
│       ├── router.py                # endpoints
│       ├── service.py               # casos de uso
│       ├── repository.py            # acesso a dados
│       ├── schemas.py               # pydantic
│       ├── models.py                # SQLAlchemy
│       └── tests/
├── lib/                             # cross-cutting (db, logger, errors)
├── middleware/                      # auth, request-id, error-handler
└── alembic/                         # migrations
tests/
pyproject.toml
```

## 4. Validação na fronteira

- **Toda** entrada externa vira `BaseModel` (pydantic).
- Configure `model_config = ConfigDict(extra='forbid')` para rejeitar campos desconhecidos.
- Settings via `pydantic-settings` carregando `.env`; falha fast no boot.

## 5. Padrão de erro

- Hierarquia: `DomainError` → `NotFoundError`, `ConflictError`, etc.
- Exception handler global converte para JSON (RFC 7807).
- **Nunca** retorne traceback em produção. Configure `DEBUG=false`.
- Logs estruturados com `structlog` + correlation ID.

## 6. Async vs sync

- FastAPI: prefira `async def` para handlers IO-bound; `def` quando tudo é CPU-bound síncrono (executa em threadpool).
- Não misture `requests` (sync) em `async def` — use `httpx.AsyncClient`.
- DB: SQLAlchemy 2 com driver async (`asyncpg`).

## 7. Testes

- **pytest** com fixtures.
- **httpx.AsyncClient** para testes de FastAPI (`ASGITransport`).
- DB de teste: testcontainers (Postgres) ou SQLite em memória.
- `pytest-cov` para cobertura.
- Marca `@pytest.mark.asyncio` (ou `asyncio_mode = "auto"` em `pyproject.toml`).

## 8. Tipagem

- `mypy --strict` ou `pyright` em modo `strict`.
- `from __future__ import annotations` para evitar forward refs custosas.
- Sem `Any` sem justificativa; prefira `TypeVar`, `Protocol`, `Literal`.

## 9. Performance & observability

- Pool de conexões DB (SQLAlchemy `pool_size`, `max_overflow`).
- Evite N+1: use `selectinload`/`joinedload`.
- `gunicorn` com `uvicorn.workers.UvicornWorker` em prod (ou `uvicorn` direto com supervisão).
- Métricas: `prometheus-client`.
- Tracing: OpenTelemetry (`opentelemetry-instrumentation-fastapi`).

## 10. Anti-padrões específicos

- ❌ `except: pass` ou `except Exception: pass`.
- ❌ Mutable default args (`def f(x=[])`).
- ❌ `print` em vez de logger.
- ❌ Acessar `os.environ` direto — use settings.
- ❌ String formatting em SQL (`f"SELECT ... {x}"`) — use bind params / ORM.
- ❌ Misturar sync IO em handler async.
- ❌ Esquecer de `await` (mypy/pyright pega isto).

## 11. Bootstrap (uma vez)

```bash
uv init --package
uv add fastapi pydantic-settings sqlalchemy alembic structlog httpx
uv add --dev ruff mypy pytest pytest-asyncio pytest-cov httpx
```

Configure `app/settings.py` com `pydantic-settings` antes da primeira feature.
