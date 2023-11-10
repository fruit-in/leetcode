impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        let max_num = *nums.iter().max().unwrap();
        let mut dp = vec![vec![0; max_num as usize * 2 + 1]; max_num as usize + 1];
        let mut ret = 2;

        for i in 0..nums.len() {
            for diff in -max_num..=max_num {
                if nums[i] - diff < 0 || nums[i] - diff > max_num {
                    dp[nums[i] as usize][(diff + max_num) as usize] = 1;
                } else {
                    dp[nums[i] as usize][(diff + max_num) as usize] =
                        dp[(nums[i] - diff) as usize][(diff + max_num) as usize] + 1;
                }
                ret = ret.max(dp[nums[i] as usize][(diff + max_num) as usize]);
            }
        }

        ret
    }
}
