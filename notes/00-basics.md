# 00 · Basics: setup, tooling & project anatomy

> **How to use this manual:** each section ends with a **Test yourself** block. Try to answer from memory *before* expanding the answer. If you can't, re-read the section, not the whole course.

## Setup (one time, Windows)

1. **C++ build tools:** Visual Studio Installer → *Desktop development with C++* (Rust needs Microsoft's linker).
2. **Rust:** install `rustup-init.exe` from rustup.rs → check with `rustc --version` and `cargo --version`.
3. **Update Rust later:** `rustup update`.
4. **Cursor:** install the **rust-analyzer** extension, and turn **Cursor Tab** (AI autocomplete) off while learning.
5. **Tutor rule:** `.cursor/rules/rust-tutor.mdc` tells the agent to explain errors and give hints, not write the code.
6. **Editor hints:** `.vscode/settings.json` keeps inferred *type* hints and hides argument-name, chaining and closing-brace hints. The gray `: i32` labels are **not code**; rust-analyzer draws them. Restart with *Ctrl+Shift+P → rust-analyzer: Restart server*.
7. **Markdown preview:** `Ctrl+K`, then `V`.

## Git routine

```
git add .
git commit -m "what I did"
git push
```

- First time on a new PC: `git config --global user.name "..."` and `git config --global user.email "..."`.
- `.gitignore` already excludes `target/`. Commit `Cargo.lock`.

---

## 0. Project anatomy & Cargo

`cargo new task-tracker` creates:

```
task-tracker/
├── src/main.rs    ← your code. fn main() is where the program starts
├── Cargo.toml     ← project settings + dependencies (like package.json)
├── Cargo.lock     ← exact installed versions. Cargo manages it. Commit it, never edit it
├── .gitignore     ← ignores target/
└── target/        ← compiled output (.exe). Rebuilt on every build, safe to delete
```

**Rule:** you work in `src/` and `Cargo.toml`. Cargo handles everything else.

| Command | What it does |
|---|---|
| `cargo new name` | Create a new project (also runs `git init`) |
| `cargo run` | Compile + run (debug build → `target/debug/`) |
| `cargo check` | Only check that it compiles. Much faster than `run`; use it constantly |
| `cargo build --release` | Optimized build → `target/release/`. Slower to compile, faster to run |
| `cargo add rand` | Add a crate to `[dependencies]` in Cargo.toml |
| `cargo fmt` | Auto-format your code to the standard style |
| `cargo clippy` | Linter: suggests more idiomatic Rust. Run before every commit |

- `edition = "2024"` in Cargo.toml is the Rust *language edition* (a set of language rules), not the compiler version.
- The first build is slow because it compiles every dependency. After that it only recompiles what changed.

<details><summary>Test yourself</summary>

- Which files do you edit, and which do you never touch? → Edit `src/*` and `Cargo.toml`. Never touch `Cargo.lock` or `target/`.
- Fastest way to know whether your code compiles? → `cargo check`
- Where is the .exe? → `target/debug/` (or `target/release/` with `--release`)
</details>

---

[Index](README.md) · [01 · Core Concepts](01-core-concepts.md) →
