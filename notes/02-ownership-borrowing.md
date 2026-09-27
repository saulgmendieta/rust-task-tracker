# 02 · Ownership & Borrowing (course module 3, "Rust 102" slides)

> **How to use this manual:** each section ends with a **Test yourself** block. Try to answer from memory *before* expanding the answer. If you can't, re-read the section, not the whole course.

## 11. Ownership & Borrowing

Ownership, borrowing and lifetimes are **three connected systems**. Together they're ~90% of Rust's difficulty, and they change how you design code.

**Why they exist:** not "to avoid a garbage collector". The real goal is to **limit the ways you can reference and change data**, so data never changes *unexpectedly* behind your back. Fewer bugs, and code you can reason about locally. Avoiding the garbage collector is a *side effect*. (Source: *without.boats — "references are like jumps"*.)

> **Rule 12, the tie-breaker:** when in doubt, remember Rust wants to **minimize unexpected updates to data**.

### The rules

| # | Rule | System |
|---|---|---|
| 1 | Every value is **owned** by a single variable, struct, vector, etc. at a time | Ownership |
| 2 | Reassigning it, passing it to a function, or putting it into a vector **moves** it. The old variable can't be used anymore | Ownership |
| 3 | You can have **many read-only references** (`&`) to a value at the same time | Borrowing |
| 4 | You can't **move** a value while a reference to it exists | Borrowing |
| 5 | You can have **one mutable reference** (`&mut`), and only if no read-only references are in use | Borrowing |
| 6 | You can't mutate a value **through its owner** while any reference to it exists | Borrowing |
| 7 | Some types are **copied** instead of moved (see "Copy vs move" below) | Borrowing |
| 8 | When the owner goes **out of scope**, the value is **dropped** (freed) | Lifetimes |
| 9 | A value can't be dropped while references to it are still active | Lifetimes |
| 10 | A reference can't **outlive** the value it points to | Lifetimes |

**Short version:** *one owner · many readers OR one writer, never both · references never outlive the owner.*

### Moves

```rust
let account = Account::new(1, String::from("me"));
let list_of_accounts = vec![account];  // value MOVED into the vector
println!("{:#?}", account);            // ❌ borrow of moved value: `account` has NO VALUE now
```

A move doesn't copy anything expensive. The **binding** just stops being valid. The value lives on in its new owner.

### Two ways to write useful code under these rules

1. **Move values back and forth by hand.** It works, but it's tedious:
   ```rust
   fn change_account(mut account: Account) -> Account { account.balance = 10; account }
   account = change_account(account);   // give it away, get it back
   ```
2. **Borrow** with references. This is what you'll use 95% of the time.

### References: `&` and `&mut`

**`&` means two different things depending on where it goes:**

| Where | Example | Meaning |
|---|---|---|
| On a **type** | `fn print_account(account: &Account)` | "this parameter must be a *reference* to an Account" |
| On a **value** | `print_account(&account)` | "create a reference to this value" |

```rust
fn print_account(account: &Account) { println!("{:#?}", account); }  // look, don't take
fn change_account(account: &mut Account) { account.balance = 10; }    // change, don't take

let mut account = Account::new(1, String::from("me"));
print_account(&account);          // many & allowed
change_account(&mut account);     // needs `let mut` on the owner too
println!("{:#?}", account);       // ✅ still the owner
```

- `&` = **read-only** reference. Many at once are fine (rule 3).
- `&mut` = **writable** reference. Only one, and no `&` at the same time (rule 5).
- **Why only one writer?** The course's car example: two cars sharing one engine is fine *as long as nobody can modify it*. If one car could change the shared engine, the other would see an unexpected change. That's the bug Rust makes impossible.

> ⚠️ **Not obvious:** a reference is only "in use" **until its last use**, not until the end of the block. This compiles:
> ```rust
> let r = &list;
> println!("{}", r.tasks.len());   // last use of r → the borrow ends here
> list.add_task(t);                // ✅ &mut is allowed now
> ```

### Copy vs move (rule 7)

These types are **copied** instead of moved. The old variable keeps working:

- All numbers (`i32`, `u32`, `f64`...), `bool`, `char`
- Arrays and tuples, **if everything inside is Copy**
- **Read-only references `&T`**

> ⚠️ **Slide correction:** the slide lists "references (both readable and writable)" as Copy. **`&mut T` is NOT Copy.** If it were, you could have two writers at once and break rule 5. Rust just re-borrows `&mut` automatically in most calls, which is why it *looks* copied.

`String`, `Vec`, and your own structs are **moved**. They own heap data, and copying them silently would be expensive. If you really want a second copy, ask for it: `.clone()`. You can also make a small struct Copy with `#[derive(Clone, Copy)]`, but only if all its fields are Copy. Clone when the result has to outlive the list or survive changes to it; borrow when I just read it right away.

Copy **task\.id** doesn't give error cause it's a copiable type (i32), instead with **task.description** (String) 

### Answers to my open questions

**Q: Why does `shuffle` need `&mut self` and not `self`? What happens to `deck` if it takes `self`?**
With `self`, the method would **take ownership** of the deck (rule 2). After `deck.shuffle()`, `deck` would be empty ("borrow of moved value"), unless `shuffle` returned the deck and you wrote `deck = deck.shuffle();`. That's the tedious Option #1. `&mut self` means *"let me change it, then give it back automatically"*.

**Q: Why `&mut rng` when passing it to `shuffle`?**
A random number generator has **internal state** that changes every time it produces a number. `shuffle` needs to *change* the rng (so `&mut`) without *keeping* it, so you can reuse `rng` afterwards. `&rng` wouldn't allow the change, and `rng` (no `&`) would move it away.

**Q: Why `for task in &self.tasks`? What happens without the `&`?**
`for` **consumes** whatever you give it. Without `&`, the loop would try to *move* the whole Vec out of `self.tasks`:
- Inside a `&self` method: ❌ `cannot move out of self.tasks which is behind a shared reference`. You only borrowed `self`, so you can't give its contents away.
- In `main` (`for task in task_list.tasks`): it compiles, but the tasks are moved out and **`task_list` can't be used after the loop**.

| Loop | `task` is | Use when |
|---|---|---|
| `for task in &list` | `&Task` | read |
| `for task in &mut list` | `&mut Task` | modify each item |
| `for task in list` | `Task` (owned) | you're done with the list and want to consume it |

<details><summary>Test yourself</summary>

- The 3-line summary of the rules? → One owner · many `&` OR one `&mut` · references never outlive the owner.
- `let b = a;` then `println!("{}", a)`: when does this work, and when does it fail? → It works if `a` is Copy (numbers, bool, char, `&T`). It fails if `a` is `String`, `Vec` or a struct (moved).
- What does `&` mean in `fn f(x: &Task)` vs in `f(&task)`? → On a type: "must be a reference". On a value: "make a reference".
- Why can't there be two `&mut` to the same value? → Two writers means unexpected updates, the exact bug class Rust eliminates.
- Is `&mut T` Copy? → No. Only `&T` is.
- Why is the ownership system NOT mainly about avoiding garbage collection? → Its goal is controlling shared mutable data. No GC is a consequence.
</details>

---

## Open questions → to answer in the Lifetimes section

- Can a function return a reference to one of my tasks (e.g. `fn get(&self, id) -> &Task`)? How long is that reference valid?
-

---

← [01 · Core Concepts](01-core-concepts.md) · [Index](README.md)
