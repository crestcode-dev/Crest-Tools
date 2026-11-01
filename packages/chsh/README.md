# chsh (CrestEditor Shell Changer)

A lightweight POSIX-compliant `chsh` utility designed for Alpine Linux and CrestEditor environment.

## Features
- Standard `chsh -s <shell>` command line interface.
- Automatically resolves shell names (`zsh`, `fish`, `bash`) or absolute paths (`/bin/zsh`).
- Updates `~/.config/crest/shell` for seamless integration with CrestEditor dynamic shell login system.
- Updates `/etc/passwd` and `/etc/shells` when writable.
- List installed shells with `chsh -l`.
- Interactive shell prompt when invoked without arguments.

## Usage
```sh
# Change login shell to zsh
chsh -s zsh

# Change login shell using full path
chsh -s /bin/bash

# List available shells
chsh -l

# Interactive mode
chsh
```
