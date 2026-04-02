# Vortex

Multi-core VPN client with a modern UI. Built with Tauri v2 (Rust) and React (TypeScript).

Vortex is a GUI wrapper for two powerful proxy cores:
- **[Mihomo](https://github.com/MetaCubeX/mihomo)** (Clash Meta) — feature-rich proxy with rule-based routing
- **[Xray-core](https://github.com/XTLS/Xray-core)** — high-performance proxy with XTLS/REALITY support

## Features

- Dual-core support: switch between Mihomo and Xray seamlessly
- TUN mode for system-wide proxying
- Rule-based traffic routing
- Subscription management (Base64, SIP008, Clash YAML)
- Protocol support: VMess, VLESS, Trojan, Shadowsocks, Hysteria2, TUIC, WireGuard
- Real-time traffic statistics
- Cross-platform: Windows, macOS, Linux

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Frontend | React + TypeScript + Tailwind CSS |
| Backend | Rust (Tauri v2) |
| Bundler | Vite |
| Cores | Mihomo (Go), Xray-core (Go) |

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) >= 18
- [pnpm](https://pnpm.io/)
- [Rust](https://rustup.rs/) >= 1.77
- Platform-specific dependencies for [Tauri v2](https://v2.tauri.app/start/prerequisites/)

### Development

```bash
# Install frontend dependencies
pnpm install

# Start in development mode
pnpm tauri dev
```

### Build

```bash
# Build production installer
pnpm tauri build
```

## Project Structure

```
src/                    # React frontend
src-tauri/
  src/                  # Rust backend logic
  bin/[platform]/       # Proxy core binaries (sidecar)
  capabilities/         # Tauri v2 security permissions
```

## License

[MIT](LICENSE)
