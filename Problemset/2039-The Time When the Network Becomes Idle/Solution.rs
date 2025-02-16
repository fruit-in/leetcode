use std::collections::VecDeque;

impl Solution {
    pub fn network_becomes_idle(edges: Vec<Vec<i32>>, patience: Vec<i32>) -> i32 {
        let mut neighbors = vec![vec![]; patience.len()];
        let mut deque = VecDeque::from([0]);
        let mut dists = vec![0; patience.len()];
        let mut ret = 0;

        for edge in &edges {
            neighbors[edge[0] as usize].push(edge[1] as usize);
            neighbors[edge[1] as usize].push(edge[0] as usize);
        }

        while let Some(server) = deque.pop_front() {
            for &neighbor in &neighbors[server] {
                if neighbor > 0 && dists[neighbor] == 0 {
                    dists[neighbor] = dists[server] + 1;
                    deque.push_back(neighbor);
                    ret = ret.max(
                        (2 * dists[neighbor] - 1) / patience[neighbor] * patience[neighbor]
                            + 2 * dists[neighbor]
                            + 1,
                    );
                }
            }
        }

        ret
    }
}
