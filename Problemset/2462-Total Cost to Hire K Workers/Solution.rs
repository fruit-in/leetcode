use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        let mut heap = BinaryHeap::new();
        let mut left = 0;
        let mut right = costs.len() - 1;
        let mut ret = 0;

        for _ in 0..candidates {
            if left <= right {
                heap.push(Reverse((costs[left], left)));
                left += 1;
            }
            if right >= left {
                heap.push(Reverse((costs[right], right)));
                right -= 1;
            }
        }

        for _ in 0..k {
            let Reverse((cost, i)) = heap.pop().unwrap();

            if left <= right {
                if i < left {
                    heap.push(Reverse((costs[left], left)));
                    left += 1;
                } else {
                    heap.push(Reverse((costs[right], right)));
                    right -= 1;
                }
            }

            ret += cost as i64;
        }

        ret
    }
}
