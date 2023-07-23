impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut nums = nums;
        for i in 1..len {
            let val = nums[i];
            let mut j = i;
            while j > 0 && nums[j - 1] > val {
                nums[j] = nums[j - 1];
                j -= 1;
            }
            nums[j] = val;
        }
        nums
    }
}
