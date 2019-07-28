impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        let mut result = 0;
        let mut mask = 1 << 30;
        while mask != 0 && m & mask == n & mask {
            result |= m & mask;
            mask >>= 1;
        }
        result
    }
}
