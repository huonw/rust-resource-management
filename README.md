# Rust Tutorial — CSV Summary

This tutorial walks through writing a program for summarising a CSV
file, starting from somewhere in the middle: a lot of code is provided
to focus the learning on getting familiar with the memory model and
ownership & borrowing.

## Structure

- `slides/slides.html` contains slides (compiled from
  `slides/slides.md`) that introduce and set-up some exercises
- the exercises themselves are in `src/bin`
- there are (example) solutions in `examples/`, which can be run with
  `cargo run --example <name>`
- `data/` contains some test files, including a Python (2 or 3)
  generator for large data

## Useful links

Reference:

- [Documentation for the standard
  library](https://doc.rust-lang.org/std/index.html) (search by
  pressing `s` or using the search bar at the top)
- [Rust Language Cheat Sheet](https://cheats.rs)

External packages:

- [crates.io](https://crates.io) package repository
- [docs.rs](https://docs.rs) automatic documentation hosting (for
  crates.io packages)

Learn more (resources provided by rust-lang):

- [Learn Rust summary page](https://www.rust-lang.org/learn)
- [The Rust Programming Language book](https://doc.rust-lang.org/stable/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [rustlings](https://github.com/rust-lang/rustlings/)
