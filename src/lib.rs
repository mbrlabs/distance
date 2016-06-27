//! # A collection of approximate string matching algorithms 
//!
//! This library contains algorithms for approximate string matching.
//! These algorithms can be used to tell the approximate difference between two 
//! strings. This is usful for a varity of things like spell checking, fuzzy search, etc.
//! 
//! ## Algorithms
//! - Levenshtein distance 
//!
//! ## Literature
//! - [Approximate string matching - Wikipedia](https://en.wikipedia.org/wiki/Approximate_string_matching)
//! - [Levenshtein distance - Wikipedia](https://en.wikipedia.org/wiki/Levenshtein_distance)
//!

pub use self::levenshtein::*;

mod levenshtein;