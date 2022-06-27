impl Solution {
    pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();
        let mut count = [0_i32; 26];

        for i in 0..word1.len() {
            count[(word1[i] - b'a') as usize] += 1;
            count[(word2[i] - b'a') as usize] -= 1;
        }

        count.iter().all(|&x| x.abs() <= 3)
    }
}
