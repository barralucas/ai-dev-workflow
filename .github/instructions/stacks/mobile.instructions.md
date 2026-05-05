---
description: 'Adendo de stack — React Native / Expo + TypeScript estrito.'
applyTo: '**'
---

# Stack — Mobile (React Native / Expo)

> Adendo ao [`workflow.instructions.md`](../workflow.instructions.md). Use para apps mobile cross-platform.

## 1. Stack canônica

| Camada       | Escolha                                                | Versão |
| ------------ | ------------------------------------------------------ | ------ |
| Framework    | Expo (managed) — preferido para a maioria dos casos    | recent |
| UI           | React Native                                           | recent |
| Linguagem    | TypeScript estrito                                     | 5.x    |
| Navegação    | expo-router (file-based) **ou** React Navigation       | recent |
| Estado       | Zustand / Jotai / Redux Toolkit (pick one)             | —      |
| Validação    | Zod                                                    | 4.x    |
| HTTP         | TanStack Query + fetch                                 | —      |
| Testes       | Jest + React Native Testing Library + Maestro/Detox    | —      |
| Gerenciador  | pnpm (com `node-linker=hoisted` se necessário)         | 10+    |

## 2. Comandos

```bash
pnpm start           # expo start
pnpm ios             # build & run iOS sim
pnpm android         # build & run Android sim
pnpm lint            # ESLint
pnpm typecheck       # tsc --noEmit
pnpm test            # Jest
eas build --platform ios     # build de prod (iOS)
eas build --platform android # build de prod (Android)
```

**Pipeline VERIFY**: `pnpm lint && pnpm typecheck && pnpm test`.

## 3. Estrutura de pastas

```
app/                                  # rotas (expo-router file-based)
├── (tabs)/
├── _layout.tsx
└── index.tsx
src/
├── components/
│   ├── ui/
│   └── features/
├── features/<feature>/
│   ├── components/
│   ├── hooks/
│   ├── api/                         # chamadas + queries
│   └── schemas.ts
├── lib/                             # storage, http client, theme
├── hooks/
└── types/
assets/                              # imagens, fontes
```

## 4. Padrões fundamentais

- **expo-router** (file-based) por padrão.
- **Componentes funcionais + hooks**; sem classes.
- **TanStack Query** para estado de servidor; estado local em hooks ou Zustand.
- **TypeScript estrito**.
- **i18n**: `i18n-js` ou `lingui` se houver múltiplos idiomas.
- **Storage**: `expo-secure-store` para tokens; `AsyncStorage` para preferências.

## 5. Validação na fronteira

- Toda resposta de API valida com **zod** antes de usar (não confie em tipos do backend).
- Forms: `react-hook-form` + `zodResolver`.

## 6. Acessibilidade

- `accessibilityLabel`, `accessibilityRole`, `accessibilityHint` em interativos.
- Tamanhos de toque mínimos 44x44pt (iOS) / 48x48dp (Android).
- Contraste AA; respeitar `prefers-reduced-motion` (via `AccessibilityInfo`).
- Suporte a Dynamic Type / fontScale.

## 7. Performance

- Use `FlatList`/`SectionList` (com `keyExtractor`, `getItemLayout` quando possível) — nunca `map` em listas grandes.
- `useMemo`/`useCallback` apenas quando perfilou e mediu.
- Imagens: `expo-image` (com cache); dimensione antes de mandar pro device.
- `react-native-reanimated` para animações (UI thread).

## 8. Build & release

- **EAS Build** + **EAS Submit** para stores.
- Versionamento: `app.json` (`version`, `ios.buildNumber`, `android.versionCode`).
- OTA updates: `expo-updates` para correções rápidas (respeitando regras das stores).
- Crash reporting: Sentry (`@sentry/react-native`).
- Analytics: PostHog/Amplitude (com consent).

## 9. Testes

- **Unit**: hooks, helpers, schemas.
- **Componente**: React Native Testing Library — query por `accessibilityLabel`/role.
- **E2E**: Maestro (preferido — declarativo) ou Detox.
- Snapshots com moderação.

## 10. Segurança específica

- **Nunca** comite chaves de API embutidas (apps são engenharia-reversáveis).
- Tokens em **Secure Store** (Keychain/Keystore), nunca em `AsyncStorage`.
- Certificate pinning para endpoints sensíveis (`expo-network` + lib específica).
- Deep links: valide payload antes de navegar (XSS-like via params).

## 11. Anti-padrões específicos

- ❌ `console.log` em produção (use logger condicional).
- ❌ `setState` em loop sem dep correta → re-render infinito.
- ❌ Lista grande sem virtualização (`FlatList`).
- ❌ Imagens enormes sem redimensionar.
- ❌ Token em `AsyncStorage`.
- ❌ Bloquear thread JS com cálculos pesados — use Reanimated/worker.
- ❌ Dependências nativas sem checar suporte ao Expo managed (precisará dev build).

## 12. Bootstrap (uma vez)

```bash
pnpm create expo-app . --template default
pnpm add zod @tanstack/react-query
pnpm add -D @testing-library/react-native jest-expo
```

Configure `eas.json` quando for fazer primeiro build de produção.
