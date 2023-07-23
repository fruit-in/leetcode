use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let set1: HashSet<_> = nums1.iter().collect();
        let set2: HashSet<_> = nums2.iter().collect();

        set1.intersection(&set2).map(|&&x| x).collect()
    }
}
