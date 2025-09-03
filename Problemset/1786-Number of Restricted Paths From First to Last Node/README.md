# 1786. Number of Restricted Paths From First to Last Node
There is an undirected weighted connected graph. You are given a positive integer `n` which denotes that the graph has `n` nodes labeled from `1` to `n`, and an array `edges` where each <code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>, weight<sub>i</sub>]<code> denotes that there is an edge between nodes <code>u<sub>i</sub><code> and <code>v<sub>i</sub><code> with weight equal to <code>weight<sub>i</sub><code>.

A path from node `start` to node `end` is a sequence of nodes <code>[z<sub>0</sub>, z<sub>1</sub>, z<sub>2</sub>, ..., z<sub>k</sub>]</code> such that <code>z<sub>0</sub> = start</code> and <code>z<sub>k</sub> = end</code> and there is an edge between <code>z<sub>i</sub></code> and <code>z<sub>i+1</sub></code> where `0 <= i <= k-1`.

The distance of a path is the sum of the weights on the edges of the path. Let `distanceToLastNode(x)` denote the shortest distance of a path between node `n` and node `x`. A **restricted path** is a path that also satisfies that <code>distanceToLastNode(z<sub>i</sub>) > distanceToLastNode(z<sub>i+1</sub>)</code> where `0 <= i <= k-1`.

Return *the number of restricted paths from node* `1` *to node* `n`. Since that number may be too large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/02/17/restricted_paths_ex1.png)
<pre>
<strong>Input:</strong> n = 5, edges = [[1,2,3],[1,3,3],[2,3,1],[1,4,2],[5,2,2],[3,5,1],[5,4,10]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> Each circle contains the node number in black and its distanceToLastNode value in blue. The three restricted paths are:
1) 1 --> 2 --> 5
2) 1 --> 2 --> 3 --> 5
3) 1 --> 3 --> 5
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/02/17/restricted_paths_ex22.png)
<pre>
<strong>Input:</strong> n = 7, edges = [[1,3,1],[4,1,2],[7,3,4],[2,5,3],[5,6,1],[6,7,2],[7,5,3],[2,6,4]]
<strong>Output:</strong> 1
<strong>Explanation:</strong> Each circle contains the node number in black and its distanceToLastNode value in blue. The only restricted path is 1 --> 3 --> 7.
</pre>

#### Constraints:
* <code>1 <= n <= 2 * 10<sup>4</sup></code>
* <code>n - 1 <= edges.length <= 4 * 10<sup>4</sup></code>
* `edges[i].length == 3`
* <code>1 <= u<sub>i</sub>, v<sub>i</sub> <= n</code>
* <code>u<sub>i</sub> != v<sub>i</sub></code>
* <code>1 <= weight<sub>i</sub> <= 10<sup>5</sup></code>
* There is at most one edge between any two nodes.
* There is at least one path between any two nodes.

## Solutions (Rust)

### 1. Solution
```Rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn count_restricted_paths(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut neighbors = vec![vec![]; n + 1];
        let mut min_heap = BinaryHeap::from([(Reverse(0), n)]);
        let mut max_heap = BinaryHeap::new();
        let mut distance_to_last_node = vec![i64::MAX; n + 1];
        let mut paths = vec![0; n + 1];
        distance_to_last_node[n] = 0;
        paths[1] = 1;

        for edge in &edges {
            let (u, v, weight) = (edge[0] as usize, edge[1] as usize, edge[2] as i64);
            neighbors[u].push((v, weight));
            neighbors[v].push((u, weight));
        }

        while let Some((Reverse(dist), u)) = min_heap.pop() {
            if dist > distance_to_last_node[u] {
                continue;
            }

            max_heap.push((distance_to_last_node[u], u));

            for &(v, weight) in &neighbors[u] {
                if dist + weight < distance_to_last_node[v] {
                    distance_to_last_node[v] = dist + weight;
                    min_heap.push((Reverse(distance_to_last_node[v]), v));
                }
            }

            if u == 1 {
                break;
            }
        }

        while let Some((dist, u)) = max_heap.pop() {
            for &(v, _) in &neighbors[u] {
                if distance_to_last_node[v] > dist {
                    paths[u] = (paths[u] + paths[v]) % 1_000_000_007;
                }
            }
        }

        paths[n]
    }
}
```
