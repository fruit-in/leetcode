impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        let mut n = n;
        while n > m {
            n &= n - 1;
        }
        n
    }
}
