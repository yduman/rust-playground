# Rust Notes

## Language features

- system programming language
- statically-typed
- block-scoped
- variables are immutable by default
- focus on safety and performance
- ownership system

## Cargo

- Cargo is the package manager of Rust
- `$ cargo init` 
  - create a project
- `Cargo.toml` is something like `package.json`
  - Lockfile is `Cargo.lock`
- `$ cargo run` 
  - compile and run
- `$ cargo build` 
  - compile, but don't run
- `$ cargo build --release` 
  - production build
- `$ cargo install <crate_name>` 
  - install packages
  - if the package has specified no binaries, just include the dependency in the TOML

## VS Code

- Extensions
  - Rust (rls)
  - Better TOML

## Useful Links
- [Rust Crash Course | YouTube](https://www.youtube.com/watch?v=zF34dRivLOw)
- [Style Guidelines](https://doc.rust-lang.org/1.0.0/style/README.html)
- [Naming Conventions](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html)
- [The Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/)
- [Wikipedia](https://en.wikipedia.org/wiki/Rust_(programming_language))
