As this is my first Rust project,
I'll be writing down absolutist code style decisions I've made
to reduce the mental load of deciding on a case by case basis.
Decisions enforced by linting are not listed.

- Always use `use`, merging statements as much as possible,
  but never use `as` aliases outside `main.rs`,
  whose only line is `use first_party::item as main`,
  qualifying with `::` in call sites instead.
- Always use from `super` in tests if available in `super`.
- Always use from `crate` rather than `self` if available in `crate`.
- `mod` and `use self::sub::item` statements are used only in `lib.rs`,
  except for `mod tests`,
  and nothing else appears in `lib.rs`.
- At most one custom error `enum` per crate, excluding those private to their modules.
- At most one public type _or_ function _or_ trait definition per file, excluding tests.
  Methods do not count.
  The name of the file is derived from the said item.
- Never use the `return` keyword (`?` is fine).
- Never use typedefs (I'm least confident in this one).
- Always use an `impl Trait` rather than a named type parameter unless there is a
  relationship that needs to be expressed with the type parameter.
