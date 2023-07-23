impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        match (n, (n - 2) / 3) {
            (2, _) | (3, _) => n - 1,
            (_, x) => 3i32.pow(x as u32) * (n - 3 * x),
        }
    }
}
