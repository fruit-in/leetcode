# 2608. 图中的最短环
现有一个含 `n` 个顶点的 **双向** 图，每个顶点按从 `0` 到 `n - 1` 标记。图中的边由二维整数数组 `edges` 表示，其中 <code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>]</code> 表示顶点 <code>u<sub>i</sub></code> 和 <code>v<sub>i</sub></code> 之间存在一条边。每对顶点最多通过一条边连接，并且不存在与自身相连的顶点。

返回图中 **最短** 环的长度。如果不存在环，则返回 `-1` 。

**环** 是指以同一节点开始和结束，并且路径中的每条边仅使用一次。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2023/01/04/cropped.png)
<pre>
<strong>输入:</strong> n = 7, edges = [[0,1],[1,2],[2,0],[3,4],[4,5],[5,6],[6,3]]
<strong>输出:</strong> 3
<strong>解释:</strong> 长度最小的循环是：0 -> 1 -> 2 -> 0
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2023/01/04/croppedagin.png)
<pre>
<strong>输入:</strong> n = 4, edges = [[0,1],[0,2]]
<strong>输出:</strong> -1
<strong>解释:</strong> 图中不存在循环
</pre>

#### 提示:
* `2 <= n <= 1000`
* `1 <= edges.length <= 1000`
* `edges[i].length == 2`
* <code>0 <= u<sub>i</sub>, v<sub>i</sub> < n</code>
* <code>u<sub>i</sub> != v<sub>i</sub></code>
* 不存在重复的边

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;
use std::collections::VecDeque;

impl Solution {
    pub fn find_shortest_cycle(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut neighbors = vec![vec![]; n];
        let mut ret = n + 1;

        for edge in &edges {
            neighbors[edge[0] as usize].push(edge[1] as usize);
            neighbors[edge[1] as usize].push(edge[0] as usize);
        }

        for i in 0..n {
            let mut deque = VecDeque::from([(n, i)]);
            let mut distance = HashMap::from([(i, 0)]);

            while let Some((from, to)) = deque.pop_front() {
                if distance[&to] >= ret {
                    break;
                }

                for &j in &neighbors[to] {
                    if j == from {
                        continue;
                    }

                    if let Some(d) = distance.get(&j) {
                        ret = ret.min(d + distance[&to] + 1);
                    } else {
                        deque.push_back((to, j));
                        distance.insert(j, distance[&to] + 1);
                    }
                }
            }
        }

        if ret > n {
            return -1;
        }

        ret as i32
    }
}
```
