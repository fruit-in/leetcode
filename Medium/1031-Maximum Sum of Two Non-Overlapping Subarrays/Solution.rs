impl Solution {
    pub fn max_sum_two_no_overlap(nums: Vec<i32>, first_len: i32, second_len: i32) -> i32 {
        let first_len = first_len as usize;
        let second_len = second_len as usize;
        let mut prefix_sum = vec![0; nums.len() + 1];
        let mut ret = i32::MIN;

        for i in 1..prefix_sum.len() {
            prefix_sum[i] = prefix_sum[i - 1] + nums[i - 1];
        }

        for i in 0..nums.len() - first_len + 1 {
            let first_sum = prefix_sum[i + first_len] - prefix_sum[i];
            for j in 0..(i + 1).saturating_sub(second_len) {
                let second_sum = prefix_sum[j + second_len] - prefix_sum[j];
                ret = ret.max(first_sum + second_sum);
            }
            for j in i + first_len..nums.len() - second_len + 1 {
                let second_sum = prefix_sum[j + second_len] - prefix_sum[j];
                ret = ret.max(first_sum + second_sum);
            }
        }

        ret
    }
}
