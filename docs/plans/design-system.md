# Дизайн-система Vortex — Glassmorphism Edition

> Визуальный стиль: **glassmorphism + minimalism**. Полупрозрачные frosted-glass поверхности
> поверх глубокого градиентного фона. Стеклянные панели используются только для контейнеров
> (карточки, сайдбар, модалки) — не для каждого элемента. Концепт-мокап: `src/assets/mockups/dashboard-concept.jpg`

---

## 1. Цветовая палитра

### 1.1 Background Scene (за стеклом)

Градиентный фон — это "сцена", которую стеклянные панели размывают. Без неё glassmorphism не работает.

| Токен | Значение | Применение |
|-------|----------|------------|
| `bg-base` | `#08080f` | Самый глубокий слой (body) |
| `bg-gradient-from` | `#0a0a1a` | Градиент: начало (тёмный navy) |
| `bg-gradient-via` | `#12102a` | Градиент: середина (deep purple) |
| `bg-gradient-to` | `#0a1628` | Градиент: конец (dark blue) |

Реализация фона:
```css
.app-background {
  background: #08080f;
  background-image:
    radial-gradient(ellipse 80% 60% at 20% 80%, rgba(0, 212, 170, 0.03) 0%, transparent 50%),
    radial-gradient(ellipse 60% 50% at 80% 20%, rgba(88, 40, 180, 0.06) 0%, transparent 50%),
    linear-gradient(135deg, #0a0a1a 0%, #12102a 40%, #0a1628 100%);
  min-height: 100vh;
}
```

### 1.2 Glass Surface Tokens

| Токен | Значение | Применение |
|-------|----------|------------|
| `glass-bg` | `rgba(255, 255, 255, 0.05)` | Фон стеклянных панелей (5% white) |
| `glass-bg-hover` | `rgba(255, 255, 255, 0.08)` | Hover на стеклянных панелях |
| `glass-bg-active` | `rgba(255, 255, 255, 0.10)` | Active/pressed состояние |
| `glass-bg-elevated` | `rgba(255, 255, 255, 0.07)` | Модалки, dropdowns (чуть ярче) |
| `glass-border` | `rgba(255, 255, 255, 0.10)` | Граница стеклянных панелей |
| `glass-border-subtle` | `rgba(255, 255, 255, 0.06)` | Лёгкие разделители внутри стекла |
| `glass-border-bright` | `rgba(255, 255, 255, 0.15)` | Focus, interactive borders |

### 1.3 Semantic Colors

| Токен | Hex | Применение |
|-------|-----|------------|
| `accent` | `#00d4aa` | Основной акцент (teal-cyan) |
| `accent-hover` | `#00e8bc` | Hover акцента |
| `accent-muted` | `#00d4aa1a` | Accent 10% opacity (backgrounds) |
| `accent-glow` | `#00d4aa40` | Accent 25% opacity (glow effects) |
| `text-primary` | `rgba(255, 255, 255, 0.90)` | Основной текст (90% white) |
| `text-secondary` | `rgba(255, 255, 255, 0.55)` | Вторичный текст |
| `text-tertiary` | `rgba(255, 255, 255, 0.30)` | Placeholders, disabled |
| `text-inverse` | `#08080f` | Текст на accent-фоне |
| `status-connected` | `#00d4aa` | Подключён (= accent) |
| `status-connecting` | `#f0b429` | Подключение (amber) |
| `status-disconnected` | `rgba(255, 255, 255, 0.30)` | Отключён (muted) |
| `status-error` | `#ef4444` | Ошибка (red) |
| `danger` | `#ef4444` | Деструктивные действия |

### 1.4 Light Theme (вторичная)

| Токен | Значение |
|-------|----------|
| `bg-base` | `#f0f0f5` |
| `glass-bg` | `rgba(255, 255, 255, 0.60)` |
| `glass-bg-hover` | `rgba(255, 255, 255, 0.75)` |
| `glass-border` | `rgba(255, 255, 255, 0.40)` |
| `text-primary` | `rgba(0, 0, 0, 0.85)` |
| `text-secondary` | `rgba(0, 0, 0, 0.55)` |
| `accent` | `#009d80` |

