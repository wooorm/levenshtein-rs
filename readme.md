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
use levenshtein::levenshtein;

fn main() {
    println!("{}", levenshtein("kitten", "sitting"));
}
```

Yields:

```txt
3
```

## API

### `fn levenshtein(a: &str, b: &str) -> usize`

Given two strings, returns the edit distance between them.
## Unicode text

This implementation iterates over Rust `char` values, so the edit distance is calculated over Unicode scalar values rather than UTF-8 bytes.

Canonically equivalent strings can still have different underlying scalar-value sequences. For example, a precomposed character and the same visible character represented by a base character plus a combining mark can produce different edit distances unless the inputs are normalized first.

Unicode scalar values are also not always the same as user-perceived characters. A visible character can consist of multiple scalar values, including combining sequences and emoji sequences. Applications that require one visible character to count as one edit may therefore need grapheme-cluster segmentation before calculating the distance.

For additional background on normalization, grapheme segmentation, and Unicode transformations in edit-distance processing, see [Unicode text transformations and edit distance](https://www.levenshtein.net/unicode-text-transformations).

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
