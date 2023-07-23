use std::collections::BinaryHeap;

impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut gifts = BinaryHeap::from(gifts);

        for _ in 0..k {
            let mut x = gifts.peek_mut().unwrap();
            *x = (*x as f64).sqrt() as i32;
        }

        gifts.iter().map(|&x| x as i64).sum()
    }
}
