# Snake - Rust

A terminal-based Snake game written in Rust using [crossterm](https://crates.io/crates/crossterm) for terminal handling and [rand](https://crates.io/crates/rand) for apple placement.

## Running

Requires a recent stable Rust toolchain (install it with [rustup](https://rustup.rs)).

```
git clone https://github.com/EmilBjorkeng/snake-polyglot.git
cd snake-polyglot/rust
cargo run --release
```

## How it works

The game runs in the terminal's alternate screen, so your terminal is restored when you quit.
Each cell is drawn two characters wide to make it roughly square.

## Story

This is my first ever Rust project. I made it to learn the language, with a lot of help from resources online.
