# 684. Redundant Connection
In this problem, a tree is an **undirected graph** that is connected and has no cycles.

You are given a graph that started as a tree with `n` nodes labeled from `1` to `n`, with one additional edge added. The added edge has two **different** vertices chosen from `1` to `n`, and was not an edge that already existed. The graph is represented as an array `edges` of length `n` where <code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> indicates that there is an edge between nodes <code>a<sub>i</sub></code> and <code>b<sub>i</sub></code> in the graph.

Return *an edge that can be removed so that the resulting graph is a tree of* `n` *nodes*. If there are multiple answers, return the answer that occurs last in the input.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/05/02/reduntant1-1-graph.jpg)
<pre>
<strong>Input:</strong> edges = [[1,2],[1,3],[2,3]]
<strong>Output:</strong> [2,3]
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/05/02/reduntant1-2-graph.jpg)
<pre>
<strong>Input:</strong> edges = [[1,2],[2,3],[3,4],[1,4],[1,5]]
<strong>Output:</strong> [1,4]
</pre>

#### Constraints:
* `n == edges.length`
* `3 <= n <= 1000`
* `edges[i].length == 2`
* <code>1 <= a<sub>i</sub> < b<sub>i</sub> <= edges.length</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>
* There are no repeated edges.
* The given graph is connected.

## Solutions (Rust)

### 1. Solution
```Rust
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
```
