impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let mut count = [0; 26];

        for word in &words {
            for c in word.bytes() {
                count[(c - b'a') as usize] += 1;
            }
        }

        count.iter().all(|&x| x % words.len() == 0)
    }
}
