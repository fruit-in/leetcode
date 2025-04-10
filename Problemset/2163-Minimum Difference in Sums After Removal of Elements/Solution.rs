use std::collections::BinaryHeap;

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>) -> i64 {
        let nums = nums.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        let n = nums.len() / 3;
        let mut heap = BinaryHeap::new();
        let mut min_sum1 = vec![0; n + 1];
        let mut max_sum2 = vec![0; n + 1];

        for i in 0..n {
            heap.push(nums[i]);
            min_sum1[0] += nums[i];
        }
        for i in 1..=n {
            min_sum1[i] = min_sum1[i - 1];
            if nums[n + i - 1] < *heap.peek().unwrap() {
                min_sum1[i] -= heap.pop().unwrap();
                heap.push(nums[n + i - 1]);
                min_sum1[i] += nums[n + i - 1];
            }
        }

        heap.clear();

        for i in n * 2..nums.len() {
            heap.push(-nums[i]);
            max_sum2[n] += nums[i];
        }
        for i in (0..n).rev() {
            max_sum2[i] = max_sum2[i + 1];
            if nums[n + i] > -*heap.peek().unwrap() {
                max_sum2[i] -= -heap.pop().unwrap();
                heap.push(-nums[n + i]);
                max_sum2[i] += nums[n + i];
            }
        }

        (0..=n).map(|i| min_sum1[i] - max_sum2[i]).min().unwrap()
    }
}
