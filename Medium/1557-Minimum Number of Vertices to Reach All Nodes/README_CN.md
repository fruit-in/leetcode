# 1557. 可以到达所有点的最少点数目
给你一个 **有向无环图** ， `n` 个节点编号为 `0` 到 `n-1` ，以及一个边数组 `edges` ，其中 <code>edges[i] = [from<sub>i</sub>, to<sub>i</sub>]</code> 表示一条从点  <code>from<sub>i</sub></code> 到点 <code>to<sub>i</sub></code> 的有向边。

找到最小的点集使得从这些点出发能到达图中所有点。题目保证解存在且唯一。

你可以以任意顺序返回这些节点编号。

#### 示例 1:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/08/22/5480e1.png)
<pre>
<b>输入:</b> n = 6, edges = [[0,1],[0,2],[2,5],[3,4],[4,2]]
<b>输出:</b> [0,3]
<b>解释:</b> 从单个节点出发无法到达所有节点。从 0 出发我们可以到达 [0,1,2,5] 。从 3 出发我们可以到达 [3,4,2,5] 。所以我们输出 [0,3] 。
</pre>

#### 示例 2:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/08/22/5480e2.png)
<pre>
<b>输入:</b> n = 5, edges = [[0,1],[2,1],[3,1],[1,4],[2,4]]
<b>输出:</b> [0,2,3]
<b>解释:</b> 注意到节点 0，3 和 2 无法从其他节点到达，所以我们必须将它们包含在结果点集中，这些点都能到达节点 1 和 4 。
</pre>

#### 提示:
* `2 <= n <= 10^5`
* `1 <= edges.length <= min(10^5, n * (n - 1) / 2)`
* `edges[i].length == 2`
* <code>0 <= from<sub>i</sub>, to<sub>i</sub> < n</code>
* 所有点对 <code>(from<sub>i</sub>, to<sub>i</sub>)</code> 互不相同。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut start = vec![true; n as usize];

        for edge in edges {
            start[edge[1] as usize] = false;
        }

        (0..n).filter(|&x| start[x as usize]).collect()
    }
}
```
