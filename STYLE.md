As this is my first Rust project,
I'll be writing down absolutist code style decisions I've made
to reduce mental load of deciding on a case by case basis.
Decisions enforced by linting are not listed.

- Always use `use`, merging statements as much as possible, but never use `as`,
  qualifying with `::` in call sites instead.
- Always use from `super` in tests if available in `super`.
- All first-party crates are flat (no subdirectories)
  and reexport public and private items from its modules,
  meaning that only `use self::sub::item` is used in `lib.rs` and `main.rs`,
  and only `use first_party::item` is used elsewhere.
- At most one custom error `enum` per crate, excluding those private to their modules.
- Exactly one public type _or_ function _or_ trait definition per file.
- Never use an `enum` (possible wrapped, e.g., in `Result`)
  as a return type if some variants never occur.
- Never use the `return` keyword (`?` is fine).
- Never use typedefs (I'm least confident in this one).
