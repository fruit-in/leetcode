impl Solution {
    pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();

        for i in (0..n).rev() {
            let mut indegree = vec![0; n];
            let mut children = vec![vec![]; n];
            let mut stack = vec![];
            let mut count = 1;

            for j in 0..n {
                if j != i {
                    let (u, v) = (edges[j][0] as usize - 1, edges[j][1] as usize - 1);
                    indegree[v] += 1;
                    children[u].push(v);

                    if indegree[v] > 1 {
                        indegree[0] = 0;
                        indegree[1] = 0;
                        break;
                    }
                }
            }

            for j in 0..n {
                if indegree[j] == 0 {
                    stack.push(j);
                }
                if stack.len() > 1 {
                    stack.clear();
                    break;
                }
            }

            while let Some(j) = stack.pop() {
                for &k in &children[j] {
                    indegree[k] -= 1;
                    if indegree[k] == 0 {
                        stack.push(k);
                        count += 1;
                    }
                }
            }

            if count == n {
                return edges[i].clone();
            }
        }

        unreachable!()
    }
}
