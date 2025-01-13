impl Solution {
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; nums2.len() + 1]; nums1.len() + 1];
        let mut ret = i32::MIN;

        for i in 0..nums1.len() {
            for j in 0..nums2.len() {
                ret = ret.max(nums1[i] * nums2[j]);
                dp[i + 1][j + 1] = dp[i + 1][j + 1]
                    .max(dp[i][j + 1])
                    .max(dp[i + 1][j])
                    .max(dp[i][j] + nums1[i] * nums2[j]);
            }
        }

        if ret >= 0 {
            ret = ret.max(dp[nums1.len()][nums2.len()]);
        }

        ret
    }
}
