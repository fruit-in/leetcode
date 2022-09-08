impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let mut sum_l = 0;
        let mut sum_r = nums.iter().map(|&x| x as i64).sum();
        let mut ret = 0;

        for i in 0..nums.len() - 1 {
            sum_l += nums[i] as i64;
            sum_r -= nums[i] as i64;

            if sum_l >= sum_r {
                ret += 1;
            }
        }

        ret
    }
}
