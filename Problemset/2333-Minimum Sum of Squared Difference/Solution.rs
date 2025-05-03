impl Solution {
    pub fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
        let mut k = (k1 + k2) as i64;
        let mut diffs = (0..nums1.len())
            .map(|i| (nums1[i] - nums2[i]).abs() as i64)
            .collect::<Vec<_>>();
        let mut count = 1;

        diffs.push(0);
        diffs.sort_unstable();

        for i in (0..diffs.len()).rev() {
            if diffs[i] == 0 {
                return 0;
            }

            if (diffs[i] - diffs[i - 1]) * count <= k {
                k -= (diffs[i] - diffs[i - 1]) * count;
                count += 1;
            } else {
                let tmp = diffs[i] - k / count;
                k -= (diffs[i] - tmp) * count;
                return count * tmp * tmp - 2 * k * tmp
                    + k
                    + (0..i).map(|j| diffs[j] * diffs[j]).sum::<i64>();
            }
        }

        unreachable!()
    }
}
