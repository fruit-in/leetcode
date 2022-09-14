use std::collections::BinaryHeap;

impl Solution {
    pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
        let mut piles = BinaryHeap::from(piles);

        for _ in 0..k {
            let x = piles.pop().unwrap();

            piles.push((x + 1) / 2);
        }

        piles.into_iter().sum()
    }
}
