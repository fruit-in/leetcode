impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let mut n = n;
        let mut pre = n & 1;
        while n != 0 {
            n >>= 1;
            if pre == n & 1 {
                return false;
            }
            pre = n & 1;
        }

        true
    }
}
