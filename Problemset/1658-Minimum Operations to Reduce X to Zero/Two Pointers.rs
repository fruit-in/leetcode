impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let mut l = nums.len();
        let mut r = 0;
        let mut sum = nums.iter().sum::<i32>();
        let mut ret = -1;

        while r < nums.len() {
            if sum == x && (ret == -1 || l + r < ret as usize) {
                ret = (l + r) as i32;
            }
            if (sum > x && l > 0) || l + r >= nums.len() {
                l -= 1;
                sum -= nums[l];
            } else {
                r += 1;
                sum += nums[nums.len() - r];
            }
        }

        ret
    }
}
