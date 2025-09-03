# 1786. 从第一个节点出发到最后一个节点的受限路径数
现有一个加权无向连通图。给你一个正整数 `n` ，表示图中有 `n` 个节点，并按从 `1` 到 `n` 给节点编号；另给你一个数组 `edges` ，其中每个 <code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>, weight<sub>i</sub>]</code> 表示存在一条位于节点 <code>u<sub>i</sub></code> 和 <code>v<sub>i</sub></code> 之间的边，这条边的权重为 <code>weight<sub>i</sub></code> 。

从节点 `start` 出发到节点 `end` 的路径是一个形如 <code>[z<sub>0</sub>, z<sub>1</sub>, z<sub>2</sub>, ..., z<sub>k</sub>]</code> 的节点序列，满足 <code>z<sub>0</sub> = start</code> 、<code>z<sub>k</sub> = end</code> 且在所有符合 `0 <= i <= k-1` 的节点 <code>z<sub>i</sub></code> 和 <code>z<sub>i+1</sub></code> 之间存在一条边。

路径的距离定义为这条路径上所有边的权重总和。用 `distanceToLastNode(x)` 表示节点 `n` 和 `x` 之间路径的最短距离。**受限路径** 为满足 <code>distanceToLastNode(z<sub>i</sub>) > distanceToLastNode(z<sub>i+1</sub>)</code> 的一条路径，其中 `0 <= i <= k-1` 。

返回从节点 `1` 出发到节点 `n` 的 **受限路径数** 。由于数字可能很大，请返回对 <code>10<sup>9</sup> + 7</code> **取余** 的结果。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/02/17/restricted_paths_ex1.png)
<pre>
<strong>输入:</strong> n = 5, edges = [[1,2,3],[1,3,3],[2,3,1],[1,4,2],[5,2,2],[3,5,1],[5,4,10]]
<strong>输出:</strong> 3
<strong>解释:</strong> 每个圆包含黑色的节点编号和蓝色的 distanceToLastNode 值。三条受限路径分别是：
1) 1 --> 2 --> 5
2) 1 --> 2 --> 3 --> 5
3) 1 --> 3 --> 5
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/02/17/restricted_paths_ex22.png)
<pre>
<strong>输入:</strong> n = 7, edges = [[1,3,1],[4,1,2],[7,3,4],[2,5,3],[5,6,1],[6,7,2],[7,5,3],[2,6,4]]
<strong>输出:</strong> 1
<strong>解释:</strong> 每个圆包含黑色的节点编号和蓝色的 distanceToLastNode 值。唯一一条受限路径是：1 --> 3 --> 7 。
</pre>

#### 提示:
* <code>1 <= n <= 2 * 10<sup>4</sup></code>
* <code>n - 1 <= edges.length <= 4 * 10<sup>4</sup></code>
* `edges[i].length == 3`
* <code>1 <= u<sub>i</sub>, v<sub>i</sub> <= n</code>
* <code>u<sub>i</sub> != v<sub>i</sub></code>
* <code>1 <= weight<sub>i</sub> <= 10<sup>5</sup></code>
* 任意两个节点之间至多存在一条边
* 任意两个节点之间至少存在一条路径

## 题解 (Rust)

### 1. 题解
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
