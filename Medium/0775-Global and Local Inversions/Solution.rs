impl Solution {
    pub fn is_ideal_permutation(nums: Vec<i32>) -> bool {
        let mut max = 0;

        for i in 2..nums.len() {
            max = max.max(nums[i - 2]);
            if nums[i] < max {
                return false;
            }
        }

        true
    }
}
