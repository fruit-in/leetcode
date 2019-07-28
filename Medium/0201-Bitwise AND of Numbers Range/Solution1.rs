impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        let mut result = n;
        for i in m..n {
            result &= i;
        }
        result
    }
}
