# 2203. Minimum Weighted Subgraph With the Required Paths
You are given an integer `n` denoting the number of nodes of a **weighted directed** graph. The nodes are numbered from `0` to `n - 1`.

You are also given a 2D integer array `edges` where <code>edges[i] = [from<sub>i</sub>, to<sub>i</sub>, weight<sub>i</sub>]</code> denotes that there exists a **directed** edge from <code>from<sub>i</sub></code> to <code>to<sub>i</sub></code> with weight <code>weight<sub>i</sub></code>.

Lastly, you are given three **distinct** integers `src1`, `src2`, and `dest` denoting three distinct nodes of the graph.

Return *the **minimum weight** of a subgraph of the graph such that it is **possible** to reach* `dest` *from both* `src1` *and* `src2` *via a set of edges of this subgraph*. In case such a subgraph does not exist, return `-1`.

A **subgraph** is a graph whose vertices and edges are subsets of the original graph. The **weight** of a subgraph is the sum of weights of its constituent edges.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/02/17/example1drawio.png)
<pre>
<strong>Input:</strong> n = 6, edges = [[0,2,2],[0,5,6],[1,0,3],[1,4,5],[2,1,1],[2,3,3],[2,3,4],[3,4,2],[4,5,1]], src1 = 0, src2 = 1, dest = 5
<strong>Output:</strong> 9
<strong>Explanation:</strong>
The above figure represents the input graph.
The blue edges represent one of the subgraphs that yield the optimal answer.
Note that the subgraph [[1,0,3],[0,5,6]] also yields the optimal answer. It is not possible to get a subgraph with less weight satisfying all the constraints.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/02/17/example2-1drawio.png)
<pre>
<strong>Input:</strong> n = 3, edges = [[0,1,1],[2,1,1]], src1 = 0, src2 = 1, dest = 2
<strong>Output:</strong> -1
<strong>Explanation:</strong>
The above figure represents the input graph.
It can be seen that there does not exist any path from node 1 to node 2, hence there are no subgraphs satisfying all the constraints.
</pre>

#### Constraints:
* <code>3 <= n <= 10<sup>5</sup></code>
* <code>0 <= edges.length <= 10<sup>5</sup></code>
* `edges[i].length == 3`
* <code>0 <= from<sub>i</sub>, to<sub>i</sub>, src1, src2, dest <= n - 1</code>
* <code>from<sub>i</sub> != to<sub>i</sub></code>
* `src1`, `src2`, and `dest` are pairwise distinct.
* <code>1 <= weight[i] <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn minimum_weight(n: i32, edges: Vec<Vec<i32>>, src1: i32, src2: i32, dest: i32) -> i64 {
        let n = n as usize;
        let src1 = src1 as usize;
        let src2 = src2 as usize;
        let dest = dest as usize;
        let mut froms = vec![vec![]; n];
        let mut tos = vec![vec![]; n];
        let mut from_src1 = vec![-1; n];
        let mut from_src2 = vec![-1; n];
        let mut to_dest = vec![-1; n];
        from_src1[src1] = 0;
        from_src2[src2] = 0;
        to_dest[dest] = 0;

        for edge in &edges {
            let (from, to, weight) = (edge[0] as usize, edge[1] as usize, edge[2] as i64);
            froms[to].push((from, weight));
            tos[from].push((to, weight));
        }

        let mut heap = BinaryHeap::from([(Reverse(0), src1)]);
        while let Some((Reverse(weights), from)) = heap.pop() {
            if weights > from_src1[from] {
                continue;
            }

            for &(to, weight) in &tos[from] {
                if from_src1[to] == -1 || weights + weight < from_src1[to] {
                    from_src1[to] = weights + weight;
                    heap.push((Reverse(from_src1[to]), to));
                }
            }
        }

        heap = BinaryHeap::from([(Reverse(0), src2)]);
        while let Some((Reverse(weights), from)) = heap.pop() {
            if weights > from_src2[from] {
                continue;
            }

            for &(to, weight) in &tos[from] {
                if from_src2[to] == -1 || weights + weight < from_src2[to] {
                    from_src2[to] = weights + weight;
                    heap.push((Reverse(from_src2[to]), to));
                }
            }
        }

        heap = BinaryHeap::from([(Reverse(0), dest)]);
        while let Some((Reverse(weights), to)) = heap.pop() {
            if weights > to_dest[to] {
                continue;
            }

            for &(from, weight) in &froms[to] {
                if to_dest[from] == -1 || weights + weight < to_dest[from] {
                    to_dest[from] = weights + weight;
                    heap.push((Reverse(to_dest[from]), from));
                }
            }
        }

        (0..n)
            .filter(|&i| from_src1[i] >= 0 && from_src2[i] >= 0 && to_dest[i] >= 0)
            .map(|i| from_src1[i] + from_src2[i] + to_dest[i])
            .min()
            .unwrap_or(-1)
    }
}
```
