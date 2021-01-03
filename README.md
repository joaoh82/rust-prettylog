<p align="center">
  <h1 align="center">prettylog</h1>
  <h4 align="center">Print pretty terminal messages</h4>
  <br>
</p>

## Description:
A simple Rust crate with **no dependencies** to print colorful messages with emojis on the terminal.

## Usage:
```rust
extern crate prettylog;

use prettylog::*;

fn main() {
    error("Hello, world!");
    info("Hello, world!");
    warn("Hello, world!");
    wait("Hello, world!");
    critical("Hello, world!");
    command("Hello, world!");
    link("Hello, world!");
    misc("Hello, world!");
}
```

will output

[IMAGE]

## Issue/Feedback:

log them in the [github issues](https://github.com/joaoh82/rust-prettylog/issues) or hit me on [twitter](https://twitter.com/joaoh82).

## Inspiration:

* [prettycli-node](https://github.com/siddharthkp/prettycli)

## Was this helpful?:

Please ⭐ this repo
