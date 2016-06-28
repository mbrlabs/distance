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

/// Calculates the [hamming distance](https://en.wikipedia.org/wiki/Hamming_distance) between two strings of equal length.
///
/// The hamming distance is the number of positions at which two strings are different.
/// Unicode is fully supported. Returns an error if the string arguments do not have equal length.
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
pub fn hamming(a: &str, b: &str) -> Result<i32, DistanceError> {
    if a.chars().count() != b.chars().count() {
        return Err(DistanceError::InvalidArgs);
    }

    let mut distance = 0;
    for (a_char, b_char) in a.chars().zip(b.chars()) {
        if a_char != b_char { 
            distance += 1
        }
    }

    return Ok(distance);
}

#[cfg(test)]
mod tests {
    use super::hamming;

    #[test]
    fn basic() {

    }

}