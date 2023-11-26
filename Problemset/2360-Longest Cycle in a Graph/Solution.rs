use std::collections::HashMap;

impl Solution {
    pub fn longest_cycle(edges: Vec<i32>) -> i32 {
        let mut visited = vec![false; edges.len()];
        let mut nodes = HashMap::new();
        let mut ret = -1;

        for i in 0..edges.len() {
            if visited[i] {
                continue;
            }

            let mut i = i;
            let mut count = 1;
            nodes.clear();

            while edges[i] != -1 {
                if let Some(&x) = nodes.get(&i) {
                    ret = ret.max(count - x);
                    break;
                } else if visited[i] {
                    break;
                }

                visited[i] = true;
                nodes.insert(i, count);
                i = edges[i] as usize;
                count += 1;
            }
        }

        ret
    }
}
