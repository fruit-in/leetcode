use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn minimum_weight(n: i32, edges: Vec<Vec<i32>>, src1: i32, src2: i32, dest: i32) -> i64 {
        let n = n as usize;
        let src1 = src1 as usize;
        let src2 = src2 as usize;
        let dest = dest as usize;
        let mut froms = vec![vec![]; n];
        let mut tos = vec![vec![]; n];
        let mut from_src1 = vec![-1; n];
        let mut from_src2 = vec![-1; n];
        let mut to_dest = vec![-1; n];
        from_src1[src1] = 0;
        from_src2[src2] = 0;
        to_dest[dest] = 0;

        for edge in &edges {
            let (from, to, weight) = (edge[0] as usize, edge[1] as usize, edge[2] as i64);
            froms[to].push((from, weight));
            tos[from].push((to, weight));
        }

        let mut heap = BinaryHeap::from([(Reverse(0), src1)]);
        while let Some((Reverse(weights), from)) = heap.pop() {
            if weights > from_src1[from] {
                continue;
            }

            for &(to, weight) in &tos[from] {
                if from_src1[to] == -1 || weights + weight < from_src1[to] {
                    from_src1[to] = weights + weight;
                    heap.push((Reverse(from_src1[to]), to));
                }
            }
        }

        heap = BinaryHeap::from([(Reverse(0), src2)]);
        while let Some((Reverse(weights), from)) = heap.pop() {
            if weights > from_src2[from] {
                continue;
            }

            for &(to, weight) in &tos[from] {
                if from_src2[to] == -1 || weights + weight < from_src2[to] {
                    from_src2[to] = weights + weight;
                    heap.push((Reverse(from_src2[to]), to));
                }
            }
        }

        heap = BinaryHeap::from([(Reverse(0), dest)]);
        while let Some((Reverse(weights), to)) = heap.pop() {
            if weights > to_dest[to] {
                continue;
            }

            for &(from, weight) in &froms[to] {
                if to_dest[from] == -1 || weights + weight < to_dest[from] {
                    to_dest[from] = weights + weight;
                    heap.push((Reverse(to_dest[from]), from));
                }
            }
        }

        (0..n)
            .filter(|&i| from_src1[i] >= 0 && from_src2[i] >= 0 && to_dest[i] >= 0)
            .map(|i| from_src1[i] + from_src2[i] + to_dest[i])
            .min()
            .unwrap_or(-1)
    }
}
