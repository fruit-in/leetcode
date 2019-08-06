impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        if len < 2 {
            nums
        } else {
            Self::merge(Self::sort_array(nums[0..(len / 2)].to_vec()),
                Self::sort_array(nums[(len / 2)..].to_vec()))
        }
    }
    
    pub fn merge(mut left: Vec<i32>, mut right: Vec<i32>) -> Vec<i32> {
        let mut new_array = Vec::new();
        while left.len() > 0 && right.len() > 0 {
            if left[0] < right[0] {
                new_array.push(left.remove(0));
            } else {
                new_array.push(right.remove(0));
            }
        }
        while left.len() > 0 {
            new_array.push(left.remove(0));
        }
        while right.len() > 0 {
            new_array.push(right.remove(0));
        }
        new_array
    }
}
