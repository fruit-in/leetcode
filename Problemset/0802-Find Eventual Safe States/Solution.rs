impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let n = graph.len();
        let mut graph_rev = vec![vec![]; n];
        let mut outdegree = vec![0; n];
        let mut stack = vec![];
        let mut ret = vec![];

        for i in 0..n {
            for &j in &graph[i] {
                graph_rev[j as usize].push(i);
            }
            outdegree[i] = graph[i].len();
            if outdegree[i] == 0 {
                stack.push(i);
            }
        }

        while let Some(i) = stack.pop() {
            ret.push(i as i32);

            for &j in &graph_rev[i] {
                outdegree[j] -= 1;
                if outdegree[j] == 0 {
                    stack.push(j);
                }
            }
        }

        ret.sort_unstable();

        ret
    }
}
