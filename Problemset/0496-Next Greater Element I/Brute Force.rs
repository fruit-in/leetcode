impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut ret = vec![-1; nums1.len()];

        for i in 0..nums1.len() {
            for j in (0..nums2.len()).rev() {
                if nums2[j] > nums1[i] {
                    ret[i] = nums2[j];
                } else if nums2[j] == nums1[i] {
                    break;
                }
            }
        }

        ret
    }
}
