impl Solution {
    pub fn count_orders(n: i32) -> i32 {
        let mut ret = 1_i64;

        for i in 1..n as i64 {
            ret = (ret * (2 * i + 1) * (i + 1)) % 1_000_000_007;
        }

        ret as i32
    }
}
