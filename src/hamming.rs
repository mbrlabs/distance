// Copyright (c) 2016. See AUTHORS file.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::DistanceError;

/// Calculates the Hamming distance between two strings of equal length.
///
/// # Hamming distance
/// The [Hamming distance](https://en.wikipedia.org/wiki/Hamming_distance) between two strings of equal length is the number of positions at 
/// which two strings are different.
/// This returns an error of type DistanceError::InvalidArgs if the string arguments do not have equal length.
/// Unicode is fully supported.
///
/// ## Examples
/// ```
/// use distance::*;
/// 
/// // Hamming distance
/// let distance = hamming("karolin", "kathrin").unwrap();   
/// assert_eq!(distance, 3);
/// ```
///
pub fn hamming(s: &str, t: &str) -> Result<usize, DistanceError> {
    if s.chars().count() != t.chars().count() {
        return Err(DistanceError::InvalidArgs);
    }

    let mut distance = 0;
    for (s_char, t_char) in s.chars().zip(t.chars()) {
        if s_char != t_char { 
            distance += 1
        }
    }

    return Ok(distance);
}

#[cfg(test)]
mod tests {
    use super::hamming;
    use ::DistanceError;

    #[test]
    fn basic() {
        assert_eq!(hamming("sitting", "sitting").unwrap(), 0);
        assert_eq!(hamming("abcdefg", "hijklmn").unwrap(), 7);
        assert_eq!(hamming("karolin", "kathrin").unwrap(), 3);
        assert_eq!(hamming("hello", "world").unwrap(), 4);
        assert_eq!(hamming("Rust", "rust").unwrap(), 1);
    }

    #[test]
    #[allow(unused_variables)]
    fn matching() {
        match hamming("karolin", "kathrin") {
            Ok(distance) => assert_eq!(distance, 3),
            Err(err) => panic!("This should not happen"), 
        };

        match hamming("abra", "kadabra") {
            Ok(distance) => panic!("These are not valid inputs"),
            Err(err) => assert_eq!(err, DistanceError::InvalidArgs),
        };
    }

    #[test]
    #[should_panic]
    fn different_len() {
        hamming("abra", "kadabra").unwrap();
    }

    #[test]
    fn empty() {
        assert_eq!(hamming("", "").unwrap(), 0);
    }

    #[test]
    fn unicode() {
        assert_eq!(hamming("さようなら", "さようなは").unwrap(), 1);
        assert_eq!(hamming("säge", "sage").unwrap(), 1);
        assert_eq!(hamming("ßäöüé", "ößäüè").unwrap(), 4);
    }

    #[test]
    #[should_panic]
    fn unicode_different_len() {
        hamming("さようならa", "さようなら").unwrap();
    }

}