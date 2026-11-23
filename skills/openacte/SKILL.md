---
name: openacte
description: Catalogue of the OpenActe repositories (actrepo, actos, actinstall, actesskey and the rest) with the role, binary and address of each. Use when a task names an act* tool or repository, asks which OpenActe tool does something, or needs to clone, install or read the docs of an OpenActe repository, from Claude Code, Codex or any agent that reads skills.
---

# OpenActe

GENERATED from `registry/*.toml` by `tools/generate_skill.py`: do not edit by
hand; change the registry and regenerate.

## How to use a repository

1. Pick the repository below by its role.
2. Read its `CONTEXT.md`, then its `AGENTS.md` or `CLAUDE.md`, in the checkout or on GitHub, before changing anything. They hold the current state and the rules of that repository.
3. No checkout: `git clone <repository>`.
4. Binary not installed: `cargo install --locked --git <repository>`; with a checkout, `actinstall update` keeps installed binaries current.
5. Each tool lists its commands with `<binary> --help`: trust that over this page.

## Repositories

### actautoma (cli)

Rust library that handles the tools of repository-hosting platforms (`gh` for GitHub, `glab` for GitLab, and others). The `gh` adapter exists; this README states what exists today and grows with the API.

- repository: https://github.com/grosjeanbaptiste/actautoma
- binary: `actautoma`
- usage: `actautoma rename-repo OWNER/OLD NEW [--dir OLD_DIR] [--root DIR]... [--children DIR]... [--registry FILE] [--bin-dir DIR] [--into DIR] [--local-only] [--dry-run]`
- directory: `actautoma`

### actcast (cli)

Actcast est l'orchestrateur de la stack multimédia et de téléchargement de l'écosystème Acteble.

- repository: no remote registered
- binary: `actcast`
- usage: `actcast status|start|stop [--manifest <file>] [service...]`
- usage: `actcast sync --config <file>`
- usage: `actcast index [--dotfiles <binary>] [--out <directory>]`
- usage: `actcast serve --bind <address> --config <file>`
- directory: `actcast`

### actconf (cli)

Manages the configuration of a fleet: machines, volumes and servers. Today it only lists the Stow packages of a machine.

- repository: https://github.com/grosjeanbaptiste/actconf
- binary: `actconf`
- usage: `actconf packages`
- directory: `actconf`

### actempo (cli)

Scheduler of acts for agentic sessions: given acts (units of agent work with dependencies, priority, deadline, release time, allowed providers, token cost) and sessions (each of a provider, one act at a time, providers with a token budget), it returns a plan: which act runs on which session, when, and which acts could not be placed and why. Nothing is dropped silently.

- repository: https://github.com/grosjeanbaptiste/actempo
- binary: `actempo`
- usage: `actempo lock RESOURCE... --owner NAME [--shared] [--locks DIR]`
- usage: `actempo unlock --owner NAME [--locks DIR]`
- usage: `actempo locks [--locks DIR]`
- usage: `actempo run --acts FILE --providers FILE [--tick-ms N] [--locks DIR] [--budget] [--live-sessions] ...`
- directory: `actempo`

### actesskey (cli)

Secrets, keys and passwords of the machine, in one place. It is replacing the secret handling of `dotfiles`.

- repository: https://github.com/grosjeanbaptiste/actesskey
- binary: `actesskey`
- usage: `actesskey init | remove <name> | verify | recipient add <age1…> [--label L] | recipient list | recipient remove <age1…> | rekey | backup-key <file> | restore-key <file> | github <name> <owner/repo> [--as SECRET] | push [name] | target <name> <path> [--mode M] | apply [name] | add <name> [--replace] | add-pair <name> p12|ssh <file> [--replace] | import <dir> --identity <file> | get <name> | list [--age] [--expiring]`
- directory: `actesskey`

### actinstall (cli)

Keeps the cargo-installed binaries of local repositories at their repository's latest version.

- repository: https://github.com/grosjeanbaptiste/actinstall
- binary: `actinstall`
- usage: `actinstall update`
- usage: `actinstall list`
- directory: `actinstall`

### actit (cli)

Does a tool already exist for this need? Actit answers from indexer (the catalogue of packages) and kag (reads their documentation, cites sources). It uses no other search engine and no LLM.

- repository: https://github.com/grosjeanbaptiste/Actit
- binary: `actit`
- usage: `actit --name <repo> [--owner <owner> | --contains]`
- usage: `actit "<need>"`
- directory: `Actit`

### actlimit (cli)

Shows the usage limits of the coding agents (Claude, Codex, and the agents with a token budget) as one line each, fast enough for a shell prompt.

