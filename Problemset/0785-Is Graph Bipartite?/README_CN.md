# 785. 判断二分图
存在一个 **无向图** ，图中有 `n` 个节点。其中每个节点都有一个介于 `0` 到 `n - 1` 之间的唯一编号。给你一个二维数组 `graph` ，其中 `graph[u]` 是一个节点数组，由节点 `u` 的邻接节点组成。形式上，对于 `graph[u]` 中的每个 `v` ，都存在一条位于节点 `u` 和节点 `v` 之间的无向边。该无向图同时具有以下属性：
* 不存在自环（`graph[u]` 不包含 `u`）。
* 不存在平行边（`graph[u]` 不包含重复值）。
* 如果 `v` 在 `graph[u]` 内，那么 `u` 也应该在 `graph[v]` 内（该图是无向图）
* 这个图可能不是连通图，也就是说两个节点 `u` 和 `v` 之间可能不存在一条连通彼此的路径。

**二分图** 定义：如果能将一个图的节点集合分割成两个独立的子集 `A` 和 `B` ，并使图中的每一条边的两个节点一个来自 `A` 集合，一个来自 `B` 集合，就将这个图称为 **二分图** 。

如果图是二分图，返回 `true` ；否则，返回 `false` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/10/21/bi2.jpg)
<pre>
<strong>输入:</strong> graph = [[1,2,3],[0,2],[0,1,3],[0,2]]
<strong>输出:</strong> false
<strong>解释:</strong> 不能将节点分割成两个独立的子集，以使每条边都连通一个子集中的一个节点与另一个子集中的一个节点。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2020/10/21/bi1.jpg)
<pre>
<strong>输入:</strong> graph = [[1,3],[0,2],[1,3],[0,2]]
<strong>输出:</strong> true
<strong>解释:</strong> 可以将节点分成两组: {0, 2} 和 {1, 3} 。
</pre>

#### 提示:
* `graph.length == n`
* `1 <= n <= 100`
* `0 <= graph[u].length < n`
* `0 <= graph[u][i] <= n - 1`
* `graph[u]` 不会包含 `u`
* `graph[u]` 的所有值 **互不相同**
* 如果 `graph[u]` 包含 `v`，那么 `graph[v]` 也会包含 `u`

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let mut seen = HashSet::new();
        let mut a = HashSet::new();
        let mut b = HashSet::new();

        for i in 0..graph.len() {
            if !seen.contains(&i) {
                let mut insert_a = true;
                let mut nodes0 = vec![i];

                while !nodes0.is_empty() {
                    let mut nodes1 = vec![];

                    for j in nodes0 {
                        seen.insert(j);
                        if insert_a {
                            a.insert(j);
                        } else {
                            b.insert(j);
                        }

                        for &k in &graph[j] {
                            if !seen.contains(&(k as usize)) {
                                nodes1.push(k as usize);
                            }
                        }
                    }

                    insert_a = !insert_a;
                    nodes0 = nodes1;
                }
            }
        }

        a.intersection(&b).count() == 0
    }
}
```
