impl Solution {
    pub fn word_subsets(a: Vec<String>, b: Vec<String>) -> Vec<String> {
        let mut count = [0; 26];
        for sub in b {
            let count_ = Self::count_chars(&sub);
            for i in 0..26 {
                count[i] = count[i].max(count_[i]);
            }
        }

        a.into_iter()
            .filter(|word| {
                Self::count_chars(&word)
                    .iter()
                    .zip(count.iter())
                    .all(|(x, y)| x >= y)
            })
            .collect()
    }

    fn count_chars(s: &str) -> [i32; 26] {
        let mut count = [0; 26];
        for c in s.bytes() {
            count[(c - b'a') as usize] += 1;
        }

        count
    }
}
