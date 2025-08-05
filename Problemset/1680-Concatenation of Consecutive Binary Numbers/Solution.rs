impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        fn pow_of_2(exp: u32) -> i64 {
            if exp == 0 {
                1
            } else if exp % 2 == 0 {
                pow_of_2(exp / 2) * pow_of_2(exp / 2) % 1_000_000_007
            } else {
                pow_of_2(exp / 2) * pow_of_2(exp / 2) * 2 % 1_000_000_007
            }
        }

        let mut shl = 0;
        let mut ret = 0;

        for x in (1..=n as i64).rev() {
            ret = (ret + x * pow_of_2(shl)) % 1_000_000_007;
            shl += x.ilog2() + 1;
        }

        ret as i32
    }
}
