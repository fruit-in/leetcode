use std::collections::BinaryHeap;

impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let mut workers = quality
            .into_iter()
            .zip(wage.into_iter())
            .collect::<Vec<_>>();
        let mut quality_heap = BinaryHeap::new();
        let mut quality_sum = 0;
        let mut ret = f64::INFINITY;

        workers.sort_unstable_by(|(q0, w0), (q1, w1)| (q1 * w0).cmp(&(q0 * w1)));

        for i in 0..workers.len() {
            if i >= k {
                quality_sum -= quality_heap.pop().unwrap();
            }

            quality_heap.push(workers[i].0);
            quality_sum += workers[i].0;

            if i >= k - 1 {
                ret = ret.min(quality_sum as f64 * workers[i].1 as f64 / workers[i].0 as f64);
            }
        }

        ret
    }
}
