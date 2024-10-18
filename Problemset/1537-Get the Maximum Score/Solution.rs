impl Solution {
    pub fn max_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp1 = vec![0; nums1.len()];
        let mut dp2 = vec![0; nums2.len()];
        let mut i = 0;
        let mut j = 0;

        while i < nums1.len() || j < nums2.len() {
            if j >= nums2.len() || (i < nums1.len() && nums1[i] < nums2[j]) {
                if i > 0 {
                    dp1[i] = dp1[i - 1];
                }
                dp1[i] += nums1[i] as i64;
                i += 1;
            } else if i >= nums1.len() || (j < nums2.len() && nums1[i] > nums2[j]) {
                if j > 0 {
                    dp2[j] = dp2[j - 1];
                }
                dp2[j] += nums2[j] as i64;
                j += 1;
            } else {
                if i > 0 {
                    dp1[i] = dp1[i - 1];
                }
                if j > 0 {
                    dp1[i] = dp1[i].max(dp2[j - 1]);
                }
                dp1[i] += nums1[i] as i64;
                dp2[j] = dp1[i];
                i += 1;
                j += 1;
            }
        }

        (dp1.pop().unwrap().max(dp2.pop().unwrap()) % 1_000_000_007) as i32
    }
}
