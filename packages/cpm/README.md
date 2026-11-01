# CPM (CrestEditor Package Manager)

[![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)
[![Language: Rust](https://img.shields.io/badge/language-Rust_2024-orange.svg)](https://www.rust-lang.org/)
[![Ecosystem: CrestEditor](https://img.shields.io/badge/ecosystem-CrestEditor-purple.svg)](https://github.com/heySaish/Crest-Tools)

**CPM** is a high-performance CLI tool built with Rust for Alpine Linux & CrestEditor environments. It provides an intuitive APT-style frontend wrapper over Alpine's native `apk` package manager alongside a dedicated extension manager for the CrestEditor Android editor.

---

## ⚡ Features

- **Modern CLI UX**: Rich terminal output with colorful status badges, spinners (`indicatif`), and clean structured tables (`comfy-table`).
- **APT-style Syntax**: Friendly, familiar commands (`cpm install`, `cpm remove`, `cpm update`, `cpm upgrade`) wrapping Alpine's `apk`.
- **Simulation Mode**: Run dry-runs (`--simulate` / `-s`) before modifying system packages.
- **CrestEditor Extension Manager**: Create, list, inspect, and remove extensions for CrestEditor (`~/.crestcode/extensions/`).

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

### 2. Configuration Settings (`cpm config`)

CPM stores persistent settings in `~/.config/cpm/config.toml`.

| Command | Description |
| :--- | :--- |
| `cpm config list` | Displays all configuration keys and current values |
| `cpm config get [key]` | Retrieves value of a specific setting |
| `cpm config set <key> <value>` | Updates a setting (`true`/`false`, `yes`/`no`, `1`/`0`) |
| `cpm config path` | Prints full path to the configuration file |
| `cpm config reset` | Resets all settings to default values |

#### Available Options:
- `show_banner`: Enable/disable ASCII banner on execution (default: `true`).
  ```bash
  cpm config set show_banner false
  ```
- `auto_yes`: Automatically confirm all interactive prompts (default: `false`).
- `color`: Enable/disable colorized terminal styling (default: `true`).
- `show_hints`: Enable/disable helpful hints (default: `true`).

You can also bypass the banner on single commands via `--no-banner`:
```bash
cpm --no-banner install curl
```

---

### 3. CrestEditor Extension Management

CPM manages extensions installed inside `~/.crestcode/extensions/`.

| Command | Description |
| :--- | :--- |
| `cpm ext list` | Lists all installed CrestEditor extensions |
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
    ├── config.rs       # Configuration loader & manager
    ├── ui.rs           # Formatting, colored indicators & tables
    └── commands/
        ├── mod.rs
        ├── install.rs
        ├── remove.rs
        ├── update.rs
        ├── upgrade.rs
        ├── search.rs
        ├── list.rs
        └── extension.rs # CrestEditor extension lifecycle
```

---

## 📜 License

Licensed under the [Apache License, Version 2.0](LICENSE).
You may obtain a copy of the License at [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0).
