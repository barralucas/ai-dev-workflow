# Customizing for Your Stack

Como adaptar o workflow à sua stack específica.

## Princípio

O **núcleo** (`workflow.instructions.md`) é stack-agnóstico — descreve o **fluxo**.
A **stack** (`stacks/<x>.instructions.md`) traz comandos, pastas, validação, framework de teste.

## Cenários

### A) Sua stack já existe (Next.js, Node, Python, mobile)

1. Mantenha apenas o(s) arquivo(s) relevante(s) em `.github/instructions/stacks/`.
2. Ajuste `applyTo` no frontmatter se quiser que seja só uma camada (`'src/api/**'`).
3. Atualize comandos se diferirem (ex.: você usa `npm` em vez de `pnpm`).

### B) Mais de uma stack no mesmo repo (monorepo)

Mantenha múltiplos arquivos com `applyTo` específicos:

```yaml
# stacks/nextjs.instructions.md
applyTo: 'apps/web/**'

# stacks/python.instructions.md
applyTo: 'apps/api/**'
```

### C) Stack não coberta (Go, Rust, Java, .NET, etc.)

Crie um novo arquivo seguindo o template lógico:

```markdown
---
description: 'Adendo de stack — <nome>'
applyTo: '**'
---

# Stack — <nome>

> Adendo ao `workflow.instructions.md`.

## 1. Stack canônica
| Camada | Escolha | Versão |
| ...

## 2. Comandos
\`\`\`bash
<lint>
<typecheck>
<test>
<build>
\`\`\`
**Pipeline VERIFY**: ...

## 3. Estrutura de pastas
...

## 4. Validação na fronteira
...

## 5. Padrão de erro
...

## 6. Testes
...

## 7. Performance / observability essentials
...

## 8. Anti-padrões específicos
...

## 9. Bootstrap (uma vez)
...
```

Cubra **no mínimo** as 9 seções acima. Submeta um PR para este repositório se for uma stack popular.

## Dicas

- **Não duplique** princípios já no núcleo (clean code, segurança, testes) — só o que é específico da stack.
- **Comandos exatos** > descrições — agentes copiam e colam.
- **Uma stack ativa por padrão** — se houver muitos `.instructions.md` conflitantes, o agente fica confuso.
- **Versões** sempre em `tech-stack.md`; aqui descreva apenas "use a versão LTS atual".
