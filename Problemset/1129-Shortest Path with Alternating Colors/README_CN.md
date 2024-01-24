# 1129. 颜色交替的最短路径
给定一个整数 `n`，即有向图中的节点数，其中节点标记为 `0` 到 `n - 1`。图中的每条边为红色或者蓝色，并且可能存在自环或平行边。

给定两个数组 `redEdges` 和 `blueEdges`，其中：

* <code>redEdges[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> 表示图中存在一条从节点 <code>a<sub>i</sub></code> 到节点 <code>b<sub>i</sub></code> 的红色有向边，
* <code>blueEdges[j] = [u<sub>j</sub>, v<sub>j</sub>]</code> 表示图中存在一条从节点 <code>u<sub>j</sub></code> 到节点 <code>v<sub>j</sub></code> 的蓝色有向边。

返回长度为 `n` 的数组 `answer`，其中 `answer[X]` 是从节点 `0` 到节点 `X` 的红色边和蓝色边交替出现的最短路径的长度。如果不存在这样的路径，那么 `answer[x] = -1`。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 3, redEdges = [[0,1],[1,2]], blueEdges = []
<strong>输出:</strong> [0,1,-1]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 3, redEdges = [[0,1]], blueEdges = [[2,1]]
<strong>输出:</strong> [0,1,-1]
</pre>

#### 提示:
* `1 <= n <= 100`
* `0 <= redEdges.length, blueEdges.length <= 400`
* `redEdges[i].length == blueEdges[j].length == 2`
* <code>0 <= a<sub>i</sub>, b<sub>i</sub>, u<sub>j</sub>, v<sub>j</sub> < n</code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;

impl Solution {
    pub fn shortest_alternating_paths(
        n: i32,
        red_edges: Vec<Vec<i32>>,
        blue_edges: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let n = n as usize;
        let mut red_nexts = vec![vec![]; n];
        let mut blue_nexts = vec![vec![]; n];
        let mut nodes_heap = BinaryHeap::from([(Reverse(0), 0, true), (Reverse(0), 0, false)]);
        let mut visited = HashSet::new();
        let mut answer = vec![(i32::MAX, i32::MAX); n];
        answer[0] = (0, 0);

        for edge in &red_edges {
            red_nexts[edge[0] as usize].push(edge[1] as usize);
        }
        for edge in &blue_edges {
            blue_nexts[edge[0] as usize].push(edge[1] as usize);
        }

        while let Some((Reverse(length), node, is_red)) = nodes_heap.pop() {
            if visited.contains(&(node, is_red)) {
                continue;
            }

            visited.insert((node, is_red));

            if is_red {
                for &next in &blue_nexts[node] {
                    if answer[next].1 > length + 1 {
                        answer[next].1 = length + 1;
                        nodes_heap.push((Reverse(length + 1), next, false));
                    }
                }
            } else {
                for &next in &red_nexts[node] {
                    if answer[next].0 > length + 1 {
                        answer[next].0 = length + 1;
                        nodes_heap.push((Reverse(length + 1), next, true));
                    }
                }
            }
        }

        answer
            .into_iter()
            .map(|(red, blue)| {
                if red.min(blue) == i32::MAX {
                    -1
                } else {
                    red.min(blue)
                }
            })
            .collect()
    }
}
```
