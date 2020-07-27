impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut i = 1;
        let mut j = 0;

        for k in 2..nums.len() {
            if nums[k] > nums[i] {
                i = k;
            }
        }
        for k in 1..nums.len() {
            if nums[k] > nums[j] && k != i {
                j = k;
            }
        }

        (nums[i] - 1) * (nums[j] - 1)
    }
}
