use std::collections::BinaryHeap;
use std::collections::HashSet;

impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let n = n as usize;
        let mut to_cities = vec![vec![]; n];
        let mut min_reachable = usize::MAX;
        let mut ret = 0;

        for edge in &edges {
            if edge[2] <= distance_threshold {
                to_cities[edge[0] as usize].push((edge[1] as usize, edge[2]));
                to_cities[edge[1] as usize].push((edge[0] as usize, edge[2]));
            }
        }

        for i in 0..n {
            let mut heap = BinaryHeap::from([(0, i)]);
            let mut visited = HashSet::new();

            while let Some((weight, from)) = heap.pop() {
                visited.insert(from);

                for &(to, w) in &to_cities[from] {
                    if !visited.contains(&to) && -weight + w <= distance_threshold {
                        heap.push((weight - w, to));
                    }
                }
            }

            if visited.len() <= min_reachable {
                min_reachable = visited.len();
                ret = i;
            }
        }

        ret as i32
    }
}
