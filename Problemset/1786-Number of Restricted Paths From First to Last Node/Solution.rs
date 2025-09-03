use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn count_restricted_paths(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut neighbors = vec![vec![]; n + 1];
        let mut min_heap = BinaryHeap::from([(Reverse(0), n)]);
        let mut max_heap = BinaryHeap::new();
        let mut distance_to_last_node = vec![i64::MAX; n + 1];
        let mut paths = vec![0; n + 1];
        distance_to_last_node[n] = 0;
        paths[1] = 1;

        for edge in &edges {
            let (u, v, weight) = (edge[0] as usize, edge[1] as usize, edge[2] as i64);
            neighbors[u].push((v, weight));
            neighbors[v].push((u, weight));
        }

        while let Some((Reverse(dist), u)) = min_heap.pop() {
            if dist > distance_to_last_node[u] {
                continue;
            }

            max_heap.push((distance_to_last_node[u], u));

            for &(v, weight) in &neighbors[u] {
                if dist + weight < distance_to_last_node[v] {
                    distance_to_last_node[v] = dist + weight;
                    min_heap.push((Reverse(distance_to_last_node[v]), v));
                }
            }

            if u == 1 {
                break;
            }
        }

        while let Some((dist, u)) = max_heap.pop() {
            for &(v, _) in &neighbors[u] {
                if distance_to_last_node[v] > dist {
                    paths[u] = (paths[u] + paths[v]) % 1_000_000_007;
                }
            }
        }

        paths[n]
    }
}