Light theme gradient:
```css
[data-theme="light"] .app-background {
  background: #f0f0f5;
  background-image:
    radial-gradient(ellipse 80% 60% at 20% 80%, rgba(0, 212, 170, 0.05) 0%, transparent 50%),
    radial-gradient(ellipse 60% 50% at 80% 20%, rgba(88, 40, 180, 0.04) 0%, transparent 50%),
    linear-gradient(135deg, #e8e8f0 0%, #f0eef8 40%, #e8f0f5 100%);
}
```

### 1.5 Реализация в CSS / Tailwind

CSS custom properties в `@layer base`:
```css
:root, [data-theme="dark"] {
  /* Background scene */
  --color-bg-base: 8 8 15;

  /* Glass surfaces — stored as full rgba for backdrop-filter compat */
  --glass-bg: rgba(255, 255, 255, 0.05);
  --glass-bg-hover: rgba(255, 255, 255, 0.08);
  --glass-bg-active: rgba(255, 255, 255, 0.10);
  --glass-bg-elevated: rgba(255, 255, 255, 0.07);
  --glass-border: rgba(255, 255, 255, 0.10);
  --glass-border-subtle: rgba(255, 255, 255, 0.06);
  --glass-border-bright: rgba(255, 255, 255, 0.15);

  /* Accent */
  --color-accent: 0 212 170;

  /* Text */
  --color-text-primary: rgba(255, 255, 255, 0.90);
  --color-text-secondary: rgba(255, 255, 255, 0.55);
  --color-text-tertiary: rgba(255, 255, 255, 0.30);
  --color-text-inverse: 8 8 15;

  /* Status */
  --color-status-connected: 0 212 170;
  --color-status-connecting: 240 180 41;
  --color-status-error: 239 68 68;
}

[data-theme="light"] {
  --color-bg-base: 240 240 245;
  --glass-bg: rgba(255, 255, 255, 0.60);
  --glass-bg-hover: rgba(255, 255, 255, 0.75);
  --glass-bg-active: rgba(255, 255, 255, 0.85);
  --glass-bg-elevated: rgba(255, 255, 255, 0.70);
  --glass-border: rgba(255, 255, 255, 0.40);
  --glass-border-subtle: rgba(255, 255, 255, 0.25);
  --glass-border-bright: rgba(255, 255, 255, 0.55);
  --color-text-primary: rgba(0, 0, 0, 0.85);
  --color-text-secondary: rgba(0, 0, 0, 0.55);
  --color-text-tertiary: rgba(0, 0, 0, 0.30);
  --color-accent: 0 157 128;
}
```

---

## 2. Типографика

| Назначение | Шрифт | Tailwind | Размер |
|------------|-------|----------|--------|
| UI текст | Inter | `font-sans` | -- |
| Моно (логи, данные) | JetBrains Mono | `font-mono` | -- |
| Заголовок страницы | Inter | `text-2xl font-semibold` | 24px |
| Заголовок карточки | Inter | `text-lg font-semibold` | 18px |
| Подзаголовок | Inter | `text-base font-medium` | 16px |
| Body | Inter | `text-sm` | 14px |
| Caption | Inter | `text-xs` | 12px |
| Stat большой | JetBrains Mono | `font-mono text-3xl font-bold tracking-tight` | 30px |

Текст на стеклянных поверхностях: всегда `text-primary` (90% white) для основного и
`text-secondary` (55% white) для вспомогательного. Минимальный контраст 4.5:1 WCAG AA
гарантирован при glass-bg 5-10% white на фоне темнее `#15152a`.

---

## 3. Отступы

| Имя | Tailwind | px | Применение |
|-----|----------|----|------------|
| xs | `1` | 4 | Icon gaps, badge padding |
| sm | `2` | 8 | Между связанными элементами |
| md | `4` | 16 | Padding карточек, полей |
| lg | `6` | 24 | Между секциями, content padding |
| xl | `8` | 32 | Page-level padding |

**Правила:**
- Card padding: `p-4`
- Gap между карточками: `gap-3`
- Content area padding: `p-6`
- Sidebar item padding: `py-2.5 px-3`

---

## 4. Glass Component Classes

### 4.1 Base Glass Mixin

Общий стиль для всех стеклянных контейнеров. Реализуется как утилитарный класс:

