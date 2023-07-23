impl Solution {
    pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
        words
            .iter()
            .filter(|word| s.starts_with(word.as_str()))
            .count() as i32
    }
}
