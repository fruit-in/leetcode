# 802. Find Eventual Safe States
There is a directed graph of `n` nodes with each node labeled from `0` to `n - 1`. The graph is represented by a **0-indexed** 2D integer array `graph` where `graph[i]` is an integer array of nodes adjacent to node `i`, meaning there is an edge from node `i` to each node in `graph[i]`.

A node is a **terminal node** if there are no outgoing edges. A node is a **safe node** if every possible path starting from that node leads to a **terminal node** (or another safe node).

Return *an array containing all the **safe nodes** of the graph*. The answer should be sorted in **ascending** order.

#### Example 1:
![](https://s3-lc-upload.s3.amazonaws.com/uploads/2018/03/17/picture1.png)
<pre>
<strong>Input:</strong> graph = [[1,2],[2,3],[5],[0],[5],[],[]]
<strong>Output:</strong> [2,4,5,6]
<strong>Explanation:</strong> The given graph is shown above.
Nodes 5 and 6 are terminal nodes as there are no outgoing edges from either of them.
Every path starting at nodes 2, 4, 5, and 6 all lead to either node 5 or 6.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> graph = [[1,2,3,4],[1,2],[3,4],[0,4],[]]
<strong>Output:</strong> [4]
<strong>Explanation:</strong>
Only node 4 is a terminal node, and every path starting at node 4 leads to node 4.
</pre>

#### Constraints:
* `n == graph.length`
* <code>1 <= n <= 10<sup>4</sup></code>
* `0 <= graph[i].length <= n`
* `0 <= graph[i][j] <= n - 1`
* `graph[i]` is sorted in a strictly increasing order.
* The graph may contain self-loops.
* The number of edges in the graph will be in the range <code>[1, 4 * 10<sup>4</sup>]</code>.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let n = graph.len();
        let mut graph_rev = vec![vec![]; n];
        let mut outdegree = vec![0; n];
        let mut stack = vec![];
        let mut ret = vec![];

        for i in 0..n {
            for &j in &graph[i] {
                graph_rev[j as usize].push(i);
            }
            outdegree[i] = graph[i].len();
            if outdegree[i] == 0 {
                stack.push(i);
            }
        }

        while let Some(i) = stack.pop() {
            ret.push(i as i32);

            for &j in &graph_rev[i] {
                outdegree[j] -= 1;
                if outdegree[j] == 0 {
                    stack.push(j);
                }
            }
        }

        ret.sort_unstable();

        ret
    }
}
```
