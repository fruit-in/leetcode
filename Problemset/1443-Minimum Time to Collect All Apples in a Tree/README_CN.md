# 1443. 收集树上所有苹果的最少时间
给你一棵有 `n` 个节点的无向树，节点编号为 `0` 到 `n-1` ，它们中有一些节点有苹果。通过树上的一条边，需要花费 1 秒钟。你从 **节点 0** 出发，请你返回最少需要多少秒，可以收集到所有苹果，并回到节点 0 。

无向树的边由 `edges` 给出，其中 <code>edges[i] = [from<sub>i</sub>, to<sub>i</sub>]</code> ，表示有一条边连接 `from` 和 <code>to<sub>i</sub></code> 。除此以外，还有一个布尔数组 `hasApple` ，其中 `hasApple[i] = true` 代表节点 `i` 有一个苹果，否则，节点 `i` 没有苹果。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/04/23/min_time_collect_apple_1.png)
<pre>
<strong>输入:</strong> n = 7, edges = [[0,1],[0,2],[1,4],[1,5],[2,3],[2,6]], hasApple = [false,false,true,false,true,true,false]
<strong>输出:</strong> 8
<strong>解释:</strong> 上图展示了给定的树，其中红色节点表示有苹果。一个能收集到所有苹果的最优方案由绿色箭头表示。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2020/04/23/min_time_collect_apple_2.png)
<pre>
<strong>输入:</strong> n = 7, edges = [[0,1],[0,2],[1,4],[1,5],[2,3],[2,6]], hasApple = [false,false,true,false,false,true,false]
<strong>输出:</strong> 6
<strong>解释:</strong> 上图展示了给定的树，其中红色节点表示有苹果。一个能收集到所有苹果的最优方案由绿色箭头表示。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 7, edges = [[0,1],[0,2],[1,4],[1,5],[2,3],[2,6]], hasApple = [false,false,false,false,false,false,false]
<strong>输出:</strong> 0
</pre>

#### 提示:
* <code>1 <= n <= 10<sup>5</sup></code>
* `edges.length == n - 1`
* `edges[i].length == 2`
* <code>0 <= a<sub>i</sub> < b<sub>i</sub> <= n - 1</code>
* `hasApple.length == n`

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn min_time(n: i32, edges: Vec<Vec<i32>>, mut has_apple: Vec<bool>) -> i32 {
        let n = n as usize;
        let mut children = vec![HashSet::new(); n];
        let mut parent = vec![n; n];
        let mut nodes = vec![0];

        for edge in &edges {
            children[edge[0] as usize].insert(edge[1] as usize);
            children[edge[1] as usize].insert(edge[0] as usize);
        }

        while let Some(node) = nodes.pop() {
            children[node].remove(&parent[node]);

            for &child in children[node].iter() {
                parent[child] = node;
                nodes.push(child);
            }
        }

        for node in 0..n {
            if children[node].is_empty() {
                nodes.push(node);
            }
        }

        while let Some(node) = nodes.pop() {
            if node == 0 {
                break;
            }

            has_apple[parent[node]] |= has_apple[node];
            children[parent[node]].remove(&node);
            if children[parent[node]].is_empty() {
                nodes.push(parent[node]);
            }
        }

        ((0..n).filter(|&node| has_apple[node]).count() as i32 - 1).max(0) * 2
    }
}
```
