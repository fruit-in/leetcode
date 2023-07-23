impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        if word1.is_empty() && word2.is_empty() {
            return String::new();
        }

        format!(
            "{}{}{}",
            word1.get(..1).unwrap_or(""),
            word2.get(..1).unwrap_or(""),
            Self::merge_alternately(
                word1.get(1..).unwrap_or("").to_string(),
                word2.get(1..).unwrap_or("").to_string()
            )
        )
    }
}
