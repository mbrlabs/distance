# distance 
[![](https://travis-ci.org/mbrlabs/distance.svg?branch=master)](https://travis-ci.org/mbrlabs/distance) 
[![](https://img.shields.io/crates/v/distance.svg)](https://crates.io/crates/distance)

This is a rust library for approximate string matching algorithms.   
Possible applications for this are fuzzy string searching, spell checkers, spam filters, etc.

## Algorithms
- [Levenshtein distance (since v0.1)](https://en.wikipedia.org/wiki/Levenshtein_distance) 
- [Hamming distance (since v0.2)](https://en.wikipedia.org/wiki/Hamming_distance)

## Add as dependency
distance is available on [crates.io](https://crates.io/crates/distance).

```toml
[dependencies]
distance = "0.1"
```

## Usage
```rust
use distance::*; 

// Levenshtein distance
let distance = levenshtein("hannah", "hanna");   
assert_eq!(distance, 1);

// Hamming distance
let distance = hamming("karolin", "kathrin").unwrap();   
assert_eq!(distance, 3);
```

You can also take a look at the [documentation](https://mbrlabs.github.io/distance)