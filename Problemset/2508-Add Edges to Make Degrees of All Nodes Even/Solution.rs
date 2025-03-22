use std::collections::HashSet;

impl Solution {
    pub fn is_possible(n: i32, edges: Vec<Vec<i32>>) -> bool {
        let mut edges_set = HashSet::new();
        let mut is_odd_degree = vec![false; n as usize + 1];
        let mut odd_degree = vec![];

        for edge in &edges {
            let (a, b) = (edge[0] as usize, edge[1] as usize);

            edges_set.insert((a.min(b), a.max(b)));
            is_odd_degree[a] = !is_odd_degree[a];
            is_odd_degree[b] = !is_odd_degree[b];
        }

        for i in 1..=n as usize {
            if is_odd_degree[i] {
                odd_degree.push(i);
            }
        }

        if odd_degree.len() == 0 {
            return true;
        } else if odd_degree.len() == 2 {
            let (a, b) = (odd_degree[0], odd_degree[1]);

            if !edges_set.contains(&(a, b)) {
                return true;
            }

            for c in 1..=n as usize {
                if !edges_set.contains(&(a.min(c), a.max(c)))
                    && !edges_set.contains(&(b.min(c), b.max(c)))
                {
                    return true;
                }
            }
        } else if odd_degree.len() == 4 {
            let (a, b, c, d) = (odd_degree[0], odd_degree[1], odd_degree[2], odd_degree[3]);

            if !edges_set.contains(&(a, b)) && !edges_set.contains(&(c, d)) {
                return true;
            }
            if !edges_set.contains(&(a, c)) && !edges_set.contains(&(b, d)) {
                return true;
            }
            if !edges_set.contains(&(a, d)) && !edges_set.contains(&(b, c)) {
                return true;
            }
        }

        false
    }
}
