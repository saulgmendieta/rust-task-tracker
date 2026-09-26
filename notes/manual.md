# Rust Manual

> **How to use this manual:** each section ends with a **Test yourself** block. Try to answer from memory *before* expanding the answer. If you can't, re-read the section, not the whole course.

**Course:** Rust: The Complete Developer's Guide (Stephen Grider)
**Progress:** ✅ Setup · ✅ Core Concepts · ⬜ Ownership & Borrowing · ⬜ Lifetimes · ⬜ Enums · ⬜ Modules · ⬜ Errors · ⬜ Iterators · ⬜ Advanced Lifetimes · ⬜ Generics & Traits

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

## 1. Structs: data grouped together

A struct stores related data. Add an `impl` block and it also carries functionality. It's the closest thing Rust has to a class, but **there is no inheritance**.

```rust
struct Deck {            // Name: always PascalCase (capitalized)
    cards: Vec<String>,  // field_name: Type   (trailing comma is OK and normal)
}
```

- **Definition** = the blueprint. **Instance** = one concrete value built from it (you can make many).
- Create an instance with a **struct literal**: every field must be given a value (no defaults, no `null`).

```rust
let deck: Deck = Deck { cards: vec![] };
//  ^^^^  ^^^^   ^^^^^^^^^^^^^^^^^^^^^^
//  name  type   struct literal (vec![] = empty vector)
```

- **Field init shorthand:** if a variable has the same name as the field, `Deck { cards: cards }` → `Deck { cards }`.
- Read a field with a dot: `deck.cards`.

<details><summary>Test yourself</summary>

- Can you create a `Deck` without giving `cards` a value? → No. Every field must be set.
- What's the shorthand for `Deck { cards: cards }`? → `Deck { cards }`
</details>

---

## 2. `impl`: methods vs associated functions

An `impl` block (**inherent implementation**) attaches functions to a struct. **The first parameter decides what kind of function it is:**

```rust
impl Deck {
    fn new() -> Self { ... }         // no self → ASSOCIATED FUNCTION
    fn shuffle(&mut self) { ... }    // has self → METHOD
}

let mut deck = Deck::new();   // associated fn → called with ::
deck.shuffle();               // method        → called with .
```

| | Associated function | Method |
|---|---|---|
| First param | no `self` | `self`, `&self` or `&mut self` |
| Called with | `Type::name()` | `instance.name()` |
| Use when | work not tied to a specific instance (constructors) | reading/changing *this* instance's fields |
| Examples | `new()`, `full_deck()`, `with_n_cards(10)` | `shuffle()`, `add_card()`, `has_card()` |

**`Self` vs `self`:**
- `Self` (capital) = **the type** itself. Inside `impl Deck`, `Self` means `Deck`.
- `self` (lowercase) = **the instance** the method was called on.

**The three forms of `self`:**

| Signature | Meaning | Use for |
|---|---|---|
| `&self` | borrow to **read** | printing, checking, counting |
| `&mut self` | borrow to **modify** | shuffle, add, remove |
| `self` | **take ownership** (consume it) | rare; the instance can't be used afterwards |

> ⚠️ **Not obvious:** `new` is **not** a keyword. It's just a naming convention for constructors.
>
> ⚠️ Calling a `&mut self` method requires the variable to be `let mut`.

<details><summary>Test yourself</summary>

- `Deck::new()` vs `deck.shuffle()`: why `::` in one and `.` in the other? → `new` has no `self` (associated fn, belongs to the type); `shuffle` has `self` (method, belongs to an instance).
- What does `Self` mean inside `impl Deck`? → The type `Deck`.
- `shuffle` changes the cards. Which `self` form? → `&mut self`
</details>

---

## 3. Bindings: `let`, `mut`, shadowing

Rust calls variables **bindings**. They are **immutable by default**.

```rust
let numbers = vec![];
numbers.push(1);        // ❌ can't change the value
numbers = vec![];       // ❌ can't reassign either

let mut numbers = vec![];
numbers.push(1);        // ✅
numbers = vec![];       // ✅
```

