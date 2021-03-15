# levenshtein.rs

[![Build][build-badge]][build]
[![Coverage][coverage-badge]][coverage]
[![Crate][crate-badge]][crate]

Vladimir Levenshtein’s [edit distance algorithm][wiki] as a Rust
library. There’s also a [C library][c-api], [C user command][c-cli],
and [JavaScript module][js-api].

> :tada: This is my first attempt at Rust!

## Installation

[Cargo][]:

```toml
[dependencies]
levenshtein = "1.0.5"
```

## Usage

```rust
extern crate levenshtein;
use levenshtein::{closest, distance};

fn main() {
    println!("{}", distance("kitten", "sitting"));
    println!(
        "{}",
        closest("levenshtein", &["frankenstein", "einstein"]).unwrap()
    );
}
```

Yields:

```txt
3
einstein
```

## API

### `fn distance(a: &str, b: &str) -> usize`
Given two strings, returns the edit distance between them.

### `fn closest<'a>(a: &str, pool: &['a &str]) -> Option<&'a str>`
Given one string and an array of strings, returns the first closest string from the pool.

## License

[MIT][license] © [Titus Wormer][author]

<!-- Definitions -->

[build-badge]: https://github.com/wooorm/levenshtein-rs/workflows/main/badge.svg

[build]: https://github.com/wooorm/levenshtein-rs/actions

[coverage-badge]: https://img.shields.io/codecov/c/github/wooorm/levenshtein-rs.svg

[coverage]: https://codecov.io/github/wooorm/levenshtein-rs

[crate-badge]: https://img.shields.io/crates/v/levenshtein.svg

[crate]: https://crates.io/crates/levenshtein

[license]: license

[author]: https://wooorm.com

[cargo]: https://crates.io

[wiki]: https://en.wikipedia.org/wiki/Levenshtein_distance

[c-cli]: https://github.com/wooorm/levenshtein

[c-api]: https://github.com/wooorm/levenshtein.c

[js-api]: https://github.com/words/levenshtein-edit-distance
