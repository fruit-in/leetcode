impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut left = 0usize;
        let mut right = nums.len() - 1;
        let mut index = 0usize;
        while index <= right {
            if nums[index] == 0 && index > left {
                while left < index && nums[left] == 0 {
                    left += 1;
                }
                nums.swap(left, index);
            } else if nums[index] == 2 && index < right {
                while right > index && nums[right] == 2 {
                    right -= 1;
                }
                nums.swap(index, right);
            } else {
                index += 1;
            }
        }
    }
}
