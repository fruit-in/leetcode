use std::collections::VecDeque;

impl Solution {
    pub fn constrained_subset_sum(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut deque = VecDeque::new();
        let mut ret = *nums.iter().max().unwrap();

        for i in 0..nums.len() {
            if i - deque.front().unwrap_or(&(i, 0)).0 > k {
                deque.pop_front();
            }

            let x = nums[i] + deque.front().unwrap_or(&(0, 0)).1;

            if x > 0 {
                while deque.back().unwrap_or(&(0, i32::MAX)).1 <= x {
                    deque.pop_back();
                }
                deque.push_back((i, x));
                ret = ret.max(x);
            }
        }

        ret
    }
}
