impl Solution {
    pub fn hamming_weight(n: u32) -> i32 {
        if n == 0 {
            0
        } else {
            Self::hamming_weight(n >> 1) + (n & 1)
        }
    }
}
