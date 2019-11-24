impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let mut l = nums.len();
        let mut r = 0;

        for i in 0..nums.len() {
            if nums.iter().skip(i + 1).any(|&x| x < nums[i]) {
                l = i;
                break;
            }
        }
        for i in (0..nums.len()).rev() {
            if nums.iter().take(i).any(|&x| x > nums[i]) {
                r = i;
                break;
            }
        }

        (r as i32 - l as i32 + 1).max(0)
    }
}
