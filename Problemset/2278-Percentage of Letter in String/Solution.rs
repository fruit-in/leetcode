impl Solution {
    pub fn percentage_letter(s: String, letter: char) -> i32 {
        (s.chars().filter(|&c| c == letter).count() * 100 / s.len()) as i32
    }
}
