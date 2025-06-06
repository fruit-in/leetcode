use std::collections::BinaryHeap;

impl Solution {
    pub fn max_star_sum(vals: Vec<i32>, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let mut heaps = vec![BinaryHeap::new(); vals.len()];
        let mut star_sum = vals.clone();

        for edge in &edges {
            let (a, b) = (edge[0] as usize, edge[1] as usize);

            if vals[b] > 0 {
                heaps[a].push(-vals[b]);
                star_sum[a] += vals[b];
            }
            if vals[a] > 0 {
                heaps[b].push(-vals[a]);
                star_sum[b] += vals[a];
            }
            if heaps[a].len() > k {
                star_sum[a] -= -heaps[a].pop().unwrap();
            }
            if heaps[b].len() > k {
                star_sum[b] -= -heaps[b].pop().unwrap();
            }
        }

        *star_sum.iter().max().unwrap()
    }
}
