impl Solution {
    pub fn best_rotation(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut prefix_sum = vec![0; n + 1];
        let mut max_score = 0;
        let mut ret = 0;

        for i in 0..n {
            prefix_sum[(n + i - nums[i] as usize) % n] += 1;
            prefix_sum[i] -= 1;
        }

        for i in (0..n).rev() {
            prefix_sum[i] += prefix_sum[i + 1];
            if max_score <= prefix_sum[i] {
                max_score = prefix_sum[i];
                ret = i;
            }
        }

        ret as i32
    }
}
