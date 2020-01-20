impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let mut ret = Vec::new();

        for i in (0..nums.len()).step_by(2) {
            ret.append(&mut vec![nums[i + 1]; nums[i] as usize]);
        }

        ret
    }
}
