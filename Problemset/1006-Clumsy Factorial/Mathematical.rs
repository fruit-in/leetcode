impl Solution {
    pub fn clumsy(n: i32) -> i32 {
        match n % 4 {
            0 => (n + 1).max(7),
            1 | 2 if n < 4 => n,
            1 | 2 => n + 2,
            _ => (n - 1).max(6),
        }
    }
}
