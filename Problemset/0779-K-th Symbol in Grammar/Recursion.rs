impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        if n <= 2 {
            k - 1
        } else if k > 2_i32.pow(n as u32 - 2) {
            1 - Self::kth_grammar(n - 1, k - 2_i32.pow(n as u32 - 2))
        } else {
            Self::kth_grammar(n - 1, k)
        }
    }
}
