# 1334. 阈值距离内邻居最少的城市
有 `n` 个城市，按从 `0` 到 `n-1` 编号。给你一个边数组 `edges`，其中 <code>edges[i] = [from<sub>i</sub>, to<sub>i</sub>, weight<sub>i</sub>]</code> 代表 <code>from<sub>i</sub></code> 和 <code>to<sub>i</sub></code> 两个城市之间的双向加权边，距离阈值是一个整数 `distanceThreshold`。

返回在路径距离限制为 `distanceThreshold` 以内可到达城市最少的城市。如果有多个这样的城市，则返回编号最大的城市。

注意，连接城市 ***i*** 和 ***j*** 的路径的距离等于沿该路径的所有边的权重之和。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2024/08/23/problem1334example1.png)
<pre>
<strong>输入:</strong> n = 4, edges = [[0,1,3],[1,2,1],[1,3,4],[2,3,1]], distanceThreshold = 4
<strong>输出:</strong> 3
<strong>解释:</strong> 城市分布图如上。
每个城市阈值距离 distanceThreshold = 4 内的邻居城市分别是：
城市 0 -> [城市 1, 城市 2]
城市 1 -> [城市 0, 城市 2, 城市 3]
城市 2 -> [城市 0, 城市 1, 城市 3]
城市 3 -> [城市 1, 城市 2]
城市 0 和 3 在阈值距离 4 以内都有 2 个邻居城市，但是我们必须返回城市 3，因为它的编号最大。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2024/08/23/problem1334example0.png)
<pre>
<strong>输入:</strong> n = 5, edges = [[0,1,2],[0,4,8],[1,2,3],[1,4,2],[2,3,1],[3,4,1]], distanceThreshold = 2
<strong>输出:</strong> 0
<strong>解释:</strong> 城市分布图如上。
每个城市阈值距离 distanceThreshold = 2 内的邻居城市分别是：
城市 0 -> [城市 1]
城市 1 -> [城市 0, 城市 4]
城市 2 -> [城市 3, 城市 4]
城市 3 -> [城市 2, 城市 4]
城市 4 -> [城市 1, 城市 2, 城市 3]
城市 0 在阈值距离 2 以内只有 1 个邻居城市。
</pre>

#### 提示:
* `2 <= n <= 100`
* `1 <= edges.length <= n * (n - 1) / 2`
* `edges[i].length == 3`
* <code>0 <= from<sub>i</sub> < to<sub>i</sub> < n</code>
* <code>1 <= weight<sub>i</sub>, distanceThreshold <= 10^4</code>
* 所有 <code>(from<sub>i</sub>, to<sub>i</sub>)</code> 都是不同的。

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::BinaryHeap;
use std::collections::HashSet;

impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let n = n as usize;
        let mut to_cities = vec![vec![]; n];
        let mut min_reachable = usize::MAX;
        let mut ret = 0;

        for edge in &edges {
            if edge[2] <= distance_threshold {
                to_cities[edge[0] as usize].push((edge[1] as usize, edge[2]));
                to_cities[edge[1] as usize].push((edge[0] as usize, edge[2]));
            }
        }

        for i in 0..n {
            let mut heap = BinaryHeap::from([(0, i)]);
            let mut visited = HashSet::new();

            while let Some((weight, from)) = heap.pop() {
                visited.insert(from);

                for &(to, w) in &to_cities[from] {
                    if !visited.contains(&to) && -weight + w <= distance_threshold {
                        heap.push((weight - w, to));
                    }
                }
            }

            if visited.len() <= min_reachable {
                min_reachable = visited.len();
                ret = i;
            }
        }

        ret as i32
    }
}
```
