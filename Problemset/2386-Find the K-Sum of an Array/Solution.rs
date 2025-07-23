use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn k_sum(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut max_sum = nums.iter().map(|&x| x.max(0) as i64).sum::<i64>();
        let mut max_heap = BinaryHeap::from([max_sum]);
        let mut min_heap = BinaryHeap::<Reverse<i64>>::new();
        let mut sorted_abs_nums = nums.iter().map(|&x| x.abs() as i64).collect::<Vec<_>>();
        sorted_abs_nums.sort_unstable();

        for i in 0..sorted_abs_nums.len() {
            while let Some(x) = max_heap.pop() {
                if min_heap.len() == k && x <= min_heap.peek().unwrap().0 {
                    break;
                }
                min_heap.push(Reverse(x));
                if min_heap.len() < k || x - sorted_abs_nums[i] > min_heap.peek().unwrap().0 {
                    min_heap.push(Reverse(x - sorted_abs_nums[i]));
                }
                while min_heap.len() > k {
                    min_heap.pop();
                }
            }

            if min_heap.len() == k
                && (i == sorted_abs_nums.len() - 1
                    || max_sum - sorted_abs_nums[i + 1] <= min_heap.peek().unwrap().0)
            {
                return min_heap.pop().unwrap().0;
            }

            max_heap = BinaryHeap::new();

            while let Some(Reverse(x)) = min_heap.pop() {
                max_heap.push(x);
            }
        }

        unreachable!()
    }
}
