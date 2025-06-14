use std::collections::BinaryHeap;

impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut nums = nums1.iter().zip(nums2.iter()).collect::<Vec<_>>();
        let mut heap = BinaryHeap::new();
        let mut sum = 0;
        let mut ret = 0;

        nums.sort_unstable_by_key(|&(_, y)| -y);

        for i in 0..nums.len() {
            heap.push(-nums[i].0);
            sum += *nums[i].0 as i64;

            if i >= k - 1 {
                if heap.len() > k {
                    sum -= -heap.pop().unwrap() as i64;
                }
                ret = ret.max(sum * *nums[i].1 as i64);
            }
        }

        ret
    }
}
