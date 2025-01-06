impl Solution {
    pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        if k == 0 {
            if (0..nums1.len()).all(|i| nums1[i] == nums2[i]) {
                return 0;
            } else {
                return -1;
            }
        }

        let mut inc = 0;
        let mut dec = 0;

        for i in 0..nums1.len() {
            if nums1[i] < nums2[i] && (nums2[i] - nums1[i]) % k == 0 {
                inc += ((nums2[i] - nums1[i]) / k) as i64;
            } else if nums1[i] > nums2[i] && (nums1[i] - nums2[i]) % k == 0 {
                dec += ((nums1[i] - nums2[i]) / k) as i64;
            } else if nums1[i] != nums2[i] {
                return -1;
            }
        }

        if inc == dec {
            inc
        } else {
            -1
        }
    }
}
