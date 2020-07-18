# line-col

[![Crates.io version shield](https://img.shields.io/crates/v/line-col)](https://crates.io/crates/line-col)
[![Docs.rs shield](https://docs.rs/line-col/badge.svg)](https://docs.rs/line-col)
[![Crates.io license shield](https://img.shields.io/crates/l/line-col)](https://github.com/TheBerkin/line-col-rs/blob/master/LICENSE)
![Crates.io downloads shield](https://img.shields.io/crates/d/line-col)

A simple crate for calculating 1-based line/col numbers for a string slice.

## Example

```rust
use line_col::LineColLookup;

let text = "One\nTwo";
let lookup = LineColLookup::new(text);
assert_eq!(lookup.get(0), (1, 1)); // 'O' (line 1, col 1)
assert_eq!(lookup.get(1), (1, 2)); // 'n' (line 1, col 2)
assert_eq!(lookup.get(2), (1, 3)); // 'e' (line 1, col 3)
assert_eq!(lookup.get(4), (2, 1)); // 'T' (line 2, col 1)
assert_eq!(lookup.get(5), (2, 2)); // 'w' (line 2, col 2)
assert_eq!(lookup.get(6), (2, 3)); // 'o' (line 2, col 3)
assert_eq!(lookup.get(7), (2, 4)); // <end> (line 2, col 4)
```

## Column calculation methods

This crate offers two methods for calculating the column number.

The `LineColLookup::get` method counts the number of bytes from the start of the line (plus one).
However, this does not account for grapheme clusters (e.g. complex accented characters, emoji, etc.)
If you would like to calculate the column based on the number of grapheme clusters instead, enable
the `grapheme-clusters` feature and use `LineColLookup::get_by_cluster`.

## Changelog

### 0.2.0

* Move cluster-specific `LineColLookup::get` implementation into its own method, `LineColLookup::get_by_cluster`
* Remove unnecessary feature filters on some tests

### 0.1.1

* Fixed documentation typos
* Use extended grapheme clustering

### 0.1.0

Initial version