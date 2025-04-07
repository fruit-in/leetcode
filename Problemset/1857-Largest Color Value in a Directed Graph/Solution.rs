impl Solution {
    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        let colors = colors
            .bytes()
            .map(|c| (c - b'a') as usize)
            .collect::<Vec<_>>();
        let n = colors.len();
        let mut indegree = vec![0; n];
        let mut next_nodes = vec![vec![]; n];
        let mut nodes = vec![];
        let mut node_colors = vec![[0; 26]; n];
        let mut count = 0;

        for edge in &edges {
            indegree[edge[1] as usize] += 1;
            next_nodes[edge[0] as usize].push(edge[1] as usize);
        }

        for i in 0..n {
            if indegree[i] == 0 {
                nodes.push(i);
            }
            node_colors[i][colors[i]] += 1;
        }

        while let Some(i) = nodes.pop() {
            count += 1;

            for &j in &next_nodes[i] {
                indegree[j] -= 1;
                if indegree[j] == 0 {
                    nodes.push(j);
                }

                for k in 0..26 {
                    if k != colors[j] {
                        node_colors[j][k] = node_colors[j][k].max(node_colors[i][k]);
                    } else {
                        node_colors[j][k] = node_colors[j][k].max(node_colors[i][k] + 1);
                    }
                }
            }
        }

        if count < n {
            return -1;
        }

        (0..n).map(|i| node_colors[i][colors[i]]).max().unwrap()
    }
}
