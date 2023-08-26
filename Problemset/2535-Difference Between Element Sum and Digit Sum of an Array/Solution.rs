impl Solution {
    pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut element_sum = 0;
        let mut digit_sum = 0;

        for i in 0..nums.len() {
            element_sum += nums[i];
            while nums[i] > 0 {
                digit_sum += nums[i] % 10;
                nums[i] /= 10;
            }
        }

        (element_sum - digit_sum).abs()
    }
}
