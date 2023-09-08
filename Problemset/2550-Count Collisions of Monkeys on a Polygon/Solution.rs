impl Solution {
    pub fn monkey_move(n: i32) -> i32 {
        (Self::pow2(n) - 2).rem_euclid(1_000_000_007) as i32
    }

    fn pow2(n: i32) -> i64 {
        if n == 0 {
            1
        } else if n % 2 == 0 {
            Self::pow2(n / 2).pow(2) % 1_000_000_007
        } else {
            (Self::pow2(n - 1) * 2) % 1_000_000_007
        }
    }
}
