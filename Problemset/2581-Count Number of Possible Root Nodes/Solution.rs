use std::collections::HashSet;

impl Solution {
    pub fn root_count(edges: Vec<Vec<i32>>, guesses: Vec<Vec<i32>>, k: i32) -> i32 {
        let guesses = guesses
            .iter()
            .map(|g| (g[0] as usize, g[1] as usize))
            .collect::<HashSet<_>>();
        let n = edges.len() + 1;
        let mut neighbors = vec![vec![]; n];
        let mut stack = vec![(n, 0)];
        let mut as_root = vec![0; n];

        for edge in &edges {
            neighbors[edge[0] as usize].push(edge[1] as usize);
            neighbors[edge[1] as usize].push(edge[0] as usize);
        }

        while let Some((p, a)) = stack.pop() {
            if guesses.contains(&(p, a)) {
                as_root[0] += 1;
            }

            for &b in &neighbors[a] {
                if b != p {
                    stack.push((a, b));
                }
            }
        }

        stack = vec![(n, 0)];

        while let Some((p, a)) = stack.pop() {
            for &b in &neighbors[a] {
                if b != p {
                    stack.push((a, b));
                    as_root[b] = as_root[a];
                    if guesses.contains(&(a, b)) {
                        as_root[b] -= 1;
                    }
                    if guesses.contains(&(b, a)) {
                        as_root[b] += 1;
                    }
                }
            }
        }

        as_root.iter().filter(|&&t| t >= k).count() as i32
    }
}
