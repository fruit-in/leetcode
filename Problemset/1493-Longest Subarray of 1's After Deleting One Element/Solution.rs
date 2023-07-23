impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut zeros = 0;
        let mut l = 0;
        let mut ret = 0;

        for r in 0..nums.len() {
            if nums[r] == 0 {
                zeros += 1;
            }
            while zeros > 1 {
                if nums[l] == 0 {
                    zeros -= 1;
                }
                l += 1;
            }
            ret = ret.max(r - l - zeros + 1)
        }

        ret.min(nums.len() - 1) as i32
    }
}