```css
/* @layer utilities */
.glass {
  background: var(--glass-bg);
  backdrop-filter: blur(16px) saturate(180%);
  -webkit-backdrop-filter: blur(16px) saturate(180%);
  border: 1px solid var(--glass-border);
}

.glass-elevated {
  background: var(--glass-bg-elevated);
  backdrop-filter: blur(16px) saturate(180%);
  -webkit-backdrop-filter: blur(16px) saturate(180%);
  border: 1px solid var(--glass-border);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.30);
}

.glass-subtle {
  background: var(--glass-bg);
  backdrop-filter: blur(8px) saturate(150%);
  -webkit-backdrop-filter: blur(8px) saturate(150%);
  border: 1px solid var(--glass-border-subtle);
}
```

Tailwind inline (когда custom class не нужен):
```html
<div class="bg-white/5 backdrop-blur-xl border border-white/10 rounded-xl">
```

### 4.2 Linux Fallback

`backdrop-filter` может не поддерживаться в некоторых Linux оконных менеджерах
(особенно Wayland + software rendering). Fallback:

```css
/* Detect no backdrop-filter support */
@supports not (backdrop-filter: blur(1px)) {
  .glass,
  .glass-elevated,
  .glass-subtle {
    backdrop-filter: none;
    -webkit-backdrop-filter: none;
  }

  .glass {
    background: rgba(18, 18, 26, 0.92);
  }

  .glass-elevated {
    background: rgba(22, 22, 32, 0.95);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.40);
  }

  .glass-subtle {
    background: rgba(18, 18, 26, 0.88);
  }
}
```

Дополнительно: в Rust-бэкенде определяется платформа. Если Linux, фронтенд получает
флаг и может добавить `data-platform="linux"` на body как дополнительный escape hatch:

```css
[data-platform="linux"] .glass {
  background: rgba(18, 18, 26, 0.92);
  backdrop-filter: blur(8px) saturate(150%); /* Reduced blur for performance */
  -webkit-backdrop-filter: blur(8px) saturate(150%);
}
```

### 4.3 Performance Rules

1. **Max blur: 16px** -- больше незаметно визуально, но дороже для GPU
2. **Никогда не stack glass-on-glass** -- если карточка внутри sidebar, карточка не должна
   иметь свой backdrop-blur (используем `glass-subtle` без blur или просто `bg-white/5`)
3. **Sidebar + Content area** -- только sidebar имеет `backdrop-blur`; content area
   использует прозрачный фон, карточки внутри используют `glass`
4. **Модалки** -- overlay `bg-black/40 backdrop-blur-sm` (blur 4px), сама модалка `glass-elevated`
5. **Таблицы** -- blur только на контейнере, не на каждой строке

---

## 5. Компоненты

### Card (Glass)
```html
<div class="glass rounded-xl p-4">
  <!-- Content -->
</div>

<!-- Interactive card (clickable) -->
<div class="glass rounded-xl p-4 transition-colors duration-150
            hover:bg-white/[0.08] hover:border-white/[0.15]
            cursor-pointer">
  <!-- Content -->
</div>

<!-- Card with accent glow (e.g. active profile) -->
<div class="glass rounded-xl p-4 border-accent/30
            shadow-[0_0_20px_rgba(0,212,170,0.08)]">
  <!-- Content -->
</div>
```

### Button -- Primary (Solid Accent)
```html
<button class="bg-accent hover:bg-accent-hover text-text-inverse
               px-4 py-2 rounded-lg text-sm font-medium
               transition-all duration-150
               focus:ring-2 focus:ring-accent/40 focus:outline-none
               disabled:opacity-40 disabled:cursor-not-allowed
               shadow-[0_0_16px_rgba(0,212,170,0.15)]
               hover:shadow-[0_0_24px_rgba(0,212,170,0.25)]">
  Connect
</button>
```

### Button -- Secondary (Glass)
```html
<button class="glass rounded-lg px-4 py-2 text-sm font-medium
               text-text-secondary hover:text-text-primary
               hover:bg-white/[0.08]
               transition-colors duration-150
               focus:ring-2 focus:ring-accent/30 focus:outline-none">
  Cancel
</button>
```

### Button -- Danger
```html
<button class="bg-danger hover:bg-red-500 text-white
               px-4 py-2 rounded-lg text-sm font-medium
               transition-colors duration-150
               focus:ring-2 focus:ring-danger/40 focus:outline-none">
  Delete
</button>
```

### Button -- Ghost (no border)
```html
<button class="bg-transparent hover:bg-white/[0.06]
               text-text-secondary hover:text-text-primary
               px-3 py-2 rounded-lg text-sm
               transition-colors duration-150">
  <!-- Icon button or minimal action -->
</button>
```

