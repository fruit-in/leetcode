use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut deque = VecDeque::new();
        let mut ret = vec![];

        for i in 0..nums.len() {
            if i >= k && *deque.front().unwrap_or(&100000) <= i - k {
                deque.pop_front();
            }

            while let Some(&j) = deque.back() {
                if nums[j] < nums[i] {
                    deque.pop_back();
                } else {
                    break;
                }
            }

            deque.push_back(i);

            if i >= k - 1 {
                ret.push(nums[*deque.front().unwrap()]);
            }
        }

        ret
    }
}
