# distance 
[![](https://travis-ci.org/mbrlabs/distance.svg?branch=master)](https://travis-ci.org/mbrlabs/distance) 
[![](https://img.shields.io/crates/v/distance.svg)](https://crates.io/crates/distance)

This is a rust library for approximate string matching algorithms.   
Possible applications for this are fuzzy string searching, spell checkers, spam filters, etc.

## Algorithms
- [Levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance)

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
let distance = levenshtein("kitten", "sitting");   
assert_eq!(distance, 3);
```

You can also take a look at the [documentation](https://mbrlabs.github.io/distance)