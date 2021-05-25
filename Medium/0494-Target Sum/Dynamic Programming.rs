impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let sum = nums.iter().sum::<i32>() as usize;
        if sum < target.abs() as usize {
            return 0;
        }

        let mut dp0 = vec![0; 2 * sum + 1];
        dp0[sum] = 1;

        for num in nums {
            let mut dp1 = vec![0; 2 * sum + 1];

            for i in 0..2 * sum + 1 {
                if dp0[i] > 0 {
                    dp1[i + num as usize] += dp0[i];
                    dp1[i - num as usize] += dp0[i];
                }
            }

            dp0 = dp1;
        }

        dp0[(sum as i32 + target) as usize]
    }
}
