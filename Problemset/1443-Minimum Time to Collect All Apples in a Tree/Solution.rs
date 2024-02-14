use std::collections::HashSet;

impl Solution {
    pub fn min_time(n: i32, edges: Vec<Vec<i32>>, mut has_apple: Vec<bool>) -> i32 {
        let n = n as usize;
        let mut children = vec![HashSet::new(); n];
        let mut parent = vec![n; n];
        let mut nodes = vec![0];

        for edge in &edges {
            children[edge[0] as usize].insert(edge[1] as usize);
            children[edge[1] as usize].insert(edge[0] as usize);
        }

        while let Some(node) = nodes.pop() {
            children[node].remove(&parent[node]);

            for &child in children[node].iter() {
                parent[child] = node;
                nodes.push(child);
            }
        }

        for node in 0..n {
            if children[node].is_empty() {
                nodes.push(node);
            }
        }

        while let Some(node) = nodes.pop() {
            if node == 0 {
                break;
            }

            has_apple[parent[node]] |= has_apple[node];
            children[parent[node]].remove(&node);
            if children[parent[node]].is_empty() {
                nodes.push(parent[node]);
            }
        }

        ((0..n).filter(|&node| has_apple[node]).count() as i32 - 1).max(0) * 2
    }
}
