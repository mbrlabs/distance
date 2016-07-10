# distance 
[![](https://travis-ci.org/mbrlabs/distance.svg?branch=master)](https://travis-ci.org/mbrlabs/distance) 
[![](https://img.shields.io/crates/v/distance.svg)](https://crates.io/crates/distance)
[![](https://img.shields.io/badge/docs-v0.3.0-blue.svg)](https://mbrlabs.github.io/distance)

This is a rust library for approximate string matching algorithms.   
Possible applications for this are fuzzy string searching, spell checkers, spam filters, etc.

## Algorithms
All algorithms support UTF-8 encoded strings.

- [Levenshtein distance (since v0.1)](https://en.wikipedia.org/wiki/Levenshtein_distance) 
- [Hamming distance (since v0.2)](https://en.wikipedia.org/wiki/Hamming_distance)
- [Damerau Levenshtein distance (since v0.3)](https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance)

## Add as dependency
distance is available on [crates.io](https://crates.io/crates/distance).

```toml
[dependencies]
distance = "0.3"
```

## Usage
```rust
use distance::*; 

// Levenshtein distance
let distance = levenshtein("hannah", "hanna");   
assert_eq!(1, distance);

// Damerau Levenshtein distance
let distance = damerau_levenshtein("hannah", "hannha");   
assert_eq!(1, distance);

// Hamming distance
let distance = hamming("karolin", "kathrin").unwrap();   
assert_eq!(3, distance);
```