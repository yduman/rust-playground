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

## Useful Links
- [Rust Crash Course | YouTube](https://www.youtube.com/watch?v=zF34dRivLOw)
- [Syle Guidelines](https://doc.rust-lang.org/1.0.0/style/README.html)
- [Naming Conventions](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html)
- [The Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/)
- [Wikipedia](https://en.wikipedia.org/wiki/Rust_(programming_language))