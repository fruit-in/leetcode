impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut left_max = vec![0; height.len()];
        let mut right_max = vec![0; height.len()];

        for i in 1..height.len() {
            left_max[i] = left_max[i - 1].max(height[i - 1]);
        }
        for i in (0..height.len() - 1).rev() {
            right_max[i] = right_max[i + 1].max(height[i + 1]);
        }

        (0..height.len())
            .map(|i| (left_max[i].min(right_max[i]) - height[i]).max(0))
            .sum()
    }
}
