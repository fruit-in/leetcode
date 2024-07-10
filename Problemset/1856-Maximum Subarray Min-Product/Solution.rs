impl Solution {
    pub fn max_sum_min_product(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut prefix_sum = vec![0; n + 1];
        let mut stack = vec![];
        let mut indexl = vec![None; n];
        let mut indexr = vec![None; n];
        let mut ret = nums[0] as i64 * nums[0] as i64;

        for i in 0..n {
            prefix_sum[i + 1] = prefix_sum[i] + nums[i] as i64;

            while let Some(&j) = stack.last() {
                if nums[j] >= nums[i] {
                    stack.pop();
                } else {
                    indexl[i] = Some(j);
                    break;
                }
            }
            stack.push(i);
        }

        stack.clear();
        for i in (0..n).rev() {
            while let Some(&j) = stack.last() {
                if nums[j] >= nums[i] {
                    stack.pop();
                } else {
                    indexr[i] = Some(j);
                    break;
                }
            }
            stack.push(i);

            let mut sum = prefix_sum[indexr[i].unwrap_or(n)];
            if let Some(j) = indexl[i] {
                sum -= prefix_sum[j + 1];
            }
            ret = ret.max(nums[i] as i64 * sum);
        }

        (ret % 1_000_000_007) as i32
    }
}
