impl Solution {
    pub fn maximums_spliced_array(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let mut prefix_sum1 = vec![0; n + 1];
        let mut prefix_sum2 = vec![0; n + 1];
        let mut max_diff12 = 0;
        let mut max_diff21 = 0;
        let mut ret = 0;

        for i in 0..n {
            prefix_sum1[i + 1] = prefix_sum1[i] + nums1[i];
            prefix_sum2[i + 1] = prefix_sum2[i] + nums2[i];
        }

        for i in 0..n {
            max_diff12 = max_diff12.max(prefix_sum1[i] - prefix_sum2[i]);
            max_diff21 = max_diff21.max(prefix_sum2[i] - prefix_sum1[i]);
            ret = ret.max(prefix_sum1[n] + prefix_sum2[i + 1] - prefix_sum1[i + 1] + max_diff12);
            ret = ret.max(prefix_sum2[n] + prefix_sum1[i + 1] - prefix_sum2[i + 1] + max_diff21);
        }

        ret
    }
}
