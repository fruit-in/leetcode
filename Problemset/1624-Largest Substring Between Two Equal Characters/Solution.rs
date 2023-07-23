impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut pairs = vec![(s.len() as i32, 0); 26];

        for (i, c) in s.bytes().enumerate() {
            let pair = &mut pairs[(c - b'a') as usize];

            pair.0 = pair.0.min(i as i32);
            pair.1 = pair.1.max(i as i32);
        }

        pairs.iter().map(|(i, j)| j - i - 1).max().unwrap()
    }
}
