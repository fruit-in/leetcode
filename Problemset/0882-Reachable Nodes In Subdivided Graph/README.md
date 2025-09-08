# 882. Reachable Nodes In Subdivided Graph
You are given an undirected graph (the **"original graph"**) with `n` nodes labeled from `0` to `n - 1`. You decide to **subdivide** each edge in the graph into a chain of nodes, with the number of new nodes varying between each edge.

The graph is given as a 2D array of `edges` where <code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>, cnt<sub>i</sub>]</code> indicates that there is an edge between nodes <code>u<sub>i</sub></code> and <code>v<sub>i</sub></code> in the original graph, and <code>cnt<sub>i</sub></code> is the total number of new nodes that you will **subdivide** the edge into. Note that <code>cnt<sub>i</sub> == 0</code> means you will not subdivide the edge.

To **subdivide** the edge <code>[u<sub>i</sub>, v<sub>i</sub>]</code>, replace it with <code>(cnt<sub>i</sub> + 1)</code> new edges and <code>cnt<sub>i</sub></code> new nodes. The new nodes are <code>x<sub>1</sub></code>, <code>x<sub>2</sub></code>, ..., <code>x<sub>cnt<sub>i</sub></sub></code>, and the new edges are <code>[u<sub>i</sub>, x<sub>1</sub>]</code>, <code>[x<sub>1</sub>, x<sub>2</sub>]</code>, <code>[x<sub>2</sub>, x<sub>3</sub>]</code>, ..., <code>[x<sub>cnt<sub>i</sub>-1</sub>, x<sub>cnt<sub>i</sub></sub>]</code>, <code>[x<sub>cnt<sub>i</sub></sub>, v<sub>i</sub>]</code>.

In this **new graph**, you want to know how many nodes are **reachable** from the node `0`, where a node is **reachable** if the distance is `maxMoves` or less.

Given the original graph and `maxMoves`, return *the number of nodes that are **reachable** from node* `0` *in the new graph*.

#### Example 1:
![](https://s3-lc-upload.s3.amazonaws.com/uploads/2018/08/01/origfinal.png)
<pre>
<strong>Input:</strong> edges = [[0,1,10],[0,2,1],[1,2,2]], maxMoves = 6, n = 3
<strong>Output:</strong> 13
<strong>Explanation:</strong> The edge subdivisions are shown in the image above.
The nodes that are reachable are highlighted in yellow.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> edges = [[0,1,4],[1,2,6],[0,2,8],[1,3,1]], maxMoves = 10, n = 4
<strong>Output:</strong> 23
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> edges = [[1,2,4],[1,4,5],[1,3,1],[2,3,4],[3,4,5]], maxMoves = 17, n = 5
<strong>Output:</strong> 1
<strong>Explanation:</strong> Node 0 is disconnected from the rest of the graph, so only node 0 is reachable.
</pre>

#### Constraints:
* <code>0 <= edges.length <= min(n * (n - 1) / 2, 10<sup>4</sup>)</code>
* `edges[i].length == 3`
* <code>0 <= u<sub>i</sub> < v<sub>i</sub> < n</code>
* There are **no multiple edges** in the graph.
* <code>0 <= cnt<sub>i</sub> <= 10<sup>4</sup></code>
* <code>0 <= maxMoves <= 10<sup>9</sup></code>
* `1 <= n <= 3000`

## Solutions (Rust)

### 1. Solution
```Rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn reachable_nodes(edges: Vec<Vec<i32>>, max_moves: i32, n: i32) -> i32 {
        let n = n as usize;
        let mut neighbors = vec![vec![]; n];
        let mut heap = BinaryHeap::from([(Reverse(0), 0)]);
        let mut min_moves = vec![i32::MAX; n];
        let mut ret = 0;
        min_moves[0] = 0;

        for edge in &edges {
            let (u, v, dist) = (edge[0] as usize, edge[1] as usize, edge[2] + 1);
            neighbors[u].push((v, dist));
            neighbors[v].push((u, dist));
        }

        while let Some((Reverse(moves), u)) = heap.pop() {
            if moves > min_moves[u] {
                continue;
            }

            ret += 1;

            for &(v, dist) in &neighbors[u] {
                if moves + dist <= max_moves.min(min_moves[v] - 1) {
                    min_moves[v] = moves + dist;
                    heap.push((Reverse(min_moves[v]), v));
                }
            }
        }

        for edge in &edges {
            let (u, v, cnt) = (edge[0] as usize, edge[1] as usize, edge[2]);
            ret += cnt.min((max_moves - min_moves[u]).max(0) + (max_moves - min_moves[v]).max(0));
        }

        ret
    }
}
```
