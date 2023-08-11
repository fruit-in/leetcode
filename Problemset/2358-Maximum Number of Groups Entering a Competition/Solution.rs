impl Solution {
    pub fn maximum_groups(grades: Vec<i32>) -> i32 {
        (((1.0 + 8.0 * grades.len() as f64).sqrt() - 1.0) / 2.0) as i32
    }
}
