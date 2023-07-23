impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        sentence
            .split(' ')
            .position(|s| s.starts_with(&search_word))
            .unwrap_or(-2_i32 as usize) as i32
            + 1
    }
}
