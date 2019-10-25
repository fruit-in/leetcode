impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        let mut ret = 0;

        for i in 0..31 {
            if n >> i == 0 && i > 0 {
                break;
            }
            if n & (1 << i) == 0 {
                ret ^= 1 << i;
            }
        }

        ret
    }
}
