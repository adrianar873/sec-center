<div align="center">
  <h1>SEC CENTER</h1>
  <p><em>Security & Network Control Center</em></p>
</div>

<p align="center">
  <img src="Screenshot%20from%202026-05-08%2021-35-39.png" alt="SecCenter Screenshot" width="750"/>
</p>

---

##  Features

| Tab | Description |
|-----|-------------|
| **System** | CPU, RAM, disk, GPU & temperature monitoring |
| **Network** | Local network scan + device discovery + baseline tracking |
| **Ports** | Port scanning with service detection |
| **Connections** | Active network connections per process |
| **Actions** | One-click: flush cache, export report, ping test, system update |

##  Quick Start

```bash
cargo run --release
```

### Controls
| Key | Action |
|-----|--------|
| `Tab` / `←` `→` | Navigate tabs |
| `S` | Scan (Network / Ports tab) |
| `B` | Set network baseline (Network tab) |
| `↑` `↓` / `Enter` | Select & run action (Actions tab) |
| `q` | Quit |

## 📁 Structure

```
.
├── Cargo.toml
├── Cargo.lock
├── README.md
├── src/
│   ├── main.rs
│   ├── app.rs
│   ├── actions/
│   │   └── mod.rs
│   ├── connections/
│   │   ├── mod.rs
│   │   └── monitor.rs
│   ├── network/
│   │   ├── mod.rs
│   │   ├── devices.rs
│   │   ├── scanner.rs
│   │   └── storage.rs
│   ├── ports/
│   │   ├── mod.rs
│   │   ├── scanner.rs
│   │   └── services.rs
│   ├── system/
│   │   ├── mod.rs
│   │   ├── cpu.rs
│   │   ├── disk.rs
│   │   ├── gpu.rs
│   │   ├── memory.rs
│   │   ├── motherboard.rs
│   │   ├── processes.rs
│   │   └── temperature.rs
│   └── ui/
│       ├── mod.rs
│       ├── actions_view.rs
│       ├── connections_view.rs
│       ├── help_view.rs
│       ├── layout.rs
│       ├── network_view.rs
│       ├── popup.rs
│       ├── ports_view.rs
│       └── system_view.rs
```

##  Libs

- [Ratatui](https://github.com/ratatui-org/ratatui) — terminal UI framework
- [Crossterm](https://github.com/crossterm-rs/crossterm) — terminal control
- [Sysinfo](https://github.com/GuillaumeGomez/sysinfo) — system information
- [Procfs](https://github.com/eminence/procfs) — Linux proc filesystem
- [Serde](https://serde.rs/) — serialization

## Installation

```bash
git clone <repo-url>
cd sec-center
cargo run --release
```

## 📄 License

MIT
