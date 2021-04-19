impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;

        while l <= r {
            let m = (l + r) / 2;

            if m > 0 && nums[m - 1] > nums[m] {
                return nums[m];
            } else if nums[m] >= nums[0] {
                l = m + 1;
            } else {
                r = m - 1;
            }
        }

        nums[0]
    }
}
