use std::cmp;

pub fn levenshtein(a: &str, b: &str) -> i32 {
    let len_a = a.chars().count();
    let len_b = b.chars().count();

    let row: Vec<i32> = vec![0; len_b + 1];
    let mut mat: Vec<Vec<i32>> = vec![row; len_a + 1];

    // source prefixes can be transformed into empty string by
    // dropping all characters
    for i in 1..len_a {
        mat[i][0] = i as i32;
    }

    // target prefixes can be reached from empty source prefix
    // by inserting every character
    for i in 1..len_b {
        mat[0][i] = i as i32;
    }

    // do edit operations in mat
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



}