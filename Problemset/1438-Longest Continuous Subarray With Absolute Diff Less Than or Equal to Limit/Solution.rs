use std::collections::BinaryHeap;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut max_heap = BinaryHeap::new();
        let mut min_heap = BinaryHeap::new();
        let mut i = 0;
        let mut ret = 1;

        for j in 0..nums.len() {
            max_heap.push((nums[j], j));
            min_heap.push((-nums[j], j));

            while let Some(&(x, k)) = max_heap.peek() {
                if x - nums[j] > limit {
                    max_heap.pop();
                    i = i.max(k + 1);
                } else {
                    break;
                }
            }
            while let Some(&(x, k)) = min_heap.peek() {
                if nums[j] + x > limit {
                    min_heap.pop();
                    i = i.max(k + 1);
                } else {
                    break;
                }
            }

            ret = ret.max(j - i + 1);
        }

        ret as i32
    }
}
