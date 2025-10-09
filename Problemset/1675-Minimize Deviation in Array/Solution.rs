use std::collections::BinaryHeap;

impl Solution {
    pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::new();
        let mut min_num = i32::MAX;
        let mut ret = i32::MAX;

        for &num in &nums {
            heap.push(num * (num % 2 + 1));
            min_num = min_num.min(num * (num % 2 + 1));
        }

        while let Some(num) = heap.pop() {
            ret = ret.min(num - min_num);

            if num % 2 == 0 {
                heap.push(num / 2);
                min_num = min_num.min(num / 2);
            } else {
                break;
            }
        }

        ret
    }
}
