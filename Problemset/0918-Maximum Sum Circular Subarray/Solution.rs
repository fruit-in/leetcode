impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let sum = nums.iter().sum();
        let mut max_sum = nums[0];
        let mut min_sum = nums[0];
        let mut prefix_sum = nums[0];
        let mut ret = nums[0].max(sum);

        for i in 1..nums.len() {
            prefix_sum += nums[i];
            ret = ret
                .max(prefix_sum - min_sum)
                .max(sum - prefix_sum + max_sum);
            max_sum = max_sum.max(prefix_sum);
            min_sum = min_sum.min(prefix_sum);
        }

        ret
    }
}
