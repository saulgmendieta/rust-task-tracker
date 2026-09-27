# rust-task-tracker

A small command-line task tracker written in Rust. I'm building it step by step while learning the language: each course module adds a feature that uses what that module teaches.

It's a learning project, so the code shows my progress over time rather than a finished product. My study notes are in [`notes/`](notes/README.md).

## Features

- Starts with three sample tasks, one of them already completed
- Add, complete and rename tasks from an interactive prompt
- Ids are assigned automatically
- Completed tasks are printed in green

```
Tasks (3):
[ ] 1 - Learn Rust structs
[X] 2 - Install Rust            (green)
[ ] 3 - Push project to GitHub
```

## Commands

| Command | Example | What it does |
|---|---|---|
| `add <description>` | `add Read chapter 4` | Adds a task with the next free id |
| `complete <id>` | `complete 3` | Marks the task as done |
| `rename <id>` | `rename 1` | Asks for a new description, then updates the task |
| `print` | `print` | Shows all tasks |
| `exit` | `exit` | Quits |

## Run it

Requires [Rust](https://rustup.rs) (edition 2024).

```
git clone https://github.com/saulgmendieta/rust-task-tracker.git
cd rust-task-tracker
cargo run
```

## Project structure

```
src/main.rs            the program: Task, TaskList, and the command loop
notes/                 my Rust manual, one file per course module
.cursor/rules/         instructions for the AI assistant: explain errors, don't write my code
.vscode/settings.json  rust-analyzer editor settings
```

## Design

- **`Task`** holds one task: `id`, `description`, `completed`.
- **`TaskList`** owns a `Vec<Task>` and is the only thing that changes it. It assigns ids and handles completing and renaming. `main` only reads commands and calls `TaskList` methods.
- Methods take `&self` when they only read, and `&mut self` when they change the list.

## Known limitations

These are intentional for now. Each one will be fixed in a later module:

- A non-numeric id (e.g. `complete abc`) crashes the program → *Errors & Result*
- "Not found" is signaled with `-1` instead of `Option` → *Enums & Option*
- Unknown commands are silently ignored → *Enums* (`match`)
- Tasks only live in memory and are lost on exit → *Errors* (reading/writing files)
- Everything is in one file → *Modules*

## Roadmap

| Course module | Feature it adds |
|---|---|
| ✅ Core Concepts | `Task` and `TaskList` structs, colored output |
| ✅ Ownership & Borrowing | Borrowing methods, `rename`, logic moved into `TaskList` |
| ⬜ Lifetimes | Return references to tasks (`get(id) -> &Task`) |
| ⬜ Enums & Option | `Option` instead of `-1`, a `match`-based command parser |
| ⬜ Modules | Split into `models`, `storage` and `commands` |
| ⬜ Errors & Result | Save/load tasks to a file, no crashes on bad input |
| ⬜ Iterators | Filters (pending/done) and stats |
| ⬜ Generics & Traits | A `Storage` trait with file and in-memory versions, `Display` for printing |
