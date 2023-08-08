impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
        (1..n as i64).fold(1, |acc, x| acc + x * 4)
    }
}
