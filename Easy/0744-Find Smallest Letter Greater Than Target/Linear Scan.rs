impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        for &ch in &letters {
            if ch > target {
                return ch;
            }
        }
        letters[0]
    }
}
