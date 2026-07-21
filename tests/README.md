# bats tests

In this project (so far) I don't use Rust's built-in test features. Instead it uses
[Bats](https://bats-core.readthedocs.io/en/stable/). For some reasons:

- Historical: I originally wrote this program in Nushell, and many of the tests are
  leftover from those days. Nushell didn't have a great testing story at the time,
  so Bats was the next best thing. I decided to keep the tests alive during the Rust
  rewrite.
- Bats allows me to test the whole system end-to-end. I prefer end-to-end tests first,
  and finer-grained tests only when it makes sense. This project has such a small scope
  that I haven't found a compelling need for finer-grained tests yet.
- I appreciate the additional confidence that comes with writing test scenarios that an
  end-user would actually reproduce in the terminal.
