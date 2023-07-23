impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let allowed = allowed.bytes().fold(0, |ac, c| ac | (1 << (c - b'a')));

        words
            .iter()
            .filter(|word| word.bytes().all(|c| allowed & (1 << (c - b'a')) != 0))
            .count() as i32
    }
}
