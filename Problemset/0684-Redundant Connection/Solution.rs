impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();

        for i in (0..n).rev() {
            let mut neighbors = vec![vec![]; n];
            let mut stack = vec![(n, 0)];
            let mut visited = vec![false; n];
            visited[0] = true;

            for j in 0..n {
                if j != i {
                    let (a, b) = (edges[j][0] as usize - 1, edges[j][1] as usize - 1);
                    neighbors[a].push(b);
                    neighbors[b].push(a);
                }
            }

            while let Some((from, to)) = stack.pop() {
                for &j in &neighbors[to] {
                    if j != from {
                        if visited[j] {
                            stack = vec![];
                            visited[0] = false;
                            break;
                        } else {
                            stack.push((to, j));
                            visited[j] = true;
                        }
                    }
                }
            }

            if visited.iter().all(|&v| v) {
                return edges[i].clone();
            }
        }

        unreachable!()
    }
}
