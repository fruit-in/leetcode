use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut heap = (0..nums1.len().min(k as usize))
            .map(|i| (Reverse(nums1[i] + nums2[0]), i, 0))
            .collect::<BinaryHeap<_>>();
        let mut ret = vec![];

        for _ in 0..k {
            let (_, i, j) = heap.pop().unwrap();

            if j + 1 < nums2.len() {
                heap.push((Reverse(nums1[i] + nums2[j + 1]), i, j + 1));
            }
            ret.push(vec![nums1[i], nums2[j]]);
        }

        ret
    }
}
