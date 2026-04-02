# Design Research: Proxy/VPN Client Applications

## 1. Application Comparison

### 1.1 Clash Verge Rev

**Tech:** Tauri 2 + React + TypeScript (59.7% TS codebase)
**Platforms:** Windows, macOS, Linux
**Core:** Mihomo (Clash Meta)

**UI/UX Design:**
- Clean, minimalist sidebar navigation with tabs: Proxies, Profiles, Rules, Connections, Logs, Settings
- Custom theme colors with dark/light mode
- CSS injection support for deep UI customization
- Custom proxy group and tray icons
- Visual node and rule editor (not just text/YAML editing)
- Syntax-highlighted YAML editor for profiles

**Subscription Management:**
- Remote profile import via URL (Clash-format only)
- Displays traffic usage and subscription expiry when headers are present
- Manual refresh via top-right controls
- Local profile editing with external editor integration

**Profile Enhancement System (unique):**
- 4 profile types: Remote, Local, Script (JS via quickjs), Merge (YAML)
- Sequential chain processing: main profile -> Script/Merge profiles in order
- Merge profiles can append/prepend rules, proxies, proxy-groups
- Script profiles use JS `main()` function for dynamic config modification
- Error handling with visual red highlighting on invalid scripts

**Proxy Group Switching:**
- Proxies tab shows groups with selection strategies (url-test, select, fallback)
- Tap to select active node within each group

**Connections Tab:**
- Real-time list of all active connections
- Shows hostname, matched rule, proxy used (or DIRECT), protocol, bytes transferred
- Click any row for full connection details
- Keyword filtering

**Configuration Sync:** WebDAV backup/sync across devices

