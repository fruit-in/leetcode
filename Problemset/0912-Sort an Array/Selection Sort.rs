impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut nums = nums;
        for i in 0..(len - 1) {
            let mut min_index = i;
            for j in (i + 1)..len {
                if nums[j] < nums[min_index] {
                    min_index = j;
                }
            }
            nums.swap(i, min_index);
        }
        nums
    }
}
