impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let mut ans = 0;
        let mut mask = 1;
        let mut c = false;
        for _ in 0..32 {
            if (a & mask == b & mask && c) || (a & mask != b & mask && !c) {
                ans |= mask;
            }
            if a & mask != 0 && b & mask != 0 {
                c = true;
            }
            if a & mask == 0 && b & mask == 0 {
                c = false;
            }
            mask <<= 1;
        }
        ans
    }
}
