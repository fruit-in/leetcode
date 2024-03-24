impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut i = 0;
        let mut sum;
        let mut ret = 1;

        nums.sort_unstable();
        sum = nums[0] as i64;

        for j in 1..nums.len() {
            while (j - i) as i64 * nums[j] as i64 - sum > k as i64 {
                sum -= nums[i] as i64;
                i += 1;
            }

            sum += nums[j] as i64;
            ret = ret.max(j - i + 1);
        }

        ret as i32
    }
}
