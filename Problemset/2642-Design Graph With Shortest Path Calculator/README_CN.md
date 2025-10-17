# 2642. 设计可以求最短路径的图类
给你一个有 `n` 个节点的 **有向带权** 图，节点编号为 `0` 到 `n - 1` 。图中的初始边用数组 `edges` 表示，其中 <code>edges[i] = [from<sub>i</sub>, to<sub>i</sub>, edgeCost<sub>i</sub>]</code> 表示从 <code>from<sub>i</sub></code> 到 <code>to<sub>i</sub></code> 有一条代价为 <code>edgeCost<sub>i</sub></code> 的边。

请你实现一个 `Graph` 类：
* `Graph(int n, int[][] edges)` 初始化图有 `n` 个节点，并输入初始边。
* `addEdge(int[] edge)` 向边集中添加一条边，其中 `edge = [from, to, edgeCost]` 。数据保证添加这条边之前对应的两个节点之间没有有向边。
* `int shortestPath(int node1, int node2)` 返回从节点 `node1` 到 `node2` 的路径 **最小** 代价。如果路径不存在，返回 `-1` 。一条路径的代价是路径中所有边代价之和。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2023/01/11/graph3drawio-2.png)
<pre>
<strong>输入:</strong>
["Graph", "shortestPath", "shortestPath", "addEdge", "shortestPath"]
[[4, [[0, 2, 5], [0, 1, 2], [1, 2, 1], [3, 0, 3]]], [3, 2], [0, 3], [[1, 3, 4]], [0, 3]]
<strong>输出:</strong>
[null, 6, -1, null, 6]
<strong>解释:</strong>
Graph g = new Graph(4, [[0, 2, 5], [0, 1, 2], [1, 2, 1], [3, 0, 3]]);
g.shortestPath(3, 2); // 返回 6 。从 3 到 2 的最短路径如第一幅图所示：3 -> 0 -> 1 -> 2 ，总代价为 3 + 2 + 1 = 6 。
g.shortestPath(0, 3); // 返回 -1 。没有从 0 到 3 的路径。
g.addEdge([1, 3, 4]); // 添加一条节点 1 到节点 3 的边，得到第二幅图。
g.shortestPath(0, 3); // 返回 6 。从 0 到 3 的最短路径为 0 -> 1 -> 3 ，总代价为 2 + 4 = 6 。
</pre>

#### 提示:
* `1 <= n <= 100`
* `0 <= edges.length <= n * (n - 1)`
* `edges[i].length == edge.length == 3`
* <code>0 <= from<sub>i</sub>, to<sub>i</sub>, from, to, node1, node2 <= n - 1</code>
* <code>1 <= edgeCost<sub>i</sub>, edgeCost <= 10<sup>6</sup></code>
* 图中任何时候都不会有重边和自环。
* 调用 `addEdge` 至多 `100` 次。
* 调用 `shortestPath` 至多 `100` 次。

## 题解 (Rust)

### 1. 题解
```Rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Graph {
    tos: Vec<Vec<(usize, i32)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Graph {
    fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        let mut tos = vec![vec![]; n as usize];

        for edge in &edges {
            tos[edge[0] as usize].push((edge[1] as usize, edge[2]));
        }

        Self { tos }
    }

    fn add_edge(&mut self, edge: Vec<i32>) {
        self.tos[edge[0] as usize].push((edge[1] as usize, edge[2]));
    }

    fn shortest_path(&self, node1: i32, node2: i32) -> i32 {
        let node1 = node1 as usize;
        let node2 = node2 as usize;
        let mut heap = BinaryHeap::from([(Reverse(0), node1)]);
        let mut min_cost = vec![i32::MAX; self.tos.len()];
        min_cost[node1] = 0;

        while let Some((Reverse(total_cost), from)) = heap.pop() {
            if from == node2 {
                return total_cost;
            } else if total_cost > min_cost[from] {
                continue;
            }

            for &(to, cost) in &self.tos[from] {
                if total_cost + cost < min_cost[to] {
                    min_cost[to] = total_cost + cost;
                    heap.push((Reverse(min_cost[to]), to));
                }
            }
        }

        -1
    }
}

/**
 * Your Graph object will be instantiated and called as such:
 * let obj = Graph::new(n, edges);
 * obj.add_edge(edge);
 * let ret_2: i32 = obj.shortest_path(node1, node2);
 */
```
