impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        match letters.binary_search(&target) {
            Ok(i) => letters[(i + 1) % letters.len()],
            Err(i) => letters[i % letters.len()],
        }
    }
}
