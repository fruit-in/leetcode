impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        (1..(m as i64)).fold(1, |acc, x| acc * (n as i64 - 1 + x) / x) as i32
    }
}
