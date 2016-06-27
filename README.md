# distance [![](https://travis-ci.org/mbrlabs/distance.svg?branch=master)](https://travis-ci.org/mbrlabs/distance)
This is a rust library for approximate string matching algorithms.

## Algorithms
- [Levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance)

## Usage
```rust
use distance::*; 

// Levenshtein distance
let distance = levenshtein("kitten", "sitting");   
assert_eq!(distance, 3);
```