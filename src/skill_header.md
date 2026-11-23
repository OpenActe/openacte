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