### Input
```html
<input class="w-full bg-white/[0.05] border border-white/[0.10]
              rounded-lg px-3 py-2 text-sm text-[--color-text-primary]
              placeholder:text-[--color-text-tertiary]
              focus:border-accent/50 focus:ring-1 focus:ring-accent/20
              focus:outline-none
              transition-colors duration-150" />
```

Не используем backdrop-blur на input -- это вложенный элемент внутри стеклянной карточки.

### Select / Dropdown
```html
<!-- Trigger -->
<button class="w-full bg-white/[0.05] border border-white/[0.10]
               rounded-lg px-3 py-2 text-sm text-left
               text-[--color-text-primary]
               hover:bg-white/[0.08] transition-colors duration-150">
  <span>Selected value</span>
  <ChevronDown />
</button>

<!-- Dropdown panel -->
<div class="glass-elevated rounded-lg mt-1 py-1 min-w-[180px]">
  <div class="px-3 py-2 text-sm text-[--color-text-secondary]
              hover:bg-white/[0.06] hover:text-[--color-text-primary]
              cursor-pointer transition-colors duration-100">
    Option
  </div>
</div>
```

### Toggle Switch
```html
<!-- Container: 40x20px -->
<button class="relative w-10 h-5 rounded-full transition-colors duration-200
               bg-white/[0.12]            {/* OFF */}
               data-[checked]:bg-accent">  {/* ON */}
  <!-- Thumb: 16x16px -->
  <span class="absolute top-0.5 left-0.5 w-4 h-4 rounded-full
               bg-white shadow-sm
               transition-transform duration-200
               data-[checked]:translate-x-5" />
</button>
```

### Badge -- Latency
```html
<!-- Good <100ms -->
<span class="px-2 py-0.5 rounded-md text-xs font-mono
             bg-[rgb(0,212,170)]/15 text-status-connected">42ms</span>

<!-- Medium 100-300ms -->
<span class="px-2 py-0.5 rounded-md text-xs font-mono
             bg-[rgb(240,180,41)]/15 text-status-connecting">186ms</span>

<!-- Bad >300ms / timeout -->
<span class="px-2 py-0.5 rounded-md text-xs font-mono
             bg-[rgb(239,68,68)]/15 text-status-error">timeout</span>
```

### Badge -- Status
```html
<span class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs font-medium
             bg-accent/10 text-accent">
  <span class="w-1.5 h-1.5 rounded-full bg-accent animate-pulse-slow" />
  Connected
</span>
```

### Sidebar Item
```html
<!-- Inactive -->
<div class="flex items-center gap-3 px-3 py-2.5 rounded-lg
            text-[--color-text-secondary]
            hover:bg-white/[0.06] hover:text-[--color-text-primary]
            transition-colors duration-150 cursor-pointer">
  <Icon class="w-5 h-5" />
  <span class="text-sm">Dashboard</span>
</div>

<!-- Active -->
<div class="flex items-center gap-3 px-3 py-2.5 rounded-lg
            bg-accent/10 text-accent
            cursor-pointer">
  <Icon class="w-5 h-5" />
  <span class="text-sm font-medium">Dashboard</span>
</div>
```

### Modal
```html
<!-- Overlay -->
<div class="fixed inset-0 z-50 flex items-center justify-center
            bg-black/40 backdrop-blur-sm">
  <!-- Modal panel -- glass-elevated, NOT stacked inside another glass -->
  <div class="glass-elevated rounded-2xl p-6 w-full max-w-md
              shadow-[0_16px_64px_rgba(0,0,0,0.40)]
              animate-modal-in">
    <h2 class="text-lg font-semibold text-[--color-text-primary] mb-4">Title</h2>
    <!-- Content -->
    <div class="flex justify-end gap-2 mt-6">
      <button class="glass rounded-lg px-4 py-2 text-sm">Cancel</button>
      <button class="bg-accent text-text-inverse px-4 py-2 rounded-lg text-sm font-medium">
        Confirm
      </button>
    </div>
  </div>
</div>
```

