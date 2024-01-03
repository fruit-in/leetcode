impl Solution {
    pub fn max_sum_range_query(nums: Vec<i32>, requests: Vec<Vec<i32>>) -> i32 {
        let mut nums = nums;
        let mut suffix_sum = vec![0; nums.len()];

        for request in &requests {
            if request[0] > 0 {
                suffix_sum[request[0] as usize - 1] -= 1;
            }
            suffix_sum[request[1] as usize] += 1;
        }

        for i in (0..suffix_sum.len() - 1).rev() {
            suffix_sum[i] += suffix_sum[i + 1];
        }

        nums.sort_unstable();
        suffix_sum.sort_unstable();

        (nums
            .iter()
            .zip(suffix_sum.iter())
            .map(|(x, y)| *x as i64 * *y as i64)
            .sum::<i64>()
            % 1_000_000_007) as i32
    }
}
