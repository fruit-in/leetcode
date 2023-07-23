impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;

        while l < r {
            let m = (l + r) / 2;
            if nums[m] < nums[m + 1] {
                l = m + 1;
            } else {
                r = m;
            }
        }

        l as i32
    }
}
