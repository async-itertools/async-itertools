# Contributing to `async-itertools`

## Understanding what this project is

In short: <https://docs.rs/itertools> but for <https://docs.rs/futures-lite>.

- Why `futures-lite` not `futures-util`?
  - `itertools`+`futures-util` should be `futures-itertools`.
  - `smol`-adjacent projects are generally `async-` named.
  - There is a separate `futures-util`-based project that @parrrate maintains, `ruchei-itertools`,
    and maintaining *three* separate crates would be a bit too much for now.
- What do we include?
  - Things that are in `itertools` or `core`'s `Iterator` *plus* some of their `try_` variants,
  - ... and are meaningful *(clarify?)* to implement for `Stream`s,
  - ... and aren't yet present in `futures-lite`.
- What do we not include?
  - New functionality that isn't yet present for iterators.
  - Variants of existing adapters that are closely tied to scheduling. For example, `zip_lazy` that
    polls the second stream only after it got an item from the first.

## Submitting code

- If it implements new functionality, an issue MUST be opened first.
- New functionality MUST be accompanied by tests demonstrating it.
- Bug fixes SHOULD include unit tests, even for simple mistakes in code.
- Performance fixes SHOULD include benchmark code with, preferably, details of what host that was
  ran on.
- Add an entry to `CHANGELOG.md`'s `[Unreleased]` section.
- Code with comments is rejected by closing a PR. Remove comments and open a new PR.
  - Blatantly AI-written code is rejected by closing a PR without a response. ***Don't comment on
    others' code looking too AI-like, just ignore such PRs.*** Such comments will be marked as
    off-topic.
  - `unsafe` code is rejected. If you need an `unsafe` abstraction (for example, you want to make
    `option-entry`), put it in a separate crate.
  - Assert invariants through code instead of assuming them in comments.
