impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        words
            .into_iter()
            .find(|w| {
                let v = w.as_bytes();
                (0..v.len() / 2).all(|i| v[i] == v[v.len() - 1 - i])
            })
            .unwrap_or("".to_string())
    }
}