**Sources:**
- [GitHub - clash-verge-rev](https://github.com/clash-verge-rev/clash-verge-rev)
- [Clash Verge Rev User Guide](https://clashvergerev.com/en/guide)
- [Clash Verge Rev Official](https://clashverge.net/en/)

---

### 1.2 Hiddify

**Tech:** Flutter (Dart) + Go (libbox native engine) + Riverpod state management
**Platforms:** Windows, macOS, Linux, Android, iOS
**Core:** Sing-box (with V2ray/Clash config format support)

**UI/UX Design:**
- Material Design with dynamic theming and color adaptation
- Light/dark mode with system preference detection
- Layered Flutter architecture: ~90% shared codebase across all platforms
- Platform-specific wrappers (WindowWrapper for desktop, ConnectionWrapper for VPN)

**Subscription Management:**
- Multiple input methods: manual URL, QR code scanning, clipboard auto-detection, curated free profile lists
- `ProfileParser` detects format variants (base64, JSON, YAML) and validates against Sing-box
- Subscription headers parsed for traffic limits and expiration dates
- Per-profile override settings stored in Drift SQLite database
- Profiles support per-profile customization of global options

**Service Modes (unique):**
- Three modes: VPN (TUN), System Proxy, Proxy-only
- Users choose their preferred network interception method

**Proxy Group Switching:**
- Reactive streaming of active outbound groups via `watchActiveGroups()`
- Real-time UI updates during node/group switches
- Core maintains connection state during transitions

**Connection Status:**
- `ConnectionNotifier` streams states: stopped, starting, started, stopping
- Visual feedback throughout the app lifecycle
- Traffic stats via `watchStats()`: CPU, memory, upload/download bytes

**Configuration (layered override pattern):**
1. Default values (hardcoded in ConfigOptions)
2. User settings (SharedPreferences)
3. Per-profile overrides (SQLite database)
4. Merged configuration via `fullOptionsOverrided()`
- Changes applied without restart via gRPC to native core

**Unique Features:**
- Per-app proxy routing on Android
- TLS tricks: fragmentation, padding, mixed SNI case
- Cloudflare WARP integration with customizable noise parameters
- RAM consumption reduced 50% in 2025 updates

**Sources:**
- [Hiddify GitHub](https://github.com/hiddify/hiddify-app)
- [Hiddify DeepWiki](https://deepwiki.com/hiddify/hiddify-app/1-hiddify-overview)
- [Hiddify Website](https://hiddify.com/)

---

### 1.3 NekoBox / NekoRay

**Tech:** Qt (C++) with sing-box/Xray backend
**Platforms:** Windows, Linux (no longer maintained as of late 2024)
**Cores:** Xray, Sing-Box (user-selectable)

**UI/UX Design:**
- Traditional Qt desktop application UI (not web-based)
- Functional/utilitarian rather than modern/polished
- Portable builds that run without installation

**Protocol Support:**
- Shadowsocks, VMess, VLESS, WireGuard, Trojan, Trojan-Go, NaiveProxy, Hysteria, TUIC
- Custom outbound settings and custom core options

**Subscription Management:**
- Subscription-based configuration import
- Manual server addition

**Key Takeaway:** No longer maintained. The fork `nekoray-ng` continues development. The Qt approach results in a more traditional desktop look that feels dated compared to Tauri/web-based alternatives.

**Sources:**
- [NekoRay GitHub](https://github.com/MatsuriDayo/nekoray)
- [NekoBox Website](https://getnekobox.com/en/)

---

### 1.4 V2RayN

**Tech:** Dual UI - WPF (Windows) + Avalonia (cross-platform), shared ViewModels via ServiceLib
**Platforms:** Windows, Linux, macOS
**Cores:** Xray, sing-box, mihomo (multi-core support)

**UI Architecture (exemplary):**
- MVVM pattern with ReactiveUI
- 100% shared business logic between WPF and Avalonia UIs
- MaterialDesignThemes (WPF) / Semi.Avalonia (cross-platform)
- Publish-Subscribe event system (`AppEvents`) for cross-component communication

**Subscription Management:**
- Import via clipboard, QR codes, or subscription URLs
- Server profiles stored in SQLite with metadata in `ProfileExManager`
- Efficient filtering by subscription ID and bulk operations
- Atomic write pattern: serialize to temp file, then File.Move() to prevent corruption

**Proxy Group Switching:**
- Profile selection triggers `MainWindowViewModel.Reload()`
- Semaphore-based locking prevents concurrent reload executions
- Queues subsequent reload requests

**Connection Status & Traffic Stats:**
- `StatusBarViewModel` for system proxy indicators
- Speed test results published via ReactiveUI `Subject<T>` observables
- Callbacks marshaled to main UI thread via `RxApp.MainThreadScheduler`

**Configuration:**
- Hierarchical object model with 20+ specialized sections
- `Global` class centralizes constants (encryption methods, protocols, TLS fingerprints)
- 7 language support via .resx files

**Design Patterns Worth Adopting:**
- Repository Pattern for data persistence abstraction
- Strategy Pattern for core-specific config generation (CoreConfigV2rayService, CoreConfigSingboxService, CoreConfigClashService)
- Event-driven architecture with static Subject<T> observables

**Sources:**
- [V2RayN GitHub](https://github.com/2dust/v2rayN)
- [V2RayN DeepWiki](https://deepwiki.com/2dust/v2rayN/1-overview)

---

### 1.5 Clash Nyanpasu

**Tech:** Tauri 2 + React 19 + Material-UI 7 + Tailwind CSS 4 + Vite 7
**Platforms:** Windows, macOS, Linux
**Cores:** Mihomo, Clash Premium, Clash Rust (user-selectable)

**UI Architecture (most relevant to Vortex):**
- Three-layer package structure:
  - `@nyanpasu/interface` - Type-safe IPC wrappers + React Query hooks
  - `@nyanpasu/ui` - Shared Material Design 3 component library
  - `@nyanpasu/nyanpasu` - Main app with routing, pages, business logic
- TanStack Router for navigation (route-based code splitting)
- Jotai for local atomic UI state
- TanStack Query for server state caching and synchronization

**Material You Design (Google MD3):**
- Dynamic color theming using `@material/material-color-utilities`
- CSS custom properties for theming tokens
- Emotion for dynamic style generation
- Light/dark mode with system detection + manual override
- Framer Motion 12 for smooth animations and transitions

**Proxy Group Switching:**
- TanStack Query mutations for state sync
- IPC commands to backend CoreManager
- Real-time UI updates on successful operations

**Traffic Statistics:**
- Live connection tracking with traffic graphs
- D3.js visualization for historical traffic trends
- Per-connection bandwidth metrics
- Latency indicators for proxy health

**Profile Management:**
- Remote profile fetching and caching via HTTP
- Local file editing with Monaco Editor (with syntax highlighting + autocompletion)
- Script profile creation using JavaScript/Lua
- Profile chain merging with visual dependency graphs
- Drag-and-drop reordering via @dnd-kit

**Settings:**
- Verge Configuration: theme, language, core selection
- System integration toggles: system proxy, TUN mode, auto-launch
- React Hook Form + Zod schema validation for forms

**Performance Optimizations:**
- Route-based code splitting (TanStack Router)
- Async component loading
- TanStack Query prevents redundant backend requests
- React memoization to reduce re-renders

**Sources:**
- [Clash Nyanpasu Website](https://clashnyanpasu.xyz/en/)
- [Clash Nyanpasu DeepWiki](https://deepwiki.com/libnyanpasu/clash-nyanpasu)
- [Clash Nyanpasu GitHub](https://github.com/libnyanpasu/clash-nyanpasu)

---

## 2. Remnawave HWID System

### How It Works

Remnawave uses HWID headers for per-subscription device limits. When a client fetches a subscription, it sends identification headers.

**Required Header:**
- `x-hwid` - The hardware identifier (mandatory)

**Optional Headers:**
- `x-device-os` - Operating system name
- `x-ver-os` - OS version
- `x-device-model` - Device model
- `user-agent` - Client user agent string

**Server Response Headers:**
- `x-hwid-active` - Confirms HWID feature is enabled
- `x-hwid-not-supported` - Client is missing x-hwid header
- `x-hwid-max-devices-reached` - Device limit exceeded
- `x-hwid-limit` - Backwards compatibility flag

**Critical Note:** If HWID Device Limit is enabled on the server and the client does not send an `x-hwid` header, the subscription fetch will fail entirely.

**Admin Controls:**
- Per-user custom device limits
- Fallback limit when no per-user value is set
- Per-user HWID disable toggle
- Device list viewing and individual device removal
- Search by HWID

**Sources:**
- [Remnawave HWID Docs](https://docs.rw/docs/features/hwid-device-limit/)
- [Remnawave Panel GitHub](https://github.com/remnawave/panel/blob/main/docs/features/hwid-device-limit.md)

---

## 3. HWID Generation Best Practices

### Recommended Approach for Vortex

**Composite Hardware Fingerprint (survives reinstall):**
Combine 3+ hardware identifiers, hash the result. The HWID is rooted in physical hardware so it persists across OS reinstalls.

**Recommended components (in priority order):**
1. System UUID / Motherboard UUID (most stable)
2. CPU ID / Processor serial
3. OS Disk serial number
4. CPU core count (supplementary)

**Avoid:** MAC address (changes when network is disabled), username (user can change), hostname (user can change).

**Fuzzy Matching:** If using composite fingerprints, implement fuzzy matching so that a partial hardware change (e.g., new disk) still recognizes the machine when 2/3 components match. This prevents unnecessary re-registration after hardware upgrades.

### Rust Implementation

**Best crate: `machineid-rs`** (v1.2.4)
- Cross-platform: Windows, Linux, macOS
- No admin privileges required
- Builder pattern for component selection
- SHA256/SHA1/MD5 hashing

```rust
use machineid_rs::{IdBuilder, Encryption, HWIDComponent};

let mut builder = IdBuilder::new(Encryption::SHA256);
builder
    .add_component(HWIDComponent::SystemID)    // Motherboard UUID
    .add_component(HWIDComponent::CPUCores)    // CPU core count
    .add_component(HWIDComponent::CPUID);      // CPU serial

let hwid = builder.build("vortex-salt-key").unwrap();
```

**Alternative crate: `hardware-id`** - simpler API, also cross-platform.

**Sources:**
- [machineid-rs GitHub](https://github.com/Taptiive/machineid-rs)
- [hardware-id on lib.rs](https://lib.rs/crates/hardware-id)
- [crates.io HWID keyword](https://crates.io/keywords/hwid)

---

## 4. Modern VPN/Proxy App Design Patterns

### Dark Mode Best Practices (2025-2026)
- Subtle gradients and neon-style accent highlights for depth
- Glass-like UI effects with blur and transparency for modals/cards
- Cleaner layouts with reduced text, high-impact visuals
- 82% of users prefer dark interfaces for extended sessions
- Use `#121212` or similar near-black (not pure black) for backgrounds
- Maintain minimum 4.5:1 contrast ratio for text

### Common Navigation Pattern Across All Studied Apps
Every serious proxy client uses a **left sidebar** with these core sections:
1. **Dashboard / Home** - Quick connect, status overview
2. **Proxies** - Proxy groups and node selection
3. **Profiles / Subscriptions** - Subscription management
4. **Rules** - Routing rules viewer/editor
5. **Connections** - Active connection list
6. **Logs** - Core output logs
7. **Settings** - App configuration

### Key UX Patterns Worth Adopting

| Pattern | Used By | Description |
|---------|---------|-------------|
| Profile chain/merge | Clash Verge, Nyanpasu | Layer config modifications via scripts/merge |
| Visual rule editor | Clash Verge Rev | Edit routing rules without touching YAML |
| Monaco code editor | Clash Nyanpasu | In-app config editing with syntax highlighting |
| Multi-core selector | V2RayN, NekoBox, Nyanpasu | Switch between proxy cores from settings |
| Drag-and-drop reorder | Clash Nyanpasu | Reorder profiles and proxy groups |
| Real-time traffic graph | Clash Nyanpasu | D3.js visualization of bandwidth |
| Per-connection detail | Clash Verge Rev | Click any connection row for full metadata |
| Subscription header parsing | Hiddify, Clash Verge | Extract expiry/traffic from sub headers |
| WebDAV config sync | Clash Verge Rev | Cross-device configuration backup |
| TLS tricks UI | Hiddify | TLS fragmentation/padding toggles |
| Service mode selector | Hiddify | VPN (TUN) / System Proxy / Proxy-only |
| CSS injection theming | Clash Verge Rev | Power-user UI customization |

---

## 5. Actionable Recommendations for Vortex

### Architecture (adopt from Clash Nyanpasu)
- **Package structure**: Separate `interface` (IPC hooks), `ui` (component library), and `app` (pages/routing) packages
- **State management**: Jotai for local UI state + TanStack Query for backend state (matches Tauri IPC well)
- **Routing**: TanStack Router with route-based code splitting
- **Forms**: React Hook Form + Zod for settings validation

### UI/UX (highest priority)
1. **Left sidebar navigation** with icon+label: Dashboard, Proxies, Profiles, Rules, Connections, Logs, Settings
2. **Dark-first design** with light mode toggle, using Material Design 3 color tokens
3. **Dashboard view**: Large connect button, current node info, real-time upload/download speed, traffic graph
4. **Proxy groups view**: Card-based layout per group, node list with latency badges, one-click selection
5. **Profiles view**: Card per subscription showing name, traffic used/total, expiry date, last updated, with refresh button
6. **Connections view**: Sortable table with hostname, rule, proxy, protocol, up/down bytes, duration

### Subscription Management
- Accept URL import, clipboard detection, and QR code
- Parse subscription response headers for traffic/expiry metadata
- Send Remnawave-compatible headers: `x-hwid`, `x-device-os`, `x-ver-os`, `x-device-model`
- Auto-refresh on configurable interval
- Display subscription info card: remaining data, days until expiry, node count

### HWID Implementation
- Use `machineid-rs` crate with SHA256 encryption
- Combine SystemID + CPUID + CPUCores for stable fingerprint
- Send as `x-hwid` header on every subscription fetch
- Also send `x-device-os` (from `std::env::consts::OS`) and `x-device-model` (hostname)

### Core Management (adopt from V2RayN)
- Strategy pattern for core-specific config generation (MihomoConfigService, XrayConfigService)
- Atomic config file writes (write to temp, then rename)
- Store server profiles in SQLite for efficient querying
- Semaphore-based reload locking to prevent concurrent core restarts

### Differentiators to Consider
- **Dual-core advantage**: Unlike most apps that use only one core, Vortex supports both Mihomo and Xray - expose this as a key feature with easy core switching
- **Visual config editor**: Go beyond text editing; provide a form-based editor for common proxy/rule configurations
- **Smart node selection**: Implement delay-based auto-selection like Hiddify rather than simple latency test
