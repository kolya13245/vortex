# Vortex — План разработки

## Контекст

Vortex — кроссплатформенный VPN-клиент (Windows + Linux) с поддержкой двух ядер: **Mihomo (Clash Meta)** и **Xray-core**. Построен на Tauri v2 (Rust) + React (TypeScript) + Tailwind CSS. Проект на стадии scaffolding — исходного кода ещё нет, нужно строить с нуля.

**Ключевые требования пользователя:**
1. Поддержка Xray JSON конфигов
2. Множественные подписки, включая Xray JSON массив
3. Выбор ядра через хедеры
4. HWID по документации Remnawave (persistent после переустановки)
5. Linux + Windows
6. Поддержка XHTTP транспорта
7. Прокси-группы Mihomo (недоступны при Xray) + переключение Global/Rule
8. Минималистичный, простой UI

---

## Фазы реализации

### Фаза 1: Инициализация проекта
> Цель: рабочее Tauri-приложение с пустым окном

**Файлы:** `package.json`, `tsconfig.json`, `vite.config.ts`, `tailwind.config.ts`, `postcss.config.js`, `index.html`, `src/main.tsx`, `src/app.css`, `.eslintrc.cjs`, `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json`, `src-tauri/build.rs`, `src-tauri/src/main.rs`, `src-tauri/src/lib.rs`, `src-tauri/capabilities/default.json`

**Зависимости:**
- Frontend: react 19, @tauri-apps/api v2, @tauri-apps/plugin-shell, tailwindcss 4, vite, jotai, @tanstack/react-query, @tanstack/react-router, lucide-react, recharts
- Backend: tauri v2, serde, serde_json, serde_yaml, tokio, reqwest (rustls), machineid-rs, log, env_logger, dirs, tempfile, thiserror

**Проверка:** `pnpm install && pnpm tauri dev` → пустое окно

---

### Фаза 2: Backend (Rust)
> Цель: все Tauri-команды работают, ядра запускаются/останавливаются

Детали: [phase-2-backend.md](./phase-2-backend.md)

**Структура модулей:**
```
src-tauri/src/
  main.rs, lib.rs, error.rs, state.rs, commands.rs, tray.rs
  core/       → mod.rs, mihomo.rs, xray.rs, process.rs
  config/     → mod.rs, mihomo_gen.rs, xray_gen.rs, models.rs
  subscription/ → mod.rs, parser.rs, hwid.rs, headers.rs
  system/     → mod.rs, tun.rs, proxy.rs
```

**Ключевые решения:**
- **Strategy pattern** для генерации конфигов (Mihomo YAML / Xray JSON)
- **machineid-rs** для HWID: SystemID + CPUID + CPUCores → SHA256 (переживает переустановку)
- **Tauri events** для real-time данных (не прямой polling из фронтенда)
- **Atomic writes** для конфигов (temp file → rename)
- **Arc<Mutex<>>** для shared state через Tauri `manage()`

---

### Фаза 3: Frontend Shell
> Цель: Layout, навигация, роутинг, тема — все страницы как пустые заглушки

Детали: [phase-3-frontend-shell.md](./phase-3-frontend-shell.md)

**Структура:**
```
src/
  main.tsx, app.css, router.tsx
  layouts/    → app-layout.tsx, sidebar.tsx
  pages/      → dashboard.tsx, proxies.tsx, profiles.tsx,
                connections.tsx, logs.tsx, settings.tsx
  components/ → ui/ (button, card, badge, input, select, switch, modal, spinner)
  hooks/      → use-tauri.ts, use-core.ts, use-subscriptions.ts,
                use-mihomo.ts, use-settings.ts, use-theme.ts, use-traffic.ts
  stores/     → atoms.ts (jotai: coreType, connectionStatus, theme)
  types/      → core.ts, proxy.ts, subscription.ts, connection.ts, settings.ts
  lib/        → tauri.ts (type-safe invoke wrappers), constants.ts, utils.ts
```

---

