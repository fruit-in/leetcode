impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut left_sum = vec![0; n];
        let mut right_sum = vec![0; n];

        for i in 1..n {
            left_sum[i] = left_sum[i - 1] + nums[i - 1];
            right_sum[n - 1 - i] = right_sum[n - i] + nums[n - i];
        }

        (0..n).map(|i| (left_sum[i] - right_sum[i]).abs()).collect()
    }
}
