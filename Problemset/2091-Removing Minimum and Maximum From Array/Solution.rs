impl Solution {
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        let min_index = nums.iter().enumerate().min_by_key(|(_, &x)| x).unwrap().0;
        let max_index = nums.iter().enumerate().max_by_key(|(_, &x)| x).unwrap().0;
        let left = min_index.min(max_index);
        let right = min_index.max(max_index);

        (right + 1)
            .min(nums.len() - left)
            .min(left + 1 + nums.len() - right) as i32
    }
}
