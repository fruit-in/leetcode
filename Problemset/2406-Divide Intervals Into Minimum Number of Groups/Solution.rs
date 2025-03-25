use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_groups(mut intervals: Vec<Vec<i32>>) -> i32 {
        let mut groups = BinaryHeap::new();

        intervals.sort_unstable();

        for interval in &intervals {
            let (left, right) = (interval[0], interval[1]);

            if *groups.peek().unwrap_or(&Reverse(i32::MAX)) > Reverse(left) {
                groups.pop();
            }
            groups.push(Reverse(right));
        }

        groups.len() as i32
    }
}
