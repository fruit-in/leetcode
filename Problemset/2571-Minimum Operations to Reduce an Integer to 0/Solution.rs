use std::collections::VecDeque;

impl Solution {
    pub fn min_operations(n: i32) -> i32 {
        let m = n.ilog2() + 1;
        let mut deque = VecDeque::from([(n as usize, 0)]);
        let mut visited = vec![false; (1 << m) + 1];
        visited[n as usize] = true;

        while let Some((x, steps)) = deque.pop_front() {
            if x == 0 {
                return steps;
            }

            for i in 0..=m {
                if x + (1 << i) < visited.len() && !visited[x + (1 << i)] {
                    deque.push_back((x + (1 << i), steps + 1));
                    visited[x + (1 << i)] = true;
                }
                if x >= 1 << i && !visited[x - (1 << i)] {
                    deque.push_back((x - (1 << i), steps + 1));
                    visited[x - (1 << i)] = true;
                }
            }
        }

        unreachable!()
    }
}
