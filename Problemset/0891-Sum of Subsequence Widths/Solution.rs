impl Solution {
    pub fn sum_subseq_widths(mut nums: Vec<i32>) -> i32 {
        let mut x = 0;
        let mut pow2 = 1;
        let mut pow2_sum = 1;
        let mut ret = 0;

        nums.sort_unstable();

        for i in 1..nums.len() {
            x = (2 * x + (nums[i] - nums[i - 1]) as i64 * pow2_sum) % 1_000_000_007;
            pow2 = (2 * pow2) % 1_000_000_007;
            pow2_sum = (pow2_sum + pow2) % 1_000_000_007;
            ret = (ret + x) % 1_000_000_007;
        }

        ret as i32
    }
}
