impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let mut sum: i32 = nums.iter().take(k).sum();
        let mut max_sum = sum;

        for i in k..nums.len() {
            sum += nums[i] - nums[i - k];
            max_sum = max_sum.max(sum);
        }

        max_sum as f64 / k as f64
    }
}
