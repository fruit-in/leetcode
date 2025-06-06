# 2497. 图中最大星和
给你一个 `n` 个点的无向图，节点从 `0` 到 `n - 1` 编号。给你一个长度为 `n` 下标从 **0** 开始的整数数组 `vals` ，其中 `vals[i]` 表示第 `i` 个节点的值。

同时给你一个二维整数数组 `edges` ，其中 <code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> 表示节点 <code>a<sub>i</sub></code> 和 <code>b<sub>i</sub></code> 之间有一条双向边。

**星图** 是给定图中的一个子图，它包含一个中心节点和 `0` 个或更多个邻居。换言之，星图是给定图中一个边的子集，且这些边都有一个公共节点。

下图分别展示了有 `3` 个和 `4` 个邻居的星图，蓝色节点为中心节点。

![](https://assets.leetcode.com/uploads/2022/11/07/max-star-sum-descdrawio.png)

**星和** 定义为星图中所有节点值的和。

给你一个整数 `k` ，请你返回 **至多** 包含 `k` 条边的星图中的 **最大星和** 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/11/07/max-star-sum-example1drawio.png)
<pre>
<strong>输入:</strong> vals = [1,2,3,4,10,-10,-20], edges = [[0,1],[1,2],[1,3],[3,4],[3,5],[3,6]], k = 2
<strong>输出:</strong> 16
<strong>解释:</strong> 上图展示了输入示例。
最大星和对应的星图在上图中用蓝色标出。中心节点是 3 ，星图中还包含邻居 1 和 4 。
无法得到一个和大于 16 且边数不超过 2 的星图。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> vals = [-5], edges = [], k = 0
<strong>输出:</strong> -5
<strong>解释:</strong> 只有一个星图，就是节点 0 自己。
所以我们返回 -5 。
</pre>

#### 提示:
* `n == vals.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>-10<sup>4</sup> <= vals[i] <= 10<sup>4</sup></code>
* <code>0 <= edges.length <= min(n * (n - 1) / 2, 10<sup>5</sup>)</code>
* `edges[i].length == 2`
* <code>0 <= a<sub>i</sub>, b<sub>i</sub> <= n - 1</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>
* `0 <= k <= n - 1`

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn max_star_sum(vals: Vec<i32>, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let mut heaps = vec![BinaryHeap::new(); vals.len()];
        let mut star_sum = vals.clone();

        for edge in &edges {
            let (a, b) = (edge[0] as usize, edge[1] as usize);

            if vals[b] > 0 {
                heaps[a].push(-vals[b]);
                star_sum[a] += vals[b];
            }
            if vals[a] > 0 {
                heaps[b].push(-vals[a]);
                star_sum[b] += vals[a];
            }
            if heaps[a].len() > k {
                star_sum[a] -= -heaps[a].pop().unwrap();
            }
            if heaps[b].len() > k {
                star_sum[b] -= -heaps[b].pop().unwrap();
            }
        }

        *star_sum.iter().max().unwrap()
    }
}
```
