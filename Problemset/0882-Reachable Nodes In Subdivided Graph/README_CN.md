# 882. 细分图中的可到达节点
给你一个无向图（**原始图**），图中有 `n` 个节点，编号从 `0` 到 `n - 1` 。你决定将图中的每条边 **细分** 为一条节点链，每条边之间的新节点数各不相同。

图用由边组成的二维数组 `edges` 表示，其中 <code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>, cnt<sub>i</sub>]</code> 表示原始图中节点 <code>u<sub>i</sub></code> 和 <code>v<sub>i</sub></code> 之间存在一条边，<code>cnt<sub>i</sub></code> 是将边 **细分** 后的新节点总数。注意，<code>cnt<sub>i</sub> == 0</code> 表示边不可细分。

要 **细分** 边 <code>[u<sub>i</sub>, v<sub>i</sub>]</code> ，需要将其替换为 <code>(cnt<sub>i</sub> + 1)</code> 条新边，和 <code>cnt<sub>i</sub></code> 个新节点。新节点为 <code>x<sub>1</sub></code>, <code>x<sub>2</sub></code>, ..., <code>x<sub>cnt<sub>i</sub></sub></code> ，新边为 <code>[u<sub>i</sub>, x<sub>1</sub>]</code>, <code>[x<sub>1</sub>, x<sub>2</sub>]</code>, <code>[x<sub>2</sub>, x<sub>3</sub>]</code>, ..., <code>[x<sub>cnt<sub>i</sub>-1</sub>, x<sub>cnt<sub>i</sub></sub>]</code>, <code>[x<sub>cnt<sub>i</sub></sub>, v<sub>i</sub>]</code> 。

现在得到一个 **新的细分图** ，请你计算从节点 `0` 出发，可以到达多少个节点？如果节点间距离是 `maxMoves` 或更少，则视为 **可以到达** 。

给你原始图和 `maxMoves` ，返回 *新的细分图中从节点 `0` 出发 **可到达的节点数*** 。

#### 示例 1:
![](https://s3-lc-upload.s3.amazonaws.com/uploads/2018/08/01/origfinal.png)
<pre>
<strong>输入:</strong> edges = [[0,1,10],[0,2,1],[1,2,2]], maxMoves = 6, n = 3
<strong>输出:</strong> 13
<strong>解释:</strong> 边的细分情况如上图所示。
可以到达的节点已经用黄色标注出来。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> edges = [[0,1,4],[1,2,6],[0,2,8],[1,3,1]], maxMoves = 10, n = 4
<strong>输出:</strong> 23
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> edges = [[1,2,4],[1,4,5],[1,3,1],[2,3,4],[3,4,5]], maxMoves = 17, n = 5
<strong>输出:</strong> 1
<strong>解释:</strong> 节点 0 与图的其余部分没有连通，所以只有节点 0 可以到达。
</pre>

#### 提示:
* <code>0 <= edges.length <= min(n * (n - 1) / 2, 10<sup>4</sup>)</code>
* `edges[i].length == 3`
* <code>0 <= u<sub>i</sub> < v<sub>i</sub> < n</code>
* 图中 **不存在平行边**
* <code>0 <= cnt<sub>i</sub> <= 10<sup>4</sup></code>
* <code>0 <= maxMoves <= 10<sup>9</sup></code>
* `1 <= n <= 3000`

## 题解 (Rust)

### 1. 题解
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
