# Crest Tools

A modern monorepo for tools and utilities built for the Crest ecosystem.

Crest Tools brings together package-level utilities, shared libraries, and developer workflows into one clean, maintainable workspace.

## Why Crest Tools?

- Unified toolkit for Crest-related development
- Modular package structure for easier maintenance
- Shared utilities to reduce duplication across projects
- Developer-focused scripts and workflows

## Packages

| Package | Description |
| --- | --- |
| `CPM` | Crest Package Manager |
| `Crest-Git` | Git utilities |
| `Crest-Utils` | Common utilities |

## Repository Structure

```text
.
├── packages/
│   └── ... individual tool source code
├── shared/
│   └── shared libraries and reusable logic
├── scripts/
│   └── development and build scripts
├── .github/
│   └── CI/CD workflows and automation
├── README.md
├── LICENSE
└── ...
```

## Getting Started

1. Clone the repository
2. Navigate to the project root
3. Explore the package directories under `packages/`
4. Use shared utilities from `shared/`
5. Run project scripts from `scripts/`

## Development

This repo is organized as a monorepo, making it easier to:

- build and test packages independently
- share reusable functionality across tools
- keep workflows consistent across the Crest ecosystem

## Contributing

Contributions are welcome. Keep changes focused, document new utilities clearly, and follow the project convention for each package.

## License

See the individual package directories for licensing details.

---

Built for the Crest ecosystem.