### Table (Connections / Logs)
```html
<!-- Container gets glass, rows do NOT -->
<div class="glass rounded-xl overflow-hidden">
  <table class="w-full text-sm">
    <thead>
      <tr class="border-b border-white/[0.06]">
        <th class="text-left px-4 py-3 text-[--color-text-secondary]
                   text-xs font-medium uppercase tracking-wider">
          Host
        </th>
        <!-- ... -->
      </tr>
    </thead>
    <tbody>
      <tr class="border-b border-white/[0.04]
                 hover:bg-white/[0.03] transition-colors duration-100">
        <td class="px-4 py-2.5 text-[--color-text-primary] font-mono text-xs">
          example.com
        </td>
        <!-- ... -->
      </tr>
    </tbody>
  </table>
</div>
```

### Tooltip
```html
<div class="glass-elevated rounded-lg px-3 py-1.5 text-xs
            text-[--color-text-primary]
            shadow-[0_4px_16px_rgba(0,0,0,0.30)]">
  Dashboard
</div>
```

---

## 6. Layout

```
+--[Sidebar 64/224px glass]--+--[Content flex-1 transparent]--+
|  glass (blur)               |  (no blur -- see-through to    |
|  border-r border-white/10   |   app-background gradient)     |
|                             |  p-6                           |
|  [Logo/Icon]                |  [Page Title]                  |
|  [Nav Items]                |  [Glass Cards]                 |
|  [Core Badge]               |                                |
|  [Settings]                 |                                |
+---------[h-screen]----------+-------------------------------+
```

### Full App Shell
```html
<div class="app-background h-screen flex overflow-hidden"
     data-theme="dark"
     data-platform="linux|windows|macos">

  <!-- Custom title bar -->
  <div class="fixed top-0 left-0 right-0 h-8 z-50
              flex items-center px-3
              bg-transparent" data-tauri-drag-region>
    <span class="text-xs text-[--color-text-tertiary] font-medium">Vortex</span>
    <!-- Window controls on right -->
  </div>

  <!-- Sidebar (glass) -->
  <aside class="glass w-16 xl:w-56 h-screen pt-8 flex flex-col
                border-r border-white/[0.10]
                transition-[width] duration-200">
    <!-- Logo -->
    <div class="px-3 py-4 flex items-center justify-center xl:justify-start gap-3">
      <div class="w-8 h-8 rounded-lg bg-accent/20 flex items-center justify-center">
        <span class="text-accent font-bold text-sm">V</span>
      </div>
      <span class="hidden xl:block text-sm font-semibold text-[--color-text-primary]">
        Vortex
      </span>
    </div>

    <!-- Nav items -->
    <nav class="flex-1 px-2 py-2 space-y-1">
      <!-- Sidebar items here -->
    </nav>

    <!-- Core badge at bottom -->
    <div class="px-3 py-3 border-t border-white/[0.06]">
      <span class="text-xs font-mono text-[--color-text-tertiary]">Mihomo</span>
    </div>
  </aside>

  <!-- Main content (transparent -- no glass) -->
  <main class="flex-1 pt-8 overflow-y-auto">
    <div class="p-6">
      <!-- Page content with glass cards -->
    </div>
  </main>
</div>
```

- Sidebar collapsed: `w-16` (64px) -- иконки только
- Sidebar expanded: `w-56` (224px) -- иконки + текст
- Breakpoint: `xl:` (1280px) для auto-expand
- Min window: 800x560
- Default window: 1000x680
- Custom title bar: `h-8` (32px), drag region

---

## 7. Connection Orb (Dashboard Hero)

Центральный элемент дашборда -- стеклянное кольцо с glow-эффектом, отражающим статус.

### Structure
```html
<div class="relative flex items-center justify-center">
  <!-- Outer glow (behind orb) -->
  <div class="absolute w-32 h-32 rounded-full
              bg-accent/10 blur-2xl
              animate-orb-glow" />

  <!-- Glass ring -->
  <div class="relative w-24 h-24 rounded-full
              glass
              flex items-center justify-center
              shadow-[0_0_30px_rgba(0,212,170,0.12),inset_0_0_20px_rgba(0,212,170,0.05)]
              border-2 border-accent/30">

    <!-- Inner circle -->
    <div class="w-16 h-16 rounded-full bg-accent/10
                flex items-center justify-center">
      <Power class="w-6 h-6 text-accent" />
    </div>
  </div>
</div>
```

### States

