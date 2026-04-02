# Фаза 3: Frontend Shell

## Структура файлов

```
src/
  main.tsx                — Root: QueryClientProvider + RouterProvider
  app.css                 — Tailwind directives, CSS variables, globals
  router.tsx              — TanStack Router route tree
  layouts/
    app-layout.tsx        — Sidebar + content area
    sidebar.tsx           — Навигация
  pages/
    dashboard.tsx         — (заглушка)
    proxies.tsx           — (заглушка)
    profiles.tsx          — (заглушка)
    connections.tsx       — (заглушка)
    logs.tsx              — (заглушка)
    settings.tsx          — (заглушка)
  components/
    ui/
      button.tsx          — Primary, secondary, danger, icon
      card.tsx            — Базовая карточка
      badge.tsx           — Статус, задержка, ядро
      input.tsx           — Текстовое поле
      select.tsx          — Кастомный dropdown
      switch.tsx          — Toggle switch
      modal.tsx           — Диалоговое окно
      spinner.tsx         — Loading indicator
  hooks/
    use-tauri.ts          — Обёртки invoke/listen
    use-core.ts           — Core status, start/stop
    use-subscriptions.ts  — CRUD подписок (TanStack Query)
    use-mihomo.ts         — Proxy groups, connections, traffic
    use-settings.ts       — Settings read/write
    use-theme.ts          — Dark/light toggle
    use-traffic.ts        — Real-time traffic (Tauri events)
    use-logs.ts           — Log streaming
  stores/
    atoms.ts              — Jotai: coreType, connectionStatus, theme, sidebarCollapsed
  types/
    core.ts               — CoreType, CoreStatus, ConnectionStatus
    proxy.ts              — ProxyGroup, ProxyNode, ProxyDelay
    subscription.ts       — Subscription, TrafficInfo, SubscriptionFormat
    connection.ts         — Connection, ConnectionSnapshot
    settings.ts           — UserSettings, DnsConfig, TunConfig
  lib/
    tauri.ts              — Type-safe invoke для каждой Tauri команды
    constants.ts          — Пути, дефолты, метаданные
    utils.ts              — Форматирование (bytes, speed, dates)
    cn.ts                 — clsx + tailwind-merge
```

---

## Router (router.tsx)

TanStack Router:
- `/` → redirect `/dashboard`
- `/dashboard` → DashboardPage
- `/proxies` → ProxiesPage
- `/profiles` → ProfilesPage
- `/connections` → ConnectionsPage
- `/logs` → LogsPage
- `/settings` → SettingsPage

Все маршруты внутри `AppLayout`.

---

## Sidebar (sidebar.tsx)

- **Collapsed:** 64px (иконки + tooltips)
- **Expanded:** 224px (иконки + текст)
- По умолчанию collapsed для минимализма
- Expand: hover или pin toggle
- Background: `bg-bg-sidebar`, right border
- Иконки: Lucide React — LayoutDashboard, Globe, FolderSync, Cable, ScrollText, Settings
- Active route: `bg-accent/10 text-accent`
- Bottom: core indicator badge, app version

---

## Sidebar Navigation Items

| Route | Icon | Label | Условие |
|-------|------|-------|---------|
| /dashboard | LayoutDashboard | Dashboard | — |
| /proxies | Globe | Proxies | Виден всегда, disabled при Xray |
| /profiles | FolderSync | Profiles | — |
| /connections | Cable | Connections | Виден всегда, disabled при Xray |
| /logs | ScrollText | Logs | — |
| /settings | Settings | Settings | — |

---

## Type-safe Tauri IPC (lib/tauri.ts)

```typescript
import { invoke } from '@tauri-apps/api/core';
import type { CoreType, CoreStatus } from '@/types/core';

export const startCore = (coreType: CoreType) =>
  invoke<void>('start_core', { coreType });
export const stopCore = () =>
  invoke<void>('stop_core');
export const getSubscriptions = () =>
  invoke<Subscription[]>('get_subscriptions');
export const getProxyGroups = () =>
  invoke<ProxyGroup[]>('get_proxy_groups');
// ... и т.д. для каждой команды
```

---

## TanStack Query Hooks

```typescript
// use-core.ts
export function useCore() {
  const status = useQuery({ queryKey: ['core-status'], queryFn: getCoreStatus, refetchInterval: 2000 });
  const start = useMutation({ mutationFn: startCore, onSuccess: () => queryClient.invalidateQueries(['core-status']) });
  const stop = useMutation({ mutationFn: stopCore, ... });
  const switchCore = useMutation({ mutationFn: switchCoreType, ... });
  return { status, start, stop, switchCore };
}

// use-mihomo.ts
export function useMihomoProxies() {
  return useQuery({ queryKey: ['proxy-groups'], queryFn: getProxyGroups, refetchInterval: 5000, enabled: coreType === 'mihomo' });
}
```

---

## Jotai Atoms (stores/atoms.ts)

```typescript
export const coreTypeAtom = atom<CoreType>('mihomo');
export const connectionStatusAtom = atom<ConnectionStatus>('disconnected');
export const themeAtom = atom<'dark' | 'light' | 'system'>('dark');
export const sidebarCollapsedAtom = atom<boolean>(true);
```

---

## Утилиты (lib/utils.ts)

```typescript
export function formatBytes(bytes: number): string;      // "1.2 GB"
export function formatSpeed(bytesPerSec: number): string; // "42.5 MB/s"
export function formatDuration(seconds: number): string;   // "2h 15m"
export function timeAgo(date: Date): string;               // "3 hours ago"
```

---

## Проверка

- Все маршруты работают, страницы-заглушки рендерятся
- Sidebar навигация подсвечивает активный маршрут
- Тема переключается (dark/light)
- `pnpm lint` проходит без ошибок
