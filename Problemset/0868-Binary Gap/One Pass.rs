impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let mut n = n;
        let mut longest = 0;
        let mut gap = 0;
        while n & 1 == 0 {
            n >>= 1;
        }
        while n != 0 {
            n >>= 1;
            gap += 1;
            if n & 1 == 1 {
                longest = longest.max(gap);
                gap = 0;
            }
        }
        longest
    }
}
