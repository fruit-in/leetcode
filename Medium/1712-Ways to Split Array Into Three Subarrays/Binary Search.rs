impl Solution {
    pub fn ways_to_split(mut nums: Vec<i32>) -> i32 {
        let mut ret = 0;

        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }

        let sum = *nums.last().unwrap();

        for i in 0..nums.len() - 2 {
            let j = match nums[i + 1..].binary_search(&(2 * nums[i] - 1)) {
                Ok(a) => a + 1,
                Err(b) => b,
            };
            let k = match nums[i + 1..].binary_search(&((sum - nums[i]) / 2 + nums[i])) {
                Ok(a) if a == nums.len() - i - 2 => a,
                Ok(a) => a + 1,
                Err(b) if b == nums.len() - i - 1 => b - 1,
                Err(b) => b,
            };

            ret = (ret + k.saturating_sub(j) as i32) % 1_000_000_007;
        }

        ret
    }
}
