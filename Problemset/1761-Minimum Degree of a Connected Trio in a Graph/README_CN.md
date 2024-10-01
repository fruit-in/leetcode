# 1761. 一个图中连通三元组的最小度数
给你一个无向图，整数 `n` 表示图中节点的数目，`edges` 数组表示图中的边，其中 <code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>]</code> ，表示 <code>u<sub>i</sub></code> 和 <code>v<sub>i</sub></code> 之间有一条无向边。

一个 **连通三元组** 指的是 **三个** 节点组成的集合且这三个点之间 **两两** 有边。

**连通三元组的度数** 是所有满足此条件的边的数目：一个顶点在这个三元组内，而另一个顶点不在这个三元组内。

请你返回所有连通三元组中度数的 **最小值** ，如果图中没有连通三元组，那么返回 `-1` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/01/26/trios1.png)
<pre>
<strong>输入:</strong> n = 6, edges = [[1,2],[1,3],[3,2],[4,1],[5,2],[3,6]]
<strong>输出:</strong> 3
<strong>解释:</strong> 只有一个三元组 [1,2,3] 。构成度数的边在上图中已被加粗。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/01/26/trios2.png)
<pre>
<strong>输入:</strong> n = 7, edges = [[1,3],[4,1],[4,3],[2,5],[5,6],[6,7],[7,5],[2,6]]
<strong>输出:</strong> 0
<strong>解释:</strong> 有 3 个三元组：
1) [1,4,3]，度数为 0 。
2) [2,5,6]，度数为 2 。
3) [5,6,7]，度数为 2 。
</pre>

#### 提示:
* `2 <= n <= 400`
* `edges[i].length == 2`
* `1 <= edges.length <= n * (n-1) / 2`
* <code>1 <= u<sub>i</sub>, v<sub>i</sub> <= n</code>
* <code>u<sub>i</sub> != v<sub>i</sub></code>
* 图中没有重复的边。

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn min_trio_degree(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut edges_set = HashSet::new();
        let mut count = vec![0; n + 1];
        let mut ret = -1;

        for i in 0..edges.len() {
            let (u, v) = (
                edges[i][0].min(edges[i][1]) as usize,
                edges[i][0].max(edges[i][1]) as usize,
            );

            edges_set.insert((u, v));
            count[u] += 1;
            count[v] += 1;
        }

        for i in 1..=n {
            for j in i + 1..=n {
                if !edges_set.contains(&(i, j)) {
                    continue;
                }

                for k in j + 1..=n {
                    if !edges_set.contains(&(i, k)) || !edges_set.contains(&(j, k)) {
                        continue;
                    }

                    let degree = count[i] + count[j] + count[k] - 6;

                    if ret == -1 || degree < ret {
                        ret = degree;
                    }
                }
            }
        }

        ret
    }
}
```
