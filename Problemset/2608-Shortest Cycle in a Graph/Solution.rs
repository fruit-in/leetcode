use std::collections::HashMap;
use std::collections::VecDeque;

impl Solution {
    pub fn find_shortest_cycle(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut neighbors = vec![vec![]; n];
        let mut ret = n + 1;

        for edge in &edges {
            neighbors[edge[0] as usize].push(edge[1] as usize);
            neighbors[edge[1] as usize].push(edge[0] as usize);
        }

        for i in 0..n {
            let mut deque = VecDeque::from([(n, i)]);
            let mut distance = HashMap::from([(i, 0)]);

            while let Some((from, to)) = deque.pop_front() {
                if distance[&to] >= ret {
                    break;
                }

                for &j in &neighbors[to] {
                    if j == from {
                        continue;
                    }

                    if let Some(d) = distance.get(&j) {
                        ret = ret.min(d + distance[&to] + 1);
                    } else {
                        deque.push_back((to, j));
                        distance.insert(j, distance[&to] + 1);
                    }
                }
            }
        }

        if ret > n {
            return -1;
        }

        ret as i32
    }
}
