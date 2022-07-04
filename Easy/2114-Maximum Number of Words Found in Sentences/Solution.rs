impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        sentences
            .iter()
            .map(|s| s.chars().filter(|c| c.is_whitespace()).count())
            .max()
            .unwrap() as i32
            + 1
    }
}