### Фаза 4: Страницы приложения
> Цель: все страницы функциональны

Детали: [phase-4-pages.md](./phase-4-pages.md)

**Порядок реализации:**
1. **Dashboard** — статус подключения (orb), connect/disconnect, трафик, текущий прокси
2. **Settings** — выбор ядра, TUN, системный прокси, DNS, тема, HWID
3. **Profiles** — CRUD подписок, карточки с трафиком/сроком, парсинг форматов
4. **Proxies** — прокси-группы Mihomo (скрыто при Xray), Global/Rule переключение
5. **Connections** — таблица активных соединений (только Mihomo)
6. **Logs** — real-time логи, фильтр по уровню

---

### Фаза 5: Интеграция и полировка
> Цель: всё работает end-to-end на обеих платформах

- Real-time pipeline: трафик, логи, память через Tauri events
- Сохранение настроек в `{app_data}/settings.json`, подписок в `{app_data}/subscriptions.json`
- Sidecar бинарники через `externalBin` в tauri.conf.json
- Скрипт `scripts/download-cores.sh` для CI
- Single instance (lock file), minimize to tray, remember window size
- Обработка ошибок: core not found, subscription fetch fail, core crash, port conflict, TUN privileges

---

## Дизайн-система

Детали: [design-system.md](./design-system.md)

**Основные решения:**
- **Палитра:** Dark-first, near-black `#0a0a0f` фон, teal-cyan `#00d4aa` акцент
- **Шрифты:** Inter (UI) + JetBrains Mono (данные/логи)
- **Sidebar:** 64px collapsed (иконки) / 224px expanded, с tooltip'ами
- **Статус:** Connection Orb (80px) с пульсирующим glow — зелёный/жёлтый/серый/красный
- **Анимации:** Минимальные — pulse для статуса, 150ms transitions, fade-in для страниц
- **Тема:** CSS custom properties → Tailwind tokens, переключение через `data-theme`

---

## Поддержка подписок

| Формат | Описание |
|--------|----------|
| Xray JSON Array | JSON массив outbound-конфигов |
| Base64 | URI-ссылки (vmess://, vless://, trojan://, ss://) |
| Clash YAML | YAML с секцией `proxies` |
| SIP008 | JSON с секцией `servers` (Shadowsocks) |

**HWID Headers (Remnawave):**
```
x-hwid: <sha256 hardware id>      # обязательный
x-device-os: Windows|Linux
x-ver-os: <os version>
x-device-model: <hostname>
user-agent: Vortex/<version>
```

**Response headers парсинг:**
- `subscription-userinfo: upload=X; download=Y; total=Z; expire=T`
- `profile-update-interval`

---

## Протоколы и транспорты

**Протоколы:** VMess, VLESS (XTLS Vision + REALITY), Trojan, Shadowsocks (включая 2022), Hysteria2, TUIC, WireGuard

**Транспорты:** Raw/TCP, WebSocket, gRPC, **XHTTP**, HTTP/2, mKCP, HTTPUpgrade

---

## Прокси-группы (только Mihomo)

- Типы: Selector (ручной), URLTest (авто по скорости), Fallback, LoadBalance
- Управление через REST API: GET /proxies, PUT /proxies/{name}, GET /proxies/{name}/delay
- Global/Rule toggle: PATCH /configs с `{"mode": "global"|"rule"}`
- **При активном Xray** — секция Proxies скрыта, показывается подсказка "Переключитесь на Mihomo"

---

## Верификация

1. `pnpm tauri dev` — приложение запускается
2. Добавить подписку → ноды парсятся и отображаются
3. Переключить ядро Mihomo ↔ Xray → core restart, UI адаптируется
4. Connect → core запускается, трафик идёт, статус = Connected
5. Прокси-группы (Mihomo) → выбор ноды, тест задержки работает
6. TUN mode → системный трафик через прокси
7. Проверить на Linux и Windows
8. `pnpm lint && cargo clippy && cargo test` — без ошибок
