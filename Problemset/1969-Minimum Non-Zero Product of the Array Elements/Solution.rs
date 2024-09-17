impl Solution {
    pub fn min_non_zero_product(p: i32) -> i32 {
        let x = 2_u64.pow(p as u32) % 1_000_000_007;

        ((x - 1) * Self::power(x - 2, 2_u64.pow(p as u32 - 1) - 1) % 1_000_000_007) as i32
    }

    fn power(x: u64, exp: u64) -> u64 {
        if exp == 0 {
            1
        } else if exp % 2 == 0 {
            let y = Self::power(x, exp / 2);
            y * y % 1_000_000_007
        } else {
            x * Self::power(x, exp - 1) % 1_000_000_007
        }
    }
}
