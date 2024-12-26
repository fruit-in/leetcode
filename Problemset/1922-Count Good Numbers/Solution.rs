impl Solution {
    pub fn count_good_numbers(n: i64) -> i32 {
        fn twenty_pow(n: i64) -> i64 {
            if n == 0 {
                1
            } else if n % 2 == 0 {
                twenty_pow(n / 2) * twenty_pow(n / 2) % 1_000_000_007
            } else {
                twenty_pow(n - 1) * 20 % 1_000_000_007
            }
        }

        (match n % 2 {
            0 => twenty_pow(n / 2),
            _ => twenty_pow((n - 1) / 2) * 5 % 1_000_000_007,
        }) as i32
    }
}
