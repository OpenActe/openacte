# Registry of the `act*` repositories

One file per repository: `registry/<name>.toml`. This is what a tool (Baba
first) reads to know which `act*` repositories exist and how to call them.
Facts only: a field that was not verified is absent or says `unverified`.

| Field | Meaning |
|---|---|
| `name` | Lowercase name of the tool. |
| `directory` | Folder under `~/Source/Acteble/openacte/`. |
| `repository` | HTTPS URL of the origin; absent when there is none. |
| `kind` | `cli`, `library`, `service`, `app`, `scaffold`, `workspace` or `unverified`. |
| `binary` | Executable to run; absent for a library or a scaffold. |
| `role` | First paragraph of the repository's README. |
| `readme` | Path of the README, relative to the openacte folder. |
| `usage` | The command lines the binary accepts, as its own usage prints them. Absent when not yet verified. |

`usage` is read from the source or `--help` of the binary, not guessed. The
README of each repository stays the full description; this file is the index.
