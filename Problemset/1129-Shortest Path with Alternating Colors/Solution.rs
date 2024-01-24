use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;

impl Solution {
    pub fn shortest_alternating_paths(
        n: i32,
        red_edges: Vec<Vec<i32>>,
        blue_edges: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let n = n as usize;
        let mut red_nexts = vec![vec![]; n];
        let mut blue_nexts = vec![vec![]; n];
        let mut nodes_heap = BinaryHeap::from([(Reverse(0), 0, true), (Reverse(0), 0, false)]);
        let mut visited = HashSet::new();
        let mut answer = vec![(i32::MAX, i32::MAX); n];
        answer[0] = (0, 0);

        for edge in &red_edges {
            red_nexts[edge[0] as usize].push(edge[1] as usize);
        }
        for edge in &blue_edges {
            blue_nexts[edge[0] as usize].push(edge[1] as usize);
        }

        while let Some((Reverse(length), node, is_red)) = nodes_heap.pop() {
            if visited.contains(&(node, is_red)) {
                continue;
            }

            visited.insert((node, is_red));

            if is_red {
                for &next in &blue_nexts[node] {
                    if answer[next].1 > length + 1 {
                        answer[next].1 = length + 1;
                        nodes_heap.push((Reverse(length + 1), next, false));
                    }
                }
            } else {
                for &next in &red_nexts[node] {
                    if answer[next].0 > length + 1 {
                        answer[next].0 = length + 1;
                        nodes_heap.push((Reverse(length + 1), next, true));
                    }
                }
            }
        }

        answer
            .into_iter()
            .map(|(red, blue)| {
                if red.min(blue) == i32::MAX {
                    -1
                } else {
                    red.min(blue)
                }
            })
            .collect()
    }
}
