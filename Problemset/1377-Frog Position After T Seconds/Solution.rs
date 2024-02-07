impl Solution {
    pub fn frog_position(n: i32, edges: Vec<Vec<i32>>, t: i32, target: i32) -> f64 {
        let n = n as usize;
        let mut parent = vec![0; n + 1];
        let mut children = vec![vec![]; n + 1];
        let mut nodes = vec![1];
        let mut curr = target as usize;
        let mut ret = 1.0;

        while let Some(node) = nodes.pop() {
            for edge in &edges {
                if edge[0] == node as i32 && edge[1] != parent[node] as i32 {
                    parent[edge[1] as usize] = node;
                    children[node].push(edge[1] as usize);
                    nodes.push(edge[1] as usize);
                } else if edge[1] == node as i32 && edge[0] != parent[node] as i32 {
                    parent[edge[0] as usize] = node;
                    children[node].push(edge[0] as usize);
                    nodes.push(edge[0] as usize);
                }
            }
        }

        for i in 0..=t {
            if parent[curr] == 0 {
                if i < t && !children[target as usize].is_empty() {
                    return 0.0;
                }

                return ret;
            }

            curr = parent[curr];
            ret /= children[curr].len() as f64;
        }

        0.0
    }
}
