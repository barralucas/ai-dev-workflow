# Modelo de Dados — {{PROJECT_NAME}}

> Esquema lógico das entidades principais. Atualize a cada migration.

## Diagrama ER

```mermaid
erDiagram
    USER ||--o{ ORDER : places
    ORDER ||--|{ ORDER_ITEM : contains
    PRODUCT ||--o{ ORDER_ITEM : referenced_in

    USER {
        uuid id PK
        string email UK
        string name
        timestamptz created_at
    }
    ORDER {
        uuid id PK
        uuid user_id FK
        string status
        timestamptz created_at
    }
```

## Entidades

### `users`

| Campo       | Tipo        | Constraints              | Notas |
| ----------- | ----------- | ------------------------ | ----- |
| `id`        | uuid        | PK, default gen_random   |       |
| `email`     | text        | UK, NOT NULL             |       |
| `created_at`| timestamptz | NOT NULL, default now()  |       |

<!-- Repita por entidade. -->

## Convenções

- IDs: `uuid` (não auto-increment) para evitar enumeração.
- Timestamps: `timestamptz` (UTC), nunca `timestamp` sem TZ.
- Soft delete: campo `deleted_at` (nullable) — quando aplicável.
- Auditoria: `created_at`, `updated_at` em toda tabela.
- Naming: `snake_case` (alinhado a Postgres); FKs como `<entidade>_id`.

## Migrations

- Ferramenta: `<Drizzle | Prisma | Alembic | Django>`.
- Pasta: `<caminho>`.
- Política: nunca editar migration aplicada — crie nova.
