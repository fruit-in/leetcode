use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn second_greater_element(nums: Vec<i32>) -> Vec<i32> {
        let mut heap0 = BinaryHeap::new();
        let mut heap1 = BinaryHeap::new();
        let mut answer = vec![-1; nums.len()];

        for i in 0..nums.len() {
            while let Some(&Reverse((x, j))) = heap1.peek() {
                if x >= nums[i] {
                    break;
                }
                answer[j] = nums[i];
                heap1.pop();
            }
            while let Some(&Reverse((x, j))) = heap0.peek() {
                if x >= nums[i] {
                    break;
                }
                heap1.push(heap0.pop().unwrap());
            }
            heap0.push(Reverse((nums[i], i)));
        }

        answer
    }
}
