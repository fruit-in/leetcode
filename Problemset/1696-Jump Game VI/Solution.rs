use std::collections::VecDeque;

impl Solution {
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut deque = VecDeque::from([(0, nums[0])]);

        for i in 1..nums.len() {
            if i - deque[0].0 > k {
                deque.pop_front();
            }

            let x = nums[i] + deque[0].1;

            while deque.back().unwrap_or(&(0, i32::MAX)).1 <= x {
                deque.pop_back();
            }

            deque.push_back((i, x));
        }

        deque.pop_back().unwrap().1
    }
}
