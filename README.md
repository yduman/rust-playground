# Rust Notes

## Language features

- statically typed
- block-scoped
- variables are immutable by default
- [style guidelines](https://doc.rust-lang.org/1.0.0/style/README.html)
- [naming conventions](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html)
- [the book](https://doc.rust-lang.org/book/)
- [rust by example](https://doc.rust-lang.org/stable/rust-by-example/)

## Cargo

- Cargo is the package manager of Rust
- `$ cargo init` to create a project
- `Cargo.toml` is something like `package.json` in JS
  - Lockfile is `Cargo.lock`
- `$ cargo run` builds and runs the binary
- `$ cargo build` just builds the binary but doesn't run it
- `$ cargo build --release` builds it for prod
- `$ cargo install <crate_name>` to install packages
  - if the package has specified no binaries, just include them in the TOML

## VS Code

- Extensions
  - Rust (rls)
  - Better TOML