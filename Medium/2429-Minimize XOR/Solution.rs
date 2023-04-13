impl Solution {
    pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
        let ones1 = num1.count_ones() as i32;
        let ones2 = num2.count_ones() as i32;
        let mut i = 0;
        let mut x = num1;

        for _ in 0..(ones1 - ones2).max(ones2 - ones1) {
            if ones1 > ones2 {
                while x & (1 << i) == 0 {
                    i += 1;
                }

                x ^= 1 << i;
            } else {
                while x & (1 << i) != 0 {
                    i += 1;
                }

                x |= 1 << i;
            }
        }

        x
    }
}
