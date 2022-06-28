impl Solution {
    pub fn is_three(n: i32) -> bool {
        (1..=n).filter(|m| n % m == 0).count() == 3
    }
}
