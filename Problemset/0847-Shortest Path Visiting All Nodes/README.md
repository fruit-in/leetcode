# 847. Shortest Path Visiting All Nodes
You have an undirected, connected graph of `n` nodes labeled from `0` to `n - 1`. You are given an array `graph` where `graph[i]` is a list of all the nodes connected with node `i` by an edge.

Return *the length of the shortest path that visits every node*. You may start and stop at any node, you may revisit nodes multiple times, and you may reuse edges.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/05/12/shortest1-graph.jpg)
<pre>
<strong>Input:</strong> graph = [[1,2,3],[0],[0],[0]]
<strong>Output:</strong> 4
<strong>Explanation:</strong> One possible path is [1,0,2,0,3]
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/05/12/shortest2-graph.jpg)
<pre>
<strong>Input:</strong> graph = [[1],[0,2,4],[1,3,4],[2],[1,2]]
<strong>Output:</strong> 4
<strong>Explanation:</strong> One possible path is [0,1,4,2,3]
</pre>

#### Constraints:
* `n == graph.length`
* `1 <= n <= 12`
* `0 <= graph[i].length < n`
* `graph[i]` does not contain `i`.
* If `graph[a]` contains `b`, then `graph[b]` contains `a`.
* The input graph is always connected.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len() as i32;
        let mut deque = (0..n).map(|i| (i, 1 << i, 0)).collect::<VecDeque<_>>();
        let mut visited = (0..n).map(|i| (i, 1 << i)).collect::<HashSet<_>>();

        while let Some((i, mask, step)) = deque.pop_front() {
            if mask == (1 << n) - 1 {
                return step;
            }

            for &j in &graph[i as usize] {
                if !visited.contains(&(j, mask | (1 << j))) {
                    deque.push_back((j, mask | (1 << j), step + 1));
                    visited.insert((j, mask | (1 << j)));
                }
            }
        }

        unreachable!()
    }
}
```
