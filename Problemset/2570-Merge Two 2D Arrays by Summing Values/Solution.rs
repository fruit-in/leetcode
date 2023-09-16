impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut i = 0;
        let mut j = 0;
        let mut ret = vec![];

        while i < nums1.len() || j < nums2.len() {
            if j >= nums2.len() {
                ret.push(nums1[i].clone());
                i += 1;
            } else if i >= nums1.len() {
                ret.push(nums2[j].clone());
                j += 1;
            } else if nums1[i][0] == nums2[j][0] {
                ret.push(vec![nums1[i][0], nums1[i][1] + nums2[j][1]]);
                i += 1;
                j += 1;
            } else if nums1[i][0] < nums2[j][0] {
                ret.push(nums1[i].clone());
                i += 1;
            } else {
                ret.push(nums2[j].clone());
                j += 1;
            }
        }

        ret
    }
}
