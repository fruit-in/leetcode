impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let mut prefix_sum = vec![0; nums.len() + 1];
        let mut ret = 0;

        for i in 0..nums.len() {
            prefix_sum[i + 1] = prefix_sum[i] + nums[i] as i64;
        }

        for i in 0..nums.len() {
            let mut lo = 0;
            let mut hi = i + 1;

            while lo < hi {
                let mid = (lo + hi) / 2;
                let score = (prefix_sum[i + 1] - prefix_sum[mid]) * (i + 1 - mid) as i64;

                if score < k {
                    hi = mid;
                } else {
                    lo = mid + 1;
                }
            }

            ret += (i + 1 - hi) as i64;
        }

        ret
    }
}
