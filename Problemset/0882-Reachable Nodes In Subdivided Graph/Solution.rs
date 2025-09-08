use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn reachable_nodes(edges: Vec<Vec<i32>>, max_moves: i32, n: i32) -> i32 {
        let n = n as usize;
        let mut neighbors = vec![vec![]; n];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);
        let mut min_moves = vec![i32::MAX; n];
        let mut ret = 0;
        min_moves[0] = 0;

        for edge in &edges {
            let (u, v, dist) = (edge[0] as usize, edge[1] as usize, edge[2] + 1);
            neighbors[u].push((v, dist));
            neighbors[v].push((u, dist));
        }

        while let Some((Reverse(moves), u)) = heap.pop() {
            if moves > min_moves[u] {
                continue;
            }

            ret += 1;

            for &(v, dist) in &neighbors[u] {
                if moves + dist <= max_moves.min(min_moves[v] - 1) {
                    min_moves[v] = moves + dist;
                    heap.push((Reverse(min_moves[v]), v));
                }
            }
        }

        for edge in &edges {
            let (u, v, cnt) = (edge[0] as usize, edge[1] as usize, edge[2]);
            ret += cnt.min((max_moves - min_moves[u]).max(0) + (max_moves - min_moves[v]).max(0));
        }

        ret
    }
}
