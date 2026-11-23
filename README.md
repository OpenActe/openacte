# OpenActe

**Open-source software engineering, modular systems and digital autonomy.**

OpenActe is an open-source organization dedicated to the research, design, development and maintenance of reliable, modular and interoperable software systems.

Our mission is to build reusable technological foundations that promote transparency, decentralization, interoperability and long-term sustainability.

## Philosophy

We believe software should be:

- **Open** — Transparent, auditable and accessible.
- **Modular** — Composed of independent, reusable components.
- **Interoperable** — Built on open standards and explicit interfaces.
- **Reliable** — Tested, reproducible and predictable.
- **Secure** — Designed around explicit trust boundaries and least privilege.
- **Decentralized** — Minimizing unnecessary dependencies on centralized infrastructure.
- **Sustainable** — Maintainable, extensible and technologically independent.

## Engineering Principles

Our engineering practices emphasize:

- Domain-Driven Design (DDD)
- Test-Driven Development (TDD)
- Command Query Responsibility Segregation (CQRS)
- SOLID Principles
- Clean Code
- Clean Architecture
- Hexagonal Architecture
- Onion Architecture
- Explicit Architecture

Architectural patterns are adopted according to project requirements rather than imposed universally.

## Research & Development

OpenActe explores several complementary domains:

| Domain | Areas of Interest |
|---|---|
| Systems Engineering | Operating systems, processes, memory, storage and hardware |
| Infrastructure | Deployment, orchestration, networking and automation |
| Artificial Intelligence | Autonomous agents, knowledge representation and reasoning |
| Data Engineering | Processing, indexing, retrieval and transformation |
| Distributed Systems | Peer-to-peer networking, synchronization and distributed computing |
| Developer Tools | CLI applications, automation, testing and development environments |
| Applied Mathematics | Graph theory, optimization, signal processing and computational modeling |
| Software Architecture | Modular systems, interoperability and architectural patterns |

## Technology Stack

Our preferred technologies include:

- **Rust** — Performance, memory safety and systems programming.
- **Go** — Concurrent services and infrastructure tooling.
- **Python** — Artificial intelligence, scientific computing and automation.
- **TypeScript** — Web applications and developer tooling.
- **Docker** — Reproducible and isolated environments.

Technology choices remain driven by the requirements of each project.

## Development Standards

OpenActe projects aim to provide:

1. Clearly defined responsibilities and architectural boundaries.
2. Explicit and documented interfaces.
3. Automated unit, integration and acceptance tests.
4. Reproducible development and deployment environments.
5. Continuous integration and automated quality checks.
6. Dependency management and security verification.
7. Semantic versioning where applicable.
8. Documented architectural decisions.
9. Clear contribution guidelines.
10. Explicit licensing and attribution.

## Open Source & Licensing

OpenActe promotes open-source development and responsible software reuse.

**Apache License 2.0 is the recommended default license** for software repositories maintained by the organization.

Depending on individual project requirements, alternative licenses may be selected:

| License | Intended Use |
|---|---|
| Apache 2.0 | General-purpose libraries, frameworks and infrastructure |
| MPL 2.0 | Components requiring file-level copyleft |
| LGPL 3.0 | Libraries requiring limited copyleft |
| GPL 3.0 | Software requiring strong copyleft upon distribution |
| AGPL 3.0 | Network services requiring source-sharing obligations |
| MIT | Small utilities intended for permissive reuse |

Each repository defines its own licensing terms in its `LICENSE` file.

## Contributing

Contributions are welcome through issues, discussions and pull requests.

Before contributing:

1. Review the repository documentation.
2. Follow its architecture and coding conventions.
3. Include appropriate automated tests.
4. Ensure existing tests and quality checks pass.
5. Document significant behavioral or architectural changes.

Project-specific contribution requirements are defined in each repository.

## Vision

**Build open foundations. Enable independent innovation.**

OpenActe aims to develop an ecosystem of independent, composable and interoperable software components that can serve as foundations for increasingly complex technological systems.

Our long-term objective is to advance digital autonomy through open engineering, collaborative research and sustainable software development.

---

**OpenActe — Open foundations for autonomous systems.**

---

# openacte

## Repositories

Each tool has one file in [`registry/`](registry/): binary, role, command
lines. The format is in [`registry/README.md`](registry/README.md).

