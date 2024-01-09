use std::collections::BinaryHeap;

impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut w = w;
        let mut projects = capital
            .into_iter()
            .zip(profits.into_iter())
            .collect::<Vec<_>>();
        let mut heap = BinaryHeap::new();
        let mut i = 0;

        projects.sort_unstable();

        for _ in 0..k {
            while i < projects.len() && projects[i].0 <= w {
                heap.push(projects[i].1);
                i += 1;
            }

            w += heap.pop().unwrap_or(0);
        }

        w
    }
}
