# 2508. Add Edges to Make Degrees of All Nodes Even
There is an **undirected** graph consisting of `n` nodes numbered from `1` to `n`. You are given the integer `n` and a **2D** array `edges` where <code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> indicates that there is an edge between nodes <code>a<sub>i</sub></code> and <code>b<sub>i</sub></code>. The graph can be disconnected.

You can add **at most** two additional edges (possibly none) to this graph so that there are no repeated edges and no self-loops.

Return `true` *if it is possible to make the degree of each node in the graph even, otherwise return* `false`.

The degree of a node is the number of edges connected to it.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/10/26/agraphdrawio.png)
<pre>
<strong>Input:</strong> n = 5, edges = [[1,2],[2,3],[3,4],[4,2],[1,4],[2,5]]
<strong>Output:</strong> true
<strong>Explanation:</strong> The above diagram shows a valid way of adding an edge.
Every node in the resulting graph is connected to an even number of edges.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/10/26/aagraphdrawio.png)
<pre>
<strong>Input:</strong> n = 4, edges = [[1,2],[3,4]]
<strong>Output:</strong> true
<strong>Explanation:</strong> The above diagram shows a valid way of adding two edges.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2022/10/26/aaagraphdrawio.png)
<pre>
<strong>Input:</strong> n = 4, edges = [[1,2],[1,3],[1,4]]
<strong>Output:</strong> false
<strong>Explanation:</strong> It is not possible to obtain a valid graph with adding at most 2 edges.
</pre>

#### Constraints:
* <code>3 <= n <= 10<sup>5</sup></code>
* <code>2 <= edges.length <= 10<sup>5</sup></code>
* `edges[i].length == 2`
* <code>1 <= a<sub>i</sub>, b<sub>i</sub> <= n</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>
* There are no repeated edges.

## Solutions (Rust)

### 1. Solution
```Rust
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
```
