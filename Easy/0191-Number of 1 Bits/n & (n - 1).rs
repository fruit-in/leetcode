impl Solution {
    pub fn hamming_weight(n: u32) -> i32 {
        let mut ans = 0;
        let mut n = n;
        while n != 0 {
            ans += 1;
            n &= n - 1;
        }
        ans
    }
}
