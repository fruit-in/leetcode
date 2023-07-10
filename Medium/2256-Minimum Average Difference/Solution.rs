impl Solution {
    pub fn minimum_average_difference(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i64;
        let mut prefix_sum = 0;
        let mut suffix_sum = nums.iter().map(|&x| x as i64).sum::<i64>();
        let mut min_avg_diff = i64::MAX;
        let mut ret = 0;

        for i in 0..nums.len() {
            prefix_sum += nums[i] as i64;
            suffix_sum -= nums[i] as i64;
            let prefix_avg = prefix_sum / (i as i64 + 1);
            let suffix_avg = suffix_sum.checked_div(n - i as i64 - 1).unwrap_or(0);
            let avg_diff = (prefix_avg - suffix_avg).abs();

            if avg_diff < min_avg_diff {
                min_avg_diff = avg_diff;
                ret = i;
            }
        }

        ret as i32
    }
}
