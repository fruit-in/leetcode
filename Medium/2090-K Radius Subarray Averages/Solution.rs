impl Solution {
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut avgs = vec![-1; nums.len()];
        let mut sum = nums.iter().take(2 * k + 1).map(|&x| x as i64).sum::<i64>();

        for i in k..nums.len().saturating_sub(k) {
            avgs[i] = (sum / (2 * k as i64 + 1)) as i32;
            sum += (nums.get(i + k + 1).unwrap_or(&0) - nums[i - k]) as i64;
        }

        avgs
    }
}
