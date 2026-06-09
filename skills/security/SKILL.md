---
name: security
description: Use ao implementar autenticação, autorização, entradas externas, dados sensíveis ou qualquer feature com implicações de segurança. Cobre OWASP Top 10, validação de entrada, segredos, threat modeling STRIDE, CSRF, CORS e LGPD/GDPR.
---

# Security

> Princípio: **defesa em profundidade**. Toda fronteira valida; toda saída é segura por padrão.

## 1. OWASP Top 10 — Checklist Prático

- **A01 Broken Access Control**: toda rota/handler verifica autz; teste explícito de "usuário X não acessa recurso Y".
- **A02 Cryptographic Failures**: senhas com bcrypt/argon2; TLS sempre; nunca rolar seu próprio crypto.
- **A03 Injection**: queries parametrizadas (sempre); zero string concat em SQL/NoSQL/shell.
- **A04 Insecure Design**: threat modeling lite **antes** de implementar.
- **A05 Security Misconfiguration**: headers (CSP, HSTS, X-Frame-Options); modo prod desliga debug.
- **A06 Vulnerable Components**: `npm/pnpm audit`, `pip-audit`, `cargo audit` em CI.
- **A07 Identification & Auth Failures**: rate limit em login; MFA quando possível; tokens curtos + refresh.
- **A08 Data Integrity Failures**: assinatura de pacotes; SRI para CDN; checksums de releases.
- **A09 Logging & Monitoring Failures**: logs estruturados; alertas em eventos sensíveis.
- **A10 SSRF**: valide URLs de entrada; allowlist de domínios; bloqueie ranges privados em fetch server-side.

## 2. Validação de Entrada (regra de ouro)

- **Toda** entrada externa passa por schema: body, query, params, headers, env, mensagens de fila, arquivos.
- Use schema único como fonte da verdade dos tipos (`z.infer`, `pydantic.BaseModel`).
- Reject por padrão (`strict: true`); aceite só o que conhece.

## 3. Threat Modeling Lite (STRIDE)

Para cada nova entrada externa ou mudança em fluxo de auth/dados sensíveis:

| Categoria              | Pergunta                                        | Mitigação típica                        |
| ---------------------- | ----------------------------------------------- | --------------------------------------- |
| **S**poofing           | Posso me passar por outro usuário?              | Auth forte; tokens não-adivinháveis     |
| **T**ampering          | Posso alterar dados em trânsito/repouso?        | TLS; integridade (HMAC); RLS            |
| **R**epudiation        | Posso negar uma ação que fiz?                   | Audit log com timestamp + ator          |
| **I**nformation disc.  | Vazio dados que não deveria?                    | Autz por campo; redact em logs          |
| **D**enial of service  | Posso derrubar com requests?                    | Rate limit; payload size cap            |
| **E**levation of priv. | Posso virar admin?                              | Verificação dupla de papel              |

Documente as decisões em ADR ou na feature doc.

## 4. Segredos & Configuração

- **Nunca** comite `.env*` reais. Use `.env.example` com chaves vazias.
- Valide env na inicialização com schema; falhe fast se faltar.
- Em produção: secret manager (AWS SM, GCP SM, Doppler, Vault).
- **Rotacione** segredos periodicamente; tenha plano de revogação.

## 5. Auth & Sessão

- Senhas: hash com **argon2id** ou bcrypt (cost ≥ 12).
- Sessões: cookies `httpOnly`, `secure`, `sameSite=lax|strict`.
- JWT: curto (`<= 15min`); refresh token rotativo; `aud`/`iss` validados.
- **Lockout/backoff** após N tentativas falhas.
- **Logs nunca contêm**: senhas, tokens, PII desnecessária.

## 6. Output Safety

- HTML dinâmico: escape por padrão; `dangerouslySetInnerHTML` (ou equiv.) **apenas** com sanitizer (DOMPurify).
- SQL: prepared statements / ORM com binding — nunca string concat.
- Shell: nunca passe input do usuário direto; use APIs com argumentos separados (não `shell: true`).
- File uploads: validar MIME real (não só extensão), tamanho, e isolar storage.

## 7. CSRF & CORS

- CSRF: tokens em formulários POST tradicionais; `sameSite` em cookies; verificação de origem.
- CORS: allowlist explícita de origens; **nunca** `*` com credenciais.

## 8. Dependency Hygiene

- Lockfile sempre commitado.
- `npm/pnpm audit`, `pip-audit`, `cargo audit` em CI; PRs falham em vulnerabilidade `high`+.
- Renovate/Dependabot habilitado.
- Antes de adicionar lib: verifique mantenedores, downloads, último release, issues, CVEs.

## 9. Dados Pessoais (LGPD/GDPR)

- **Minimização**: colete só o necessário.
- **Propósito**: documente para que serve cada campo.
- **Retenção**: defina prazo e procedimento de exclusão.
- **Direitos do titular**: endpoint/processo para exportação e exclusão.
- **PII em logs**: nunca. Use IDs em vez de e-mails/CPFs.

## 10. Resposta a Incidente

1. Pare/contenha (rotacione segredo, desligue feature).
2. Avalie escopo e dados afetados.
3. Comunique stakeholders.
4. Crie postmortem em `docs/postmortem/YYYY-MM-DD-titulo.md`.
5. Adicione regressão em testes; atualize `risk-register.md`.

## Anti-padrões

- ❌ Logar request body cru (pode ter senha/token).
- ❌ Confiar em validação só no client.
- ❌ `eval`/`Function()` com input externo.
- ❌ Stack trace ao cliente em produção.
- ❌ Usar `Math.random()` para token/segredo (use crypto-random).
- ❌ Comparação de string para tokens (use timing-safe).
- ❌ CORS com `*` + credenciais.
- ❌ Senhas em hash MD5/SHA1.
