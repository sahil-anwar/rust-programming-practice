# Practice guide

Work through the numbered folders in order. Compile and run one `.rs` file at a time with `rustc`.

## Common mistakes to learn from

1. **Immutable variables:** `let x = 5; x = 6;` fails. Use `let mut x = 5;` if reassignment is needed.
2. **Ownership moved:** Passing a `String` by value may move it. Borrow with `&String` or `&str` when ownership is not needed.
3. **Borrow conflict:** A mutable borrow cannot coexist with active immutable borrows.
4. **Off-by-one loops:** `0..5` excludes 5; `0..=5` includes it.
5. **Unwrap panic:** `.unwrap()` panics on `None` or `Err`; prefer `match`, `if let`, or `?` when appropriate.
6. **String indexing:** Rust `String` does not support indexing by integer because UTF-8 characters have variable widths.

These are explanatory snippets, not intentionally broken source files in `practice/`.