| State | Ring border | Glow color | Inner bg | Animation |
|-------|-----------|------------|----------|-----------|
| Disconnected | `border-white/[0.10]` | None | `bg-white/[0.05]` | None |
| Connecting | `border-amber-400/40` | `bg-amber-400/10` | `bg-amber-400/10` | `animate-orb-pulse` (1s) |
| Connected | `border-accent/30` | `bg-accent/10` | `bg-accent/10` | `animate-orb-glow` (3s) |
| Error | `border-red-500/40` | `bg-red-500/10` | `bg-red-500/10` | `animate-orb-pulse` (0.8s) |

---

## 8. Анимации

### Orb Glow (Connected)
```css
@keyframes orb-glow {
  0%, 100% {
    opacity: 0.6;
    transform: scale(1);
  }
  50% {
    opacity: 1;
    transform: scale(1.08);
  }
}
```
Duration: 3s, ease-in-out, infinite. Тонкий, не отвлекающий.

### Orb Pulse (Connecting / Error)
```css
@keyframes orb-pulse {
  0%, 100% {
    box-shadow: 0 0 0 0 rgb(var(--pulse-color) / 0.3);
  }
  50% {
    box-shadow: 0 0 0 14px rgb(var(--pulse-color) / 0);
  }
}
```
Connecting: 1.5s, amber. Error: 1s, red.

### Modal Enter
```css
@keyframes modal-in {
  from {
    opacity: 0;
    transform: scale(0.96) translateY(8px);
  }
  to {
    opacity: 1;
    transform: scale(1) translateY(0);
  }
}
```
Duration: 200ms, ease-out.

### Page Transition
```css
@keyframes fade-in {
  from { opacity: 0; transform: translateY(4px); }
  to { opacity: 1; transform: translateY(0); }
}
```
Duration: 150ms, ease-out. Без exit-анимации.

### Interactive Feedback
Все интерактивные элементы: `transition-colors duration-150`.
Glass hover transitions: `transition-all duration-150` (для bg + border + shadow).

---

## 9. Tailwind Config

```ts
// tailwind.config.ts
import type { Config } from "tailwindcss";

export default {
  content: ["./index.html", "./src/**/*.{ts,tsx}"],
  darkMode: ["selector", '[data-theme="dark"]'],
  theme: {
    extend: {
      fontFamily: {
        sans: ["Inter", "system-ui", "sans-serif"],
        mono: ["JetBrains Mono", "monospace"],
      },
      colors: {
        "bg-base": "rgb(var(--color-bg-base) / <alpha-value>)",
        accent: {
          DEFAULT: "rgb(var(--color-accent) / <alpha-value>)",
          hover: "#00e8bc",
          muted: "rgba(0, 212, 170, 0.10)",
          glow: "rgba(0, 212, 170, 0.25)",
        },
        danger: {
          DEFAULT: "#ef4444",
          hover: "#f87171",
        },
        status: {
          connected: "rgb(var(--color-status-connected) / <alpha-value>)",
          connecting: "rgb(var(--color-status-connecting) / <alpha-value>)",
          error: "rgb(var(--color-status-error) / <alpha-value>)",
        },
      },
      backdropBlur: {
        xs: "2px",
        glass: "16px",
      },
      animation: {
        "orb-glow": "orb-glow 3s ease-in-out infinite",
        "orb-pulse": "orb-pulse 1.5s ease-in-out infinite",
        "orb-pulse-fast": "orb-pulse 1s ease-in-out infinite",
        "modal-in": "modal-in 200ms ease-out",
        "fade-in": "fade-in 150ms ease-out",
        "pulse-slow": "pulse 3s ease-in-out infinite",
      },
      keyframes: {
        "orb-glow": {
          "0%, 100%": { opacity: "0.6", transform: "scale(1)" },
          "50%": { opacity: "1", transform: "scale(1.08)" },
        },
        "orb-pulse": {
          "0%, 100%": { boxShadow: "0 0 0 0 var(--pulse-color, rgba(0,212,170,0.3))" },
          "50%": { boxShadow: "0 0 0 14px var(--pulse-color-end, rgba(0,212,170,0))" },
        },
        "modal-in": {
          from: { opacity: "0", transform: "scale(0.96) translateY(8px)" },
          to: { opacity: "1", transform: "scale(1) translateY(0)" },
        },
        "fade-in": {
          from: { opacity: "0", transform: "translateY(4px)" },
          to: { opacity: "1", transform: "translateY(0)" },
        },
      },
      boxShadow: {
        glass: "0 8px 32px rgba(0, 0, 0, 0.20)",
        "glass-lg": "0 16px 64px rgba(0, 0, 0, 0.35)",
        "accent-glow": "0 0 20px rgba(0, 212, 170, 0.12)",
        "accent-glow-lg": "0 0 40px rgba(0, 212, 170, 0.20)",
      },
    },
  },
  plugins: [
    // Custom glass utilities
    function ({ addUtilities }: { addUtilities: Function }) {
      addUtilities({
        ".glass": {
          background: "var(--glass-bg)",
          "backdrop-filter": "blur(16px) saturate(180%)",
          "-webkit-backdrop-filter": "blur(16px) saturate(180%)",
          border: "1px solid var(--glass-border)",
        },
        ".glass-elevated": {
          background: "var(--glass-bg-elevated)",
          "backdrop-filter": "blur(16px) saturate(180%)",
          "-webkit-backdrop-filter": "blur(16px) saturate(180%)",
          border: "1px solid var(--glass-border)",
          "box-shadow": "0 8px 32px rgba(0, 0, 0, 0.30)",
        },
        ".glass-subtle": {
          background: "var(--glass-bg)",
          "backdrop-filter": "blur(8px) saturate(150%)",
          "-webkit-backdrop-filter": "blur(8px) saturate(150%)",
          border: "1px solid var(--glass-border-subtle)",
        },
        ".glass-none": {
          background: "transparent",
          "backdrop-filter": "none",
          "-webkit-backdrop-filter": "none",
          border: "none",
        },
      });
    },
  ],
} satisfies Config;
```

