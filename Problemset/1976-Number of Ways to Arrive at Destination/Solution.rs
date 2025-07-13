use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut neighbors = vec![vec![]; n];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);
        let mut min_time_count = vec![(i64::MAX, 0); n];
        min_time_count[0] = (0, 1);

        for road in &roads {
            neighbors[road[0] as usize].push((road[1] as usize, road[2] as i64));
            neighbors[road[1] as usize].push((road[0] as usize, road[2] as i64));
        }

        while let Some((Reverse(time), u)) = heap.pop() {
            if time > min_time_count[u].0 {
                continue;
            }

            for &(v, t) in &neighbors[u] {
                if time + t < min_time_count[v].0 {
                    min_time_count[v] = (time + t, min_time_count[u].1);
                    heap.push((Reverse(time + t), v));
                } else if time + t == min_time_count[v].0 {
                    min_time_count[v].1 =
                        (min_time_count[v].1 + min_time_count[u].1) % 1_000_000_007;
                }
            }
        }

        min_time_count[n - 1].1
    }
}
