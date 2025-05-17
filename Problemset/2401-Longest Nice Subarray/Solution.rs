impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut mask = 0;
        let mut ret = 0;

        for j in 0..nums.len() {
            while mask & nums[j] != 0 {
                mask ^= nums[i];
                i += 1;
            }
            ret = ret.max(j - i + 1);
            mask |= nums[j];
        }

        ret as i32
    }
}
