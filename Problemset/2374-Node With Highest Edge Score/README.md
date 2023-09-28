# 2374. Node With Highest Edge Score
You are given a directed graph with `n` nodes labeled from `0` to `n - 1`, where each node has **exactly one** outgoing edge.

The graph is represented by a given **0-indexed** integer array `edges` of length `n`, where `edges[i]` indicates that there is a **directed** edge from node `i` to node `edges[i]`.

The **edge score** of a node `i` is defined as the sum of the **labels** of all the nodes that have an edge pointing to `i`.

Return *the node with the highest **edge score***. If multiple nodes have the same **edge score**, return the node with the **smallest** index.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/06/20/image-20220620195403-1.png)
<pre>
<strong>Input:</strong> edges = [1,0,0,0,0,7,7,5]
<strong>Output:</strong> 7
<strong>Explanation:</strong>
- The nodes 1, 2, 3 and 4 have an edge pointing to node 0. The edge score of node 0 is 1 + 2 + 3 + 4 = 10.
- The node 0 has an edge pointing to node 1. The edge score of node 1 is 0.
- The node 7 has an edge pointing to node 5. The edge score of node 5 is 7.
- The nodes 5 and 6 have an edge pointing to node 7. The edge score of node 7 is 5 + 6 = 11.
Node 7 has the highest edge score so return 7.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/06/20/image-20220620200212-3.png)
<pre>
<strong>Input:</strong> edges = [2,0,0,2]
<strong>Output:</strong> 0
<strong>Explanation:</strong>
- The nodes 1 and 2 have an edge pointing to node 0. The edge score of node 0 is 1 + 2 = 3.
- The nodes 0 and 3 have an edge pointing to node 2. The edge score of node 2 is 0 + 3 = 3.
Nodes 0 and 2 both have an edge score of 3. Since node 0 has a smaller index, we return 0.
</pre>

#### Constraints:
* `n == edges.length`
* <code>2 <= n <= 10<sup>5</sup></code>
* `0 <= edges[i] < n`
* `edges[i] != i`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn edge_score(edges: Vec<i32>) -> i32 {
        let mut scores = vec![0; edges.len()];

        for i in 0..edges.len() {
            scores[edges[i] as usize] += i as i64;
        }

        (0..edges.len())
            .max_by_key(|&i| (scores[i], -(i as i32)))
            .unwrap_or(0) as i32
    }
}
```