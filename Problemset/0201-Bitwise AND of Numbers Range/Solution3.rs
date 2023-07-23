impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        if n > m {
            Self::range_bitwise_and(m >> 1, n >> 1) << 1
        } else {
            m
        }
    }
}