- `mut` applies to the **whole binding**. You can't mark a single struct field as mutable.
- **Type annotations are usually optional** (`let deck: Deck = ...`). Rust infers types. The gray hints Cursor shows are these inferred types.

**Shadowing (not obvious):** you can redeclare a name with `let`, and **it's not the same as `mut`**:

```rust
let x = "5";
let x: i32 = x.parse().unwrap(); // new binding, can even change type. Old x is hidden
```

| | `mut` | shadowing (`let` again) |
|---|---|---|
| Same variable? | yes, modified in place | no, a brand-new variable |
| Can change type? | ❌ | ✅ |

- **Constants:** `const MAX_TASKS: u32 = 100;` Always immutable, type required, UPPER_SNAKE_CASE.
- **Unused variable warning?** Prefix it with `_`: `let _rng = ...`.

<details><summary>Test yourself</summary>

- `let v = vec![]; v.push(1);` Why does it fail? → `v` isn't `mut`.
- Difference between `let mut x` and writing `let x` twice? → `mut` modifies the same variable (same type); shadowing creates a new one (can change type).
</details>

---

## 4. Arrays vs Vectors (and the String gotcha)

```rust
let suits = ["Hearts", "Spades"];        // ARRAY: fixed size, type [&str; 2]
let suits = vec!["Hearts", "Spades"];    // VECTOR: can grow/shrink, type Vec<&str>
```

