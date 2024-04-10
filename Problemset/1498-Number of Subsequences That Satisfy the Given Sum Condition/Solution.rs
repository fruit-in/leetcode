impl Solution {
    pub fn num_subseq(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        let mut i = nums.len() - 1;
        let mut pow2 = vec![1];
        let mut ret = 0;

        nums.sort_unstable();

        for j in 0..nums.len() {
            if j > i || nums[j] * 2 > target {
                break;
            }

            while nums[i] + nums[j] > target {
                i -= 1;
            }

            while i - j >= pow2.len() {
                pow2.push(pow2.last().unwrap() * 2 % 1_000_000_007);
            }

            ret = (ret + pow2[i - j]) % 1_000_000_007;
        }

        ret
    }
}
