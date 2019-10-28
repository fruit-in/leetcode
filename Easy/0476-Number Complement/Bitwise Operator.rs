impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut ret = 0;

        for i in 0..30 {
            if num >> i == 0 {
                break;
            }
            if num & (1 << i) == 0 {
                ret ^= 1 << i;
            }
        }

        ret
    }
}
