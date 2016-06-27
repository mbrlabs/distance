# distance
This is a collection of approximate string matching algorithms written in Rust.

## Algorithms
- [Levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance)

## Usage
```
use distance::*; 

// Levenshtein distance
let distance = levenshtein("kitten", "sitting");   
assert_eq!(distance, 3);
```