impl Solution {
    pub fn maximum_top(nums: Vec<i32>, k: i32) -> i32 {
        if nums.len() == 1 && (k - nums.len() as i32) % 2 == 0 {
            -1
        } else if k == 0 {
            nums[0]
        } else if k < nums.len() as i32 {
            nums[k as usize].max(*nums.iter().take(k as usize - 1).max().unwrap_or(&0))
        } else {
            *nums.iter().take(k as usize - 1).max().unwrap()
        }
    }
}
