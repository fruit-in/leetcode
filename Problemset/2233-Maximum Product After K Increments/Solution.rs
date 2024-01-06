use std::collections::BinaryHeap;

impl Solution {
    pub fn maximum_product(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums.into_iter().map(|num| -num).collect::<BinaryHeap<_>>();

        for _ in 0..k {
            *nums.peek_mut().unwrap() -= 1;
        }

        nums.iter()
            .fold(1, |acc, x| acc * (-x as i64) % 1_000_000_007) as i32
    }
}
