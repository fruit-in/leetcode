# 2608. Shortest Cycle in a Graph
There is a **bi-directional** graph with `n` vertices, where each vertex is labeled from `0` to `n - 1`. The edges in the graph are represented by a given 2D integer array `edges`, where <code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>]</code> denotes an edge between vertex <code>u<sub>i</sub></code> and vertex <code>v<sub>i</sub></code>. Every vertex pair is connected by at most one edge, and no vertex has an edge to itself.

Return *the length of the **shortest** cycle in the graph*. If no cycle exists, return `-1`.

A cycle is a path that starts and ends at the same node, and each edge in the path is used only once.

#### Example 1:
![](https://assets.leetcode.com/uploads/2023/01/04/cropped.png)
<pre>
<strong>Input:</strong> n = 7, edges = [[0,1],[1,2],[2,0],[3,4],[4,5],[5,6],[6,3]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The cycle with the smallest length is : 0 -> 1 -> 2 -> 0
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2023/01/04/croppedagin.png)
<pre>
<strong>Input:</strong> n = 4, edges = [[0,1],[0,2]]
<strong>Output:</strong> -1
<strong>Explanation:</strong> There are no cycles in this graph.
</pre>

#### Constraints:
* `2 <= n <= 1000`
* `1 <= edges.length <= 1000`
* `edges[i].length == 2`
* <code>0 <= u<sub>i</sub>, v<sub>i</sub> < n</code>
* <code>u<sub>i</sub> != v<sub>i</sub></code>
* There are no repeated edges.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;
use std::collections::VecDeque;

impl Solution {
    pub fn find_shortest_cycle(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut neighbors = vec![vec![]; n];
        let mut ret = n + 1;

        for edge in &edges {
            neighbors[edge[0] as usize].push(edge[1] as usize);
            neighbors[edge[1] as usize].push(edge[0] as usize);
        }

        for i in 0..n {
            let mut deque = VecDeque::from([(n, i)]);
            let mut distance = HashMap::from([(i, 0)]);

            while let Some((from, to)) = deque.pop_front() {
                if distance[&to] >= ret {
                    break;
                }

                for &j in &neighbors[to] {
                    if j == from {
                        continue;
                    }

                    if let Some(d) = distance.get(&j) {
                        ret = ret.min(d + distance[&to] + 1);
                    } else {
                        deque.push_back((to, j));
                        distance.insert(j, distance[&to] + 1);
                    }
                }
            }
        }

        if ret > n {
            return -1;
        }

        ret as i32
    }
}
```
