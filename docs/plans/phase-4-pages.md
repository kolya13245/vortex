# Фаза 4: Страницы приложения

## Порядок реализации

1. Dashboard → 2. Settings → 3. Profiles → 4. Proxies → 5. Connections → 6. Logs

---

## 4.1 Dashboard

**Компоненты:** `connection-orb.tsx`, `connect-button.tsx`, `traffic-stats.tsx`

**Layout:**
```
[Centered content, py-8]
  [Connection Orb — 80x80, пульсирующий glow по статусу]
  [Status Label — "Connected" / "Disconnected" / "Connecting..."]
  [Active Proxy Name]
  [Connect/Disconnect Button]
  
  [Stats Grid — 2x2 карточки, max-w-md mx-auto]
    [Download Speed — arrow-down + font-mono text-xl]
    [Upload Speed — arrow-up + font-mono text-xl]
    [Session Traffic — total up + down]
    [Active Core — badge + switch кнопка]
```

**Connection Orb состояния:**

| Состояние | Цвет | Иконка | Анимация |
|-----------|------|--------|----------|
| Disconnected | серый | Power | нет |
| Connecting | жёлтый | Spinner | pulse 1s |
| Connected | teal `#00d4aa` | Shield-check | pulse 2s |
| Error | красный | Alert-triangle | shake → static |

**Connect Button:**
- Disconnected → "Connect" (accent)
- Connecting → "Connecting..." (disabled, spinner)
- Connected → "Disconnect" (ghost, hover → danger)
- Error → "Retry" (accent)

---

## 4.2 Settings

**Секции:**

**Core:**
- Выбор ядра: Radio group (Mihomo / Xray) с иконками
- TUN mode: Toggle + warning "Requires admin privileges"
- System proxy: Toggle
- Mixed port: Number input

**DNS:**
- DNS mode (Mihomo): Select (normal, fakeip, redir-host)
- Primary DNS: Textarea
- Fallback DNS: Textarea

**Appearance:**
- Theme: Segmented control (Dark / Light / System)
- Language: Select

**About:**
- HWID: mono text + copy button (read-only)
- Version, core versions
- Links

---

## 4.3 Profiles (Подписки)

**Компоненты:** `subscription-card.tsx`, `add-profile-dialog.tsx`, `traffic-bar.tsx`

**Subscription Card:**
```
[Card bg-surface-raised rounded-xl border p-4]
  [Header: Name + Core Badge | Actions: edit, refresh, delete]
  [URL — mono text-xs tertiary, truncate + copy]
  [Stats: "42 nodes" | "Updated 2h ago"]
  [Traffic Bar (если есть от Remnawave)]
    [Progress bar h-1.5 bg-border → fill bg-accent]
    ["12.4 GB / 100 GB" mono text-xs]
  [Expiry: "Expires: Apr 30, 2026"]
```

**Add Dialog:**
- Name (text), URL (text), Auto-update interval (select: never/1h/6h/12h/24h)
- Save → fetch → parse → regenerate config

**Парсинг подписки:**
1. Fetch URL с HWID headers
2. Detect format (JSON/YAML/Base64)
3. Parse → Vec<ServerNode>
4. Parse response headers → TrafficInfo
5. Сохранить в subscriptions.json

---

## 4.4 Proxies (только Mihomo)

**Компоненты:** `proxy-group-card.tsx`, `node-item.tsx`, `mode-toggle.tsx`

**При активном Xray:** показать сообщение "Proxy group management is available with Mihomo core. Switch to Mihomo in Settings." с кнопкой перехода.

**Layout:**
```
[Header: "Proxies" | Mode Toggle (Global / Rule)]
[Search Bar — full width]
[Proxy Groups — flex flex-col gap-3]
  [Group Card — collapsible]
    [Header: Group Name + Type Badge (Selector/URLTest/...) | Current + Test All + Chevron]
    [Node List — grid grid-cols-2 gap-2]
      [Node Item — name + latency badge, clickable для Selector]
      [Selected Node — border-accent bg-accent/5]
```

**Latency Badges:**
- < 100ms → green
- 100-300ms → yellow
- > 300ms → red
- Untested → grey
- Timeout → red "timeout"

**API calls:**
- GET /proxies → список групп
- PUT /proxies/{group} → выбор ноды
- GET /proxies/{name}/delay → тест задержки
- PATCH /configs {"mode": "..."} → Global/Rule switch

---

## 4.5 Connections (только Mihomo)

**При активном Xray:** аналогичное сообщение как на Proxies.

**Table columns:**
| Destination | Network | Rule | Proxy Chain | Upload | Download | Time | Close |

- Mono font для данных
- Close button появляется при hover
- Auto-refresh каждую 1 секунду
- Search + "Close All" button
- Pause/Resume toggle

**API:** GET /connections, DELETE /connections/{id}

---

## 4.6 Logs

**Layout:**
```
[Header: "Logs" | Level Filter (All/Info/Warn/Error) | Search | Auto-scroll toggle]
[Log Container — bg-surface rounded-xl, mono text-xs, overflow-y-auto, flex-1]
  [Log Entry]
    [Timestamp — text-tertiary w-20]
    [Level Badge — colored (info=accent, warn=amber, error=red)]
    [Message — text-primary]
```

**Источники:**
- Mihomo: WebSocket /logs → Tauri event `vortex://log-line`
- Xray: stderr ring buffer → polling via Tauri command

**Виртуальный скролл** для производительности при большом количестве строк.
