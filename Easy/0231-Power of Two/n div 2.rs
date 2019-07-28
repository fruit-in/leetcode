impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n <= 0 {
            false
        } else if n == 1 {
            true
        } else {
            n % 2 == 0 && Self::is_power_of_two(n / 2)
        }
    }
}
