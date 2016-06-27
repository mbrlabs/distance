use std::cmp;

pub fn levenshtein(a: &str, b: &str) -> i32 {
    // get length of unicode chars
    let len_a = a.chars().count();
    let len_b = b.chars().count();

    // initialize matrix
    let mut mat: Vec<Vec<i32>> = vec![vec![0; len_b + 1]; len_a + 1];
    for i in 1..(len_a + 1) { 
        mat[i][0] = i as i32; 
    }
    for i in 1..(len_b + 1) { 
        mat[0][i] = i as i32; 
    }

    // apply edit operations
    for (i, a_char) in a.chars().enumerate() {
        for (j, b_char) in b.chars().enumerate() {
            let substitution = if a_char == b_char {0} else {1};
            mat[i+1][j+1] = cmp::min(
                cmp::min(
                    mat[i][j+1] + 1,        // deletion
                    mat[i+1][j] + 1         // insertion 
                ),
                mat[i][j] + substitution    // substitution
            );
        }
    }

    return mat[len_a][len_b];
}

#[cfg(test)]
mod tests {
    use super::levenshtein;

    #[test]
    fn basic() {
        assert_eq!(levenshtein("kitten", "sitting"), 3);
        assert_eq!(levenshtein("book", "back"), 2);
        assert_eq!(levenshtein("table", "dinner"), 5);
        assert_eq!(levenshtein("person", "pardon"), 2);
        assert_eq!(levenshtein("person", "persons"), 1);
    }

    #[test]
    fn equal() {
        assert_eq!(levenshtein("kitten", "kitten"), 0);
        assert_eq!(levenshtein("a", "a"), 0);
    }

    #[test]
    fn cases() {
        assert_eq!(levenshtein("Hello", "hello"), 1);
        assert_eq!(levenshtein("World", "world"), 1);
    }

    #[test]
    fn empty() {
        assert_eq!(levenshtein("book", ""), 4);
        assert_eq!(levenshtein("", "book"), 4);
        assert_eq!(levenshtein("", ""), 0);
    }

    #[test]
    fn unicode() {
        assert_eq!(levenshtein("Späße", "Spaß"), 2);
        assert_eq!(levenshtein("さようなら", "こんにちは"), 5);
        assert_eq!(levenshtein("さようなら", "さようなう"), 1);
        assert_eq!(levenshtein("こんにちは", "こんにちは abc"), 4);
        assert_eq!(levenshtein("༆༃ʘ", "༆˥ʘ"), 1);
    }
    
}