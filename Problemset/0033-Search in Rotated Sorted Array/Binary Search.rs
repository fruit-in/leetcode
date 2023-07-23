impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len();

        while l < r {
            let m = (l + r) / 2;
            if target == nums[m] {
                return m as i32;
            }
            if nums[l] <= nums[m] && nums[m] <= nums[r - 1] {
                if target < nums[m] {
                    r = m;
                } else {
                    l = m + 1;
                }
            } else if nums[l] >= nums[m] {
                if target < nums[m] || target >= nums[l] {
                    r = m;
                } else {
                    l = m + 1;
                }
            } else if nums[m] >= nums[r - 1] {
                if target < nums[m] && target >= nums[l] {
                    r = m;
                } else {
                    l = m + 1;
                }
            }
        }

        -1
    }
}
