# CPM (CrestCode Package Manager)

[![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)
[![Language: Rust](https://img.shields.io/badge/language-Rust_2024-orange.svg)](https://www.rust-lang.org/)
[![Ecosystem: CrestCode](https://img.shields.io/badge/ecosystem-CrestCode-purple.svg)](https://github.com/heySaish/Crest-Tools)

**CPM** is a high-performance CLI tool built with Rust for Alpine Linux & CrestCode environments. It provides an intuitive APT-style frontend wrapper over Alpine's native `apk` package manager alongside a dedicated extension manager for the CrestCode Android editor.

---

## ⚡ Features

- **Modern CLI UX**: Rich terminal output with colorful status badges, spinners (`indicatif`), and clean structured tables (`comfy-table`).
- **APT-style Syntax**: Friendly, familiar commands (`cpm install`, `cpm remove`, `cpm update`, `cpm upgrade`) wrapping Alpine's `apk`.
- **Simulation Mode**: Run dry-runs (`--simulate` / `-s`) before modifying system packages.
- **CrestCode Extension Manager**: Create, list, inspect, and remove extensions for CrestCode (`~/.crestcode/extensions/`).

---

## 🚀 Installation & Building

Since CPM is written in Rust, building from source is simple:

```bash
# Navigate to the package directory
cd packages/cpm

# Build release binary
cargo build --release

# The compiled binary will be at:
# target/release/cpm
```

To install it into your local `$PATH` (e.g. `~/.local/bin` or `/usr/local/bin`):

```bash
cargo install --path .
```

---

## 📖 Usage Guide

### 1. Package Management (`apk` wrapper)

| Command | Equivalent `apk` | Description |
| :--- | :--- | :--- |
| `cpm install <pkg...>` | `apk add <pkg...>` | Installs one or more packages |
| `cpm remove <pkg...>` | `apk del <pkg...>` | Removes specified packages |
| `cpm update` | `apk update` | Updates package repository indexes |
| `cpm upgrade` | `apk upgrade` | Upgrades installed packages |
| `cpm search <query>` | `apk search <query>` | Searches available packages |
| `cpm list` | `apk info` | Lists all installed packages |

#### Options:
- `--simulate` / `-s`: Preview package operations without executing changes.
  ```bash
  cpm install git curl -s
  ```

---

### 2. CrestCode Extension Management

CPM manages extensions installed inside `~/.crestcode/extensions/`.

| Command | Description |
| :--- | :--- |
| `cpm ext list` | Lists all installed CrestCode extensions |
| `cpm ext new <id> [name]` | Scaffolds a new extension directory with `manifest.json` & `index.js` |
| `cpm ext remove <id>` | Removes an installed extension by ID |

#### Example: Creating a new extension
```bash
cpm ext new theme-dracula "Dracula Theme"
```

---

## 📂 Project Structure

```text
packages/cpm/
├── Cargo.toml          # Rust package manifest & dependencies
├── Cargo.lock          # Dependency lockfile
├── LICENSE             # Apache 2.0 License
├── README.md           # Documentation
└── src/
    ├── main.rs         # CLI entry point (Clap CLI parser)
    ├── backend.rs      # Apk execution wrapper logic
    ├── ui.rs           # Formatting, colored indicators & tables
    └── commands/
        ├── mod.rs
        ├── install.rs
        ├── remove.rs
        ├── update.rs
        ├── upgrade.rs
        ├── search.rs
        ├── list.rs
        └── extension.rs # CrestCode extension lifecycle
```

---

## 📜 License

Licensed under the [Apache License, Version 2.0](LICENSE).
You may obtain a copy of the License at [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0).