- repository: https://github.com/grosjeanbaptiste/actlimit
- binary: `actlimit`
- usage: `actlimit refresh [agent...]`
- usage: `actlimit show [agent...]`
- usage: `actlimit prompt [agent...]`
- usage: `--live keeps only the agents with a process running now`
- directory: `actlimit`

### actmail (cli)

Read-only search over every mailbox listed in the `dotfiles mail` manifest, over IMAP or from the archived Maildirs.

- repository: https://github.com/grosjeanbaptiste/actmail
- binary: `actmail`
- usage: `actmail --since YYYY-MM-DD [account...] [--subject TEXT [--save DIR] [--export DIR]]`
- directory: `actmail`

### actmorph (cli)

Actmorph is a Rust project for managing files of every type. Generic catalog and file operations do not depend on media or extension recognition; format-aware metadata is an optional enrichment.

- repository: https://github.com/grosjeanbaptiste/Actmorph
- binary: `actmorph`
- usage: `actmorph scan <library>`
- usage: `actmorph list <library>`
- usage: `actmorph search <library> <text>`
- usage: `actmorph copy <source> <destination>`
- usage: `actmorph archive create <file-or-directory> <archive.zip|tar|7z> | verify <source> <archive> | extract <archive.zip|tar|7z|rar> <new-directory> | remove-verified-source <file-or-directory> <archive> --yes`
- usage: `actmorph move|rename <source> <destination>`
- directory: `Actmorph`

### actname (cli)

Actname is a deterministic, offline-first semantic naming engine written in Rust.

- repository: https://github.com/grosjeanbaptiste/Actname
- binary: `actname`
- usage: `actname generate --prefix <p>... [--domain "<words>"] [--readme <file>] [--max-length n] [--limit n] [--strategy hybrid|composed|coined] [--exclude <name>]... [--format text|json]`
- usage: `actname --help`
- directory: `Actname`

### actos (cli)