| Tool | Kind | Role |
|---|---|---|
| [`actart`](registry/actart.toml) | service | Legal reasoning engine, Belgian law first: from a need or a file, find the applicable text |
| [`actautoma`](registry/actautoma.toml) | cli | Rust library that handles the tools of repository-hosting platforms (`gh` for GitHub, `gla |
| [`actcast`](registry/actcast.toml) | cli | Actcast est l'orchestrateur de la stack multimédia et de téléchargement de l'écosystème Ac |
| [`actcdi`](registry/actcdi.toml) | scaffold | Executes CI/CD: runs the gates (lint, tests, builds), shards them and selects what to run, |
| [`actdevice`](registry/actdevice.toml) | unverified | Run or install an app (Flutter, Dioxus, or any project with a `scripts/device-install.sh`  |
| [`actconf`](registry/actconf.toml) | cli | Manages the configuration of a fleet: machines, volumes and servers. Today it only lists the Stow packages of a machine. |
| [`actdock`](registry/actdock.toml) | scaffold | unknown: the README and CONTEXT.md carry no description yet |
| [`actedu`](registry/actedu.toml) | service | Cartographie unifiée des cursus d'enseignement supérieur et du graphe des savoirs académiq |
| [`actempo`](registry/actempo.toml) | cli | Scheduler of acts for agentic sessions: given acts (units of agent work with dependencies, |
| [`baba`](registry/baba.toml) | cli | A shell-like CLI you talk to in (controlled) English, powered by pure symbolic AI, with no |
| [`acteag`](registry/acteag.toml) | workspace | A purely algorithmic, domain-agnostic framework for benchmarking Retrieval-Augmented Gener |
| [`actesskey`](registry/actesskey.toml) | cli | Secrets, keys and passwords of the machine, in one place. It is replacing the secret handling |
| [`actindexer`](registry/actindexer.toml) | service | Index of external data sources and of chosen books and software packages, one TOML file pe |
| [`actessh`](registry/actessh.toml) | library | SSH client. Keys and passwords come from actesskey. |
| [`actinstall`](registry/actinstall.toml) | cli | Keeps the cargo-installed binaries of local repositories at their repository's latest vers |
| [`actit`](registry/actit.toml) | cli | Does a tool already exist for this need? Actit answers from indexer (the catalogue of pack |
| [`actlimit`](registry/actlimit.toml) | cli | Shows the usage limits of the coding agents (Claude, Codex, and the agents with a token bu |
| [`actmail`](registry/actmail.toml) | cli | Read-only search over every mailbox listed in the `dotfiles mail` manifest, over IMAP or from the archived Maildirs. |
| [`actmake`](registry/actmake.toml) | library | Turn a project into its artifact. One verb, `build`; the engine is chosen by the caller. |
| [`actmorph`](registry/actmorph.toml) | cli | Actmorph is a Rust project for managing files of every type. Generic catalog and file oper |
| [`actname`](registry/actname.toml) | cli | Actname is a deterministic, offline-first semantic naming engine written in Rust. |
| [`actos`](registry/actos.toml) | cli | `actos` is one app to administer a machine: its hardware components, its inputs and output |
| [`actparc`](registry/actparc.toml) | cli | Manages the configuration of a fleet: machines, volumes and servers. Today it only lists t |
| [`actpath`](registry/actpath.toml) | library | Everything about paths: rewriting the references to a renamed repository, moves, worktree  |
| [`actqueue`](registry/actqueue.toml) | cli | Manages queues of jobs (the heavy queue and its lanes), working in pair with actempo. |
| [`actrack`](registry/actrack.toml) | workspace | Workspace parent du projet Actrack — prédiction de la Coupe du Monde FIFA 2026. Il regroup |
| [`actrepo`](registry/actrepo.toml) | cli | Manage git repositories by reusing what exists: scaffolding, checking, renaming, workspac |
| [`actserve-ui`](registry/actserve-ui.toml) | app | Administration interface of [actserve](../actserve): servers, domains and DNS, WireGuard, o |
| [`actserve`](registry/actserve.toml) | service | Actserve est un orchestrateur universel en Rust conçu pour l'administration intégrale de s |
| [`actsessions`](registry/actsessions.toml) | cli | Gestion et suivi unifiés des sessions d'agents IA (Claude Code, Codex, Antigravity, Aider, |
| [`actusext`](registry/actusext.toml) | cli | Usage limits of every coding agent (Claude Code, Codex, Gemini, ...): what is left of the  |
| [`openacte`](registry/openacte.toml) | cli | Generate the OpenActe agent skill from registry/*.toml: the catalogue of every repository  |
| [`acttex`](registry/acttex.toml) | cli | Build a LaTeX project: find its main file, run pdflatex / biber / makeglossaries. |

## Use the repositories from an agent

`skills/openacte/SKILL.md` is a skill (Agent Skills format) that lists every
repository of the registry with its role, binary and address. Claude Code,
Codex and any agent that reads skills can load it.

- Claude Code: `/plugin marketplace add OpenActe/openacte`, then
  `/plugin install openacte@openacte`.
- Other agents: copy or link `skills/openacte/` into the agent's skills folder.
- The skill is generated from `registry/*.toml`: after changing the registry run
  `cargo run`; CI runs `cargo run -- --check`.
