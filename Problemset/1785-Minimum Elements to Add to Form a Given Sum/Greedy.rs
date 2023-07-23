impl Solution {
    pub fn min_elements(nums: Vec<i32>, limit: i32, goal: i32) -> i32 {
        ((nums.iter().map(|&x| x as i64).sum::<i64>() - goal as i64).abs() as f64 / limit as f64)
            .ceil() as i32
    }
}
