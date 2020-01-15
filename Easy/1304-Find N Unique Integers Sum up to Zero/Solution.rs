impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        ((1 - n)..=n).step_by(2).collect()
    }
}