---

## 10. Design Tokens Summary

Быстрая шпаргалка для разработчиков:

| Элемент | Класс / Стиль |
|---------|---------------|
| App background | `.app-background` (gradient CSS) |
| Card | `.glass.rounded-xl.p-4` |
| Sidebar | `.glass.border-r.border-white/10` |
| Modal overlay | `.bg-black/40.backdrop-blur-sm` |
| Modal panel | `.glass-elevated.rounded-2xl.p-6` |
| Button primary | `.bg-accent.text-text-inverse.rounded-lg` |
| Button secondary | `.glass.rounded-lg` |
| Input | `.bg-white/5.border.border-white/10.rounded-lg` |
| Table container | `.glass.rounded-xl.overflow-hidden` |
| Table row hover | `.hover:bg-white/[0.03]` |
| Dropdown | `.glass-elevated.rounded-lg` |
| Tooltip | `.glass-elevated.rounded-lg.text-xs` |
| Connection orb | `.glass` ring + glow animation |
| Text primary | `text-[--color-text-primary]` or Tailwind token |
| Text secondary | `text-[--color-text-secondary]` |
| Border default | `border-white/10` |
| Border subtle | `border-white/6` |
| Divider | `border-b border-white/[0.06]` |

---

## 11. Rationale

- **Glassmorphism** создаёт ощущение глубины и современности, подходит для VPN-клиента
  (ассоциация с "прозрачностью" и "защитой через слои")
- **Gradient background scene** необходим -- без него стеклянные панели выглядят как
  обычные полупрозрачные блоки. Subtle radial gradients с purple/teal дают цветовое
  богатство при размытии
- **Teal-cyan `#00d4aa`** -- идеальный акцент для glassmorphism: яркий, чистый,
  отлично светится (glow) на тёмном фоне
- **Glass only on containers** -- blur на каждом элементе убивает производительность
  и визуальную иерархию. Карточки, сайдбар, модалки = стекло. Кнопки, inputs, badges = нет
- **16px blur cap** -- визуально достаточно для эффекта, 20px+ даёт незначительное улучшение
  при значительном росте нагрузки на GPU
- **Linux fallback** -- solid semi-transparent backgrounds при отсутствии backdrop-filter;
  reduced blur (8px) на Linux даже при поддержке для стабильности
- **No glass-on-glass stacking** -- двойной blur визуально мутный и дорогой; nested
  элементы используют `bg-white/5` без blur
- **Inter + JetBrains Mono** -- не меняем, отлично читаются на стеклянных поверхностях
  при white/90% opacity
- **Минимум анимаций** -- orb glow медленный (3s) и тонкий; transitions 150ms;
  glass сам по себе декоративен, не нужно добавлять motion
