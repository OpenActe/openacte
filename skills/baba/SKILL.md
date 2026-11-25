---
name: baba
description: Install Baba, the controlled-English command (baba "<request>"), when it is missing, and do nothing when it is already installed and healthy. Use when a task needs the baba command, when baba is not found on PATH, or when asked to install or set up Baba, from Claude Code, Codex or any agent that reads skills.
---

# Install Baba

Hand-written (the `openacte` skill next to this one is generated). The README
quickstart of the Baba repository is the source of these steps.

Tell the user what each step will run before running it. Stop at the first
failing step and show its output: do not guess around it, and never overwrite
an existing checkout.

## 1. Check, and stop if it is fine

```sh
command -v baba && baba doctor
```

`baba` on `PATH` and `baba doctor` ending with `healthy`: Baba is already
installed. Say so and stop. Nothing is installed, cloned or changed.

## 2. Dependencies

```sh
command -v swipl link-parser
```

Either one missing: macOS `brew install swi-prolog link-grammar`, Debian or
Ubuntu `sudo apt-get install swi-prolog link-grammar`. Native Windows is not
supported: use WSL2.

## 3. The workspace root

Baba must live at `$HOME/Source/Baba` (papis library path, docs and links
hard-code it). If that directory exists, keep it: check it holds
`Baba.code-workspace`; if it does not, stop and ask. If it is absent:

```sh
git clone git@github.com:grosjeanbaptiste/baba.git "$HOME/Source/Baba"
```

## 4. The six sub-repositories

They are independent repositories that sit inside the root. The repositories
are private except `baba-config`: a clone failing on authentication means the
user has no access. Stop and say so. Clone each missing one, skip each that
exists:

```sh
cd "$HOME/Source/Baba"
git clone git@github.com:grosjeanbaptiste/baba-engine.git engine
git clone git@github.com:grosjeanbaptiste/baba-cli.git cli
git clone git@github.com:grosjeanbaptiste/baba-actions.git actions
git clone git@github.com:grosjeanbaptiste/baba-corpus.git corpus
git clone git@github.com:grosjeanbaptiste/baba-packs.git packs
git clone git@github.com:grosjeanbaptiste/baba-config.git config
```

## 5. Bootstrap the workspace

Idempotent, safe to re-run:

```sh
cd "$HOME/Source/Baba" && cargo run
```

## 6. Build

```sh
cd "$HOME/Source/Baba/cli" && cargo build --release
cd "$HOME/Source/Baba/actions" && cargo build --release
```

These are heavy builds. On a machine with a job queue, run them through it.

## 7. Verify

```sh
baba doctor
```

It must end with `healthy`. Anything else: show the output and stop.

## Optional

`acteag` and `actinstall` are not required. Without `actinstall`,
`baba scan --install` says what is missing. For `acteag`, follow the
`openacte` skill: binary missing, `cargo install --locked --git <repository>`.
