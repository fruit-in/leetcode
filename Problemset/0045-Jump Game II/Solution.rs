impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = 0;
        let mut ret = 0;

        for k in 0..nums.len() {
            if k > i {
                i = j;
                ret += 1;
            }
            j = j.max(k + nums[k] as usize);
        }

        ret
    }
}
