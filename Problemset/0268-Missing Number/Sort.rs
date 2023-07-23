impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut l = 0;
        let mut r = nums.len() - 1;

        while l <= r {
            let m = (l + r) / 2;
            if nums[m] != m as i32 {
                if m == 0 || nums[m - 1] == m as i32 - 1 {
                    return m as i32;
                }
                r = m - 1;
            } else {
                l = m + 1;
            }
        }
        nums.len() as i32
    }
}