`actos` is one app to administer a machine: its hardware components, its inputs and outputs (network and physical ports), and its bytes (files, archives, duplicates — handled with the dedicated

- repository: https://github.com/grosjeanbaptiste/actos
- binary: `actos`
- usage: `actos <command>; commands listed by `actos --help``
- directory: `actos`

### actparc (cli)

Manages the configuration of a fleet: machines, volumes and servers. Today it only lists the Stow packages of a machine.

- repository: https://github.com/grosjeanbaptiste/actparc
- binary: `actparc`
- usage: `actparc packages`
- directory: `actparc`

### actqueue (cli)

Manages queues of jobs (the heavy queue and its lanes), working in pair with actempo.

- repository: https://github.com/grosjeanbaptiste/actqueue
- binary: `actqueue`
- usage: `actqueue report | state | run [--quick|--medium] [--name <label>] [--wait-max <seconds>] -- <command> [args...]`
- directory: `actqueue`

### actrepo (cli)

Manage git repositories by reusing what exists: scaffolding, checking, renaming, workspaces, pull requests, install.

- repository: https://github.com/grosjeanbaptiste/actrepo
- binary: `actrepo`
- usage: `actrepo <command>; commands listed by `actrepo --help``
- directory: `actrepo`

### actsessions (cli)

Gestion et suivi unifiés des sessions d'agents IA (Claude Code, Codex, Antigravity, Aider, etc.) sur une machine locale.

- repository: https://github.com/grosjeanbaptiste/actsessions
- binary: `actsessions`
- usage: `actsessions list [--json]`
- usage: `actsessions live`
- usage: `actsessions inspect <session-id> [--json]`
- usage: `actsessions orphans`
- usage: `actsessions clean [--kill]`
- usage: `actsessions stop <session-id>`
- directory: `actsessions`

### acttex (cli)

Build a LaTeX project: find its main file, run pdflatex / biber / makeglossaries.

- repository: https://github.com/grosjeanbaptiste/acttex
- binary: `acttex`
- usage: `acttex <initial|compile|rebuild> [main.tex]`
- directory: `acttex`

### actusext (cli)

Usage limits of every coding agent (Claude Code, Codex, Gemini, ...): what is left of the week, and a plan that fits it.

- repository: https://github.com/grosjeanbaptiste/actusext
- binary: `actusext`
- usage: `actusext codex`
- usage: `actusext claude`
- usage: `actusext <agent> profile [--days N]`
- usage: `actusext <agent> daily|models|projects|sessions [--days N]`
- usage: `actusext claude guard [--days N] [--threshold TOKENS] [--min-requests N]`
- usage: `actusext line <agent>`
- usage: `actusext recommend <project> [--days N]`
- usage: `actusext plan --forecast FILE --roadmap FILE --today YYYY-MM-DD`
- directory: `actusext`

### baba (cli)

A shell-like CLI you talk to in (controlled) English, powered by pure symbolic AI, with no LLM in the loop.

- repository: https://github.com/grosjeanbaptiste/baba
- binary: `baba`
- usage: `baba "<request in controlled English>"`
- directory: `baba`

### openacte (cli)

Generate the OpenActe agent skill from registry/*.toml: the catalogue of every repository in this folder.

- repository: https://github.com/OpenActe/openacte
- binary: `openacte-skill`
- usage: `openacte-skill`
- usage: `openacte-skill --check`
- directory: `openacte`

### actart (service)

Legal reasoning engine, Belgian law first: from a need or a file, find the applicable texts, reason about how they apply, answer with citations. Orchestrates specialised repositories (one per legal domain), consumed by tag or rev.

- repository: https://github.com/grosjeanbaptiste/actart
- binary: `actart-api`
- directory: `actart`

### actedu (service)

Cartographie unifiée des cursus d'enseignement supérieur et du graphe des savoirs académiques.

- repository: https://github.com/grosjeanbaptiste/actedu
- directory: `actedu`

### actindexer (service)

Index of external data sources (knowledge bases, event providers, third-party services, datasets) and of chosen books and software packages, in one format, for any project. One TOML file per source, validated by Rust, rendered to JSON and served as a public read API at https://indexer.acteble.workers.dev.

- repository: https://github.com/grosjeanbaptiste/actindexer
- binary: `actindexer`
- directory: `actindexer`

### actserve (service)

Actserve est un orchestrateur universel en Rust conçu pour l'administration intégrale de serveurs. Il unifie l'ensemble de la chaîne de valeur système et cloud : noms de domaine (Registrars), zones et enregistrements DNS, délivrance de certificats TLS/SSL Let's Encrypt, tunnels VPN WireGuard, moteurs d'automatisation déclarative (OpenTofu, Ansible, Crossplane), et réutilisation maximale de dépôts d'administration existants (Ansible Galaxy, LinuxServer.io, collections GitHub Sysadmin & Homelab).

- repository: https://github.com/grosjeanbaptiste/actserve
- binary: `actserve`
- directory: `actserve`

### actserve-ui (app)

Administration interface of [actserve](../actserve): servers, domains and DNS, WireGuard, orchestrations and workflows, ecosystem presets. A Dioxus app, one crate for web, desktop, tablet and phone.

- repository: https://github.com/grosjeanbaptiste/actserve-ui
- binary: `actserve-ui`
- directory: `actserve-ui`

### actessh (library)

SSH client. Keys and passwords come from actesskey.

- repository: https://github.com/grosjeanbaptiste/actessh
- directory: `actessh`

### actmake (library)

Turn a project into its artifact. One verb, `build`; the engine is chosen by the caller.

- repository: https://github.com/grosjeanbaptiste/actmake
- usage: `actmake::build(&System, dir, Engine, &Options)`
- directory: `actmake`

### actpath (library)

Everything about paths: rewriting the references to a renamed repository, moves, worktree repair, cargo install records (ADR 0001). Today only the first is built.

- repository: https://github.com/grosjeanbaptiste/actpath
- usage: `actpath::realign::{for_repo, replace, replace_words, encoded}`
- directory: `actpath`

### actcdi (scaffold)

Executes CI/CD: runs the gates (lint, tests, builds), shards them and selects what to run, for every repository.

- repository: https://github.com/grosjeanbaptiste/actcdi
- directory: `actcdi`

### actstack (scaffold)

technology stack of a project: databases, containers, services (intent, nothing built yet)

- repository: https://github.com/grosjeanbaptiste/actstack
- directory: `actstack`

### acteag (workspace)

A purely algorithmic, domain-agnostic framework for benchmarking Retrieval-Augmented Generation pipelines, organized around a taxonomy of interchangeable components. Every pipeline stage and every evaluation metric is one algorithm among a family, selected by label and swappable without touching the rest.

- repository: https://github.com/grosjeanbaptiste/acteag
- binary: `acteag`
- directory: `acteag`

### actrack (workspace)

Workspace parent du projet Actrack — prédiction de la Coupe du Monde FIFA 2026. Il regroupe des sous-projets qui sont chacun leur propre dépôt git (non imbriqués : ce dépôt racine les ignore via `.gitignore`).

- repository: https://github.com/grosjeanbaptiste/actrack
- directory: `actrack`

### actdevice (unverified)

Run or install an app (Flutter, Dioxus, or any project with a `scripts/device-install.sh` hook) on an iOS simulator, an Android device or the host.

- repository: https://github.com/grosjeanbaptiste/actdevice
- directory: `actdevice`
