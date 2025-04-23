use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len() as i32;
        let mut deque = (0..n).map(|i| (i, 1 << i, 0)).collect::<VecDeque<_>>();
        let mut visited = (0..n).map(|i| (i, 1 << i)).collect::<HashSet<_>>();

        while let Some((i, mask, step)) = deque.pop_front() {
            if mask == (1 << n) - 1 {
                return step;
            }

            for &j in &graph[i as usize] {
                if !visited.contains(&(j, mask | (1 << j))) {
                    deque.push_back((j, mask | (1 << j), step + 1));
                    visited.insert((j, mask | (1 << j)));
                }
            }
        }

        unreachable!()
    }
}
