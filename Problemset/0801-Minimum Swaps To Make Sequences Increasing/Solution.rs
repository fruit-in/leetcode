impl Solution {
    pub fn min_swap(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = [[0, 1], [i32::MAX; 2]];

        for i in 1..nums1.len() {
            if nums1[i] > nums1[i - 1] && nums2[i] > nums2[i - 1] {
                dp[1][0] = dp[0][0];
                dp[1][1] = dp[0][1] + 1;
            }
            if nums1[i] > nums2[i - 1] && nums2[i] > nums1[i - 1] {
                dp[1][0] = dp[1][0].min(dp[0][1]);
                dp[1][1] = dp[1][1].min(dp[0][0] + 1);
            }

            dp = [dp[1], [i32::MAX; 2]];
        }

        dp[0][0].min(dp[0][1])
    }
}
