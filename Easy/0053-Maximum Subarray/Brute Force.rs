impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        for i in 0..nums.len() {
            let mut sum = 0;
            for j in i..nums.len() {
                sum += nums[j];
                if sum > max {
                    max = sum;
                }
            }
        }
        max
    }
}