| | Array `[T; N]` | Vector `Vec<T>` |
|---|---|---|
| Size | fixed at compile time (it's part of the type!) | grows/shrinks at runtime |
| Create | `[1, 2, 3]` | `vec![1, 2, 3]` or `Vec::new()` |
| Use when | the list never changes (suits, card values) | the list changes (cards in a deck, tasks) |

**Rule:** *will this list change over time?* No → array. Yes → vector.

Common `Vec` operations: `.push(x)` add to end · `.pop()` remove last · `.len()` count · `.is_empty()` · `v[0]` index (panics if out of range) · `.split_off(i)` cut off everything from index `i` into a new Vec · `.shuffle(&mut rng)` (needs the `rand` crate).

### ⚠️ `&str` vs `String` (you'll hit this constantly)

- `"Hearts"` (text in quotes) is a **`&str`**: a fixed, borrowed string slice.
- **`String`** is an owned, growable string. It's what you put inside structs.
- So `vec!["a", "b"]` is `Vec<&str>`, **not** `Vec<String>`. Putting it into `cards: Vec<String>` gives a *mismatched types* error.

Convert `&str` → `String`: `"Hearts".to_string()` or `String::from("Hearts")`.
Build a String from pieces: `format!("{} of {}", value, suit)` (like `println!`, but returns the String instead of printing it).

*(Full explanation comes in the Errors section → "Strings, String Refs, and String Slices".)*

<details><summary>Test yourself</summary>

- A list of weekday names: array or vector? → Array (never changes).
- Why does `let cards: Vec<String> = vec!["Ace"];` fail? → `"Ace"` is `&str`, not `String`. Use `"Ace".to_string()`.
- How do you build `"Ace of Spades"` from two variables? → `format!("{} of {}", value, suit)`
</details>

---

## 5. Functions & implicit returns

```rust
fn is_even(num: i32) -> bool {   // parameters MUST have types; -> ReturnType
    return num % 2 == 0;          // explicit return
}

fn is_even(num: i32) -> bool {
    num % 2 == 0                  // implicit return: SAME THING. No semicolon!
}
```

**Leaving off the final semicolon = `return`.** Rust returns the **last expression** in the block.

### Why this works: expressions vs statements (the idea behind it)

- An **expression** produces a value: `5`, `a + b`, `num % 2 == 0`, `if ... { } else { }`, a `{ }` block.
- Adding `;` turns an expression into a **statement**, which throws the value away and produces `()` (the "unit" type = "nothing").
- So `num % 2 == 0;` returns `()`, and you get:
  `error: mismatched types, expected bool, found ()`. **That error almost always means "remove the last semicolon."**

**`if` is an expression too**, so it can return a value:

```rust
fn is_even(num: i32) -> bool {
    if num % 2 == 0 { true } else { false }   // last expression executed is returned
}
let label = if done { "✓" } else { " " };      // like a ternary. Both branches must be the same type
```

- **`return` still exists.** Use it for **early exits** (e.g. `if list.is_empty() { return; }`). Use implicit return for the final value.
- A function with no `->` returns `()`. `main` is one of those.
- Functions and variables use `snake_case`; types (structs) use `PascalCase`. The compiler warns if you don't.

<details><summary>Test yourself</summary>

- What does a function return if its last line ends in `;`? → `()` (unit / nothing)
- You see "expected `i32`, found `()`". First thing to check? → A semicolon on the last line.
- When would you still write `return`? → Early exit from the middle of a function.
</details>

---

## 6. Macros & printing

A `!` after a name means **macro**, not a function: code that writes code at compile time. So far: `println!`, `vec!`, `format!`.

```rust
println!("Deck: {}", x);      // {}   → Display: user-facing. Structs DON'T have it by default
println!("Deck: {:?}", deck); // {:?}  → Debug: one line
println!("Deck: {:#?}", deck);// {:#?} → Debug, pretty-printed on multiple lines
println!("{name} has {n}");   // variables can go right inside the braces
```

> ⚠️ Printing your struct with `{:?}` gives the error **"`Deck` doesn't implement `Debug`"**. Fix: add `#[derive(Debug)]` (next section).

---

## 7. Attributes & traits: `#[derive(Debug)]`

```rust
#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}
```

- `#[...]` is an **attribute**: an extra instruction to the compiler about the item right below it.
- `derive` tells the compiler: *"automatically write this code for my struct."*
- `Debug` is a **trait**: a named set of functions a type can have. The `Debug` trait lets the type be printed with `{:?}`.

In plain words: *"Hey compiler, add all the Debug functions to this struct for me."*

Common derives you'll see later: `Debug`, `Clone`, `PartialEq` (enables `==`), `Default`.

<details><summary>Test yourself</summary>

- What's a trait, in one sentence? → A named set of functions (behavior) that a type can implement.
- Why can't you `println!("{:?}", deck)` without `#[derive(Debug)]`? → The struct doesn't have the Debug trait's printing functions yet.
</details>

---

## 8. Number types

| Type | Kind | Range |
|---|---|---|
| `i8` / `i16` / `i32` / `i64` / `i128` | signed integers (±) | `i8`: -128 to 127 · `i32`: **-2,147,483,648 to 2,147,483,647** |
| `u8` / `u16` / `u32` / `u64` / `u128` | unsigned (0 and up) | `u8`: 0 to 255 · `u32`: 0 to ~4.29 billion |
| `isize` / `usize` | size of the CPU's pointer (64-bit on your PC) | `usize`: 0 to ~1.84E19 |
| `f32` / `f64` | decimals | `f64` is more precise |

**The not-obvious rules:**
- Defaults when you don't annotate: integers → **`i32`**, decimals → **`f64`**.
- **`usize` is the type for indexes and lengths.** `vec.len()` returns `usize`, and `vec[i]` needs `i: usize`. That's why `deal(num_cards: usize)`.
- **No automatic conversion**, not even `i32` → `i64`. Mixing types is an error. Convert explicitly: `x as i64`.
- Overflow (e.g. `0usize - 1`) **panics (crashes) in debug builds**. Picking the right type matters.
- `u` for things that can never be negative (ids, counts, ages).

<details><summary>Test yourself</summary>

- What type is `let x = 7;`? → `i32`
- What type do you use for a Vec index? → `usize`
- Can you add an `i32` and a `u32`? → No. Convert one with `as`.
</details>

---

## 9. Crates, modules and `use`

| | Standard library (`std`) | External crates |
|---|---|---|
| Install | nothing, included in every project | `cargo add rand` (edits Cargo.toml) |
| Find | — | **crates.io** |
| Docs | **doc.rust-lang.org/std** | **docs.rs** |

- **Crate** = a package (a library, or your program).
- **Module** = a named section inside a crate. Every crate has a **root module**, and can have **submodules** (`mod seq`, `mod rngs`...). Your project can have its own modules too (e.g. `mod games`, containing `struct Deck`).
- `::` is the **path separator**: `rand::seq::SliceRandom` = crate → module → item. (`.` is only for fields and methods.)

**Accessing code:**
- External crates are reachable directly by name: `rand::thread_rng()`.
- Your own internal modules must be declared with `mod games;` first, then used as `games::Deck::new()`.

**`use` = shortcut, not install.** It just saves typing `rand::` everywhere:

```rust
use rand::{seq::SliceRandom, thread_rng};  // several items in one line with { }
```

> ⚠️ **Not obvious #1:** `cargo add` downloads the crate; `use` only brings names into scope. You need `cargo add` once per project, and `use` in each file.
>
> ⚠️ **Not obvious #2:** **a trait's methods only work if the trait is in scope.** `vec.shuffle()` comes from the `SliceRandom` trait. Without `use rand::seq::SliceRandom;` you get *"no method named `shuffle` found"*, even though `rand` is installed.
>
> ⚠️ **Course is dated here:** `thread_rng()` was renamed **`rand::rng()`**. In `rand` 0.9 the old name only gives a warning, but in **0.10+ (what `cargo add rand` installs today) it's removed**, so the course code fails with *"no `thread_rng` in the root"*. `SliceRandom` and `shuffle` are unchanged.

<details><summary>Test yourself</summary>

- What does `use` do, and what does it NOT do? → Brings names into scope. It does NOT install anything (that's `cargo add`).
- `rand` is installed but `.shuffle()` says "method not found". Why? → The `SliceRandom` trait isn't imported with `use`.
- Where are the docs for an external crate? → docs.rs
</details>

---

## 10. Reference: the course's Deck program

```rust
use rand::{seq::SliceRandom, rng};   // course: thread_rng (old name)

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {                                // associated function (constructor)
        let suits = ["Hearts", "Spades", "Diamonds"]; // arrays: they never change
        let values = ["Ace", "Two", "Three"];
        let mut cards = vec![];                       // vector: it grows

        for suit in suits {
            for value in values {
                cards.push(format!("{} of {}", value, suit)); // &str pieces → String
            }
        }

        Deck { cards }                                // shorthand + implicit return
    }

    fn shuffle(&mut self) {                           // &mut: it changes the cards
        let mut rng = rng();
        self.cards.shuffle(&mut rng);                 // works thanks to `use SliceRandom`
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards) // implicit return
    }
}

fn main() {
    let mut deck = Deck::new();   // mut: shuffle/deal change it
    deck.shuffle();
    let cards = deck.deal(3);     // ⚠️ panics if num_cards > cards left → error handling later
    println!("Here's your hand: {:#?}", cards);
    println!("Here's your deck: {:#?}", deck);
}
```

---

## Gotchas cheat-sheet

| Error / situation | Usual cause | Fix |
|---|---|---|
| `expected X, found ()` | semicolon on the last line of a function | remove it |
| `cannot borrow as mutable` / `cannot assign twice` | binding isn't `mut` | `let mut` |
| `doesn't implement Debug` | printing a struct with `{:?}` | `#[derive(Debug)]` |
| `mismatched types: expected String, found &str` | used `"text"` where a String is needed | `.to_string()` / `String::from` |
| `no method named X found` | the trait that provides it isn't imported | `use` the trait |
| `mismatched types: i32 / usize` | numbers don't auto-convert | `as usize` or use the right type |
| `unused variable` warning | declared but never used | use it, or prefix with `_` |
| `no thread_rng in the root` | `rand` 0.10+ removed the old name | `rand::rng()` |

---

## Open questions → to answer during Ownership & Borrowing

*(Write here every error or doubt you hit while building the task tracker.)*

- Why does `shuffle` need `&mut self` and not just `self`? What happens to `deck` if it takes `self`?
- Why `&mut rng` when passing it to `shuffle`?
-
