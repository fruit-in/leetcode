impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut ret = Vec::new();
        for n1 in &nums1 {
            if !ret.contains(n1) {
                for n2 in &nums2 {
                    if n2 == n1 {
                        ret.push(*n1);
                        break;
                    }
                }
            }
        }
        ret
    }
}
