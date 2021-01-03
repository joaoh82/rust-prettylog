<p align="center">
  <h1 align="center">prettylog</h1>
  <h4 align="center">Print pretty terminal log messages</h4>
  <br>
</p>

## Description:
A simple Rust crate with **no dependencies** to print colorful log messages with emojis on the terminal.

## Usage:
```rust
extern crate prettylog;

use prettylog::*;

fn main() {
    error("Hello, prettylog!");
    info("Hello, prettylog!");
    warn("Hello, prettylog!");
    wait("Hello, prettylog!");
    critical("Hello, prettylog!");
    command("Hello, prettylog!");
    link("Hello, prettylog!");
    misc("Hello, prettylog!");
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
