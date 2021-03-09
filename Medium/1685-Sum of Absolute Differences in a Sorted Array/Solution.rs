impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len() as i32;
        let total_sum = nums.iter().sum::<i32>();
        let mut left_sum = 0;
        let mut result = vec![0; nums.len()];

        for i in 0..nums.len() {
            result[i] = (2 * i as i32 - len) * nums[i] + total_sum - 2 * left_sum;
            left_sum += nums[i];
        }

        result
    }
}
