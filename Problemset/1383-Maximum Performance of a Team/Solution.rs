use std::collections::BinaryHeap;

impl Solution {
    pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        let mut engineers = efficiency.iter().zip(speed.iter()).collect::<Vec<_>>();
        let mut heap = BinaryHeap::new();
        let mut speed_sum = 0;
        let mut ret = 0;

        engineers.sort_unstable();

        for &(&e, &s) in engineers.iter().rev() {
            if heap.len() == k as usize {
                speed_sum += heap.pop().unwrap();
            }
            speed_sum += s as i64;
            heap.push(-s as i64);
            ret = ret.max(speed_sum * e as i64);
        }

        (ret % 1_000_000_007) as i32
    }
}
