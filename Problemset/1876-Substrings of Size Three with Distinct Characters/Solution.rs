impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        s.into_bytes()
            .windows(3)
            .filter(|gs| gs[0] != gs[1] && gs[0] != gs[2] && gs[1] != gs[2])
            .count() as i32
    }
}
