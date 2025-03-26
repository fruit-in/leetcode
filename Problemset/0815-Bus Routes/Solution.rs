use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target {
            return 0;
        }

        let mut routes = routes
            .into_iter()
            .map(|route| route.into_iter().collect::<HashSet<_>>())
            .collect::<Vec<_>>();
        let mut deque = VecDeque::new();
        let mut visited = HashSet::new();
        let mut neighbors = vec![vec![]; routes.len()];

        for i in 0..routes.len() {
            if routes[i].contains(&source) {
                deque.push_back((i, 1));
                visited.insert(i);
            }

            for j in i + 1..routes.len() {
                for stop in routes[i].iter() {
                    if routes[j].contains(stop) {
                        neighbors[i].push(j);
                        neighbors[j].push(i);
                        break;
                    }
                }
            }
        }

        while let Some((i, buses)) = deque.pop_front() {
            if routes[i].contains(&target) {
                return buses;
            }

            for &j in &neighbors[i] {
                if !visited.contains(&j) {
                    deque.push_back((j, buses + 1));
                    visited.insert(j);
                }
            }
        }

        -1
    }
}
