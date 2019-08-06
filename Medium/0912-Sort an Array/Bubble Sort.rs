impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut nums = nums;
        for i in 0..(len - 1) {
            for j in 0..(len - i - 1) {
                if nums[j] > nums[j + 1] {
                    nums.swap(j, j + 1);
                }
            }
        }
        nums
    }
}
