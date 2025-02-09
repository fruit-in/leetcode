impl Solution {
    pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
        use std::collections::HashMap;
        use std::collections::VecDeque;

        let n = n as usize;
        let mut neighbors = vec![vec![]; n + 1];
        let mut visited = HashMap::from([(1, 0)]);
        let mut deque = VecDeque::from([(1, 0)]);
        let mut min_step = i32::MAX;
        let mut smin_step = i32::MAX;
        let mut ret = 0;

        for edge in &edges {
            neighbors[edge[0] as usize].push(edge[1] as usize);
            neighbors[edge[1] as usize].push(edge[0] as usize);
        }

        while let Some((u, step)) = deque.pop_front() {
            if u == n {
                if step < min_step {
                    min_step = step;
                    smin_step = step + 2;
                } else if step > min_step {
                    smin_step = step;
                }
            }

            if step >= smin_step {
                break;
            }

            for &v in &neighbors[u] {
                if !visited.contains_key(&v) {
                    visited.insert(v, step + 1);
                    deque.push_back((v, step + 1));
                } else if visited[&v] == step {
                    visited.insert(v, -1);
                    deque.push_back((v, step + 1));
                }
            }
        }

        for _ in 0..smin_step {
            if ret % (2 * change) >= change {
                ret = (ret / (2 * change) + 1) * 2 * change;
            }

            ret += time;
        }

        ret
    }
}
