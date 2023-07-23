impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let mut l = 0;
        let mut r = nums.len();

        while l < r {
            let m = (l + r) / 2;
            if target == nums[m] {
                return true;
            }
            if nums[l] == nums[m] && nums[m] == nums[r - 1] {
                l += 1;
                r -= 1;
            } else if nums[l] <= nums[m] {
                if target < nums[m] && target >= nums[l] {
                    r = m;
                } else {
                    l = m + 1;
                }
            } else {
                if target < nums[m] || target > *nums.last().unwrap() {
                    r = m;
                } else {
                    l = m + 1;
                }
            }
        }

        false
    }
}
