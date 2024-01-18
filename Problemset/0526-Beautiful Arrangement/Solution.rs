impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        Self::backtracking(n, 1, 0)
    }

    fn backtracking(n: i32, i: i32, bitmask: i32) -> i32 {
        if i > n {
            return 1;
        }

        (1..=n)
            .filter(|p| (bitmask >> p) & 1 == 0 && (p % i == 0 || i % p == 0))
            .map(|p| Self::backtracking(n, i + 1, bitmask | (1 << p)))
            .sum()
    }
}
