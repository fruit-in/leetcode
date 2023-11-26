# 2360. 图中的最长环
给你一个 `n` 个节点的 **有向图** ，节点编号为 `0` 到 `n - 1` ，其中每个节点 **至多** 有一条出边。

图用一个大小为 `n` 下标从 **0** 开始的数组 `edges` 表示，节点 `i` 到节点 `edges[i]` 之间有一条有向边。如果节点 `i` 没有出边，那么 `edges[i] == -1` 。

请你返回图中的 **最长** 环，如果没有任何环，请返回 `-1` 。

一个环指的是起点和终点是 **同一个** 节点的路径。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/06/08/graph4drawio-5.png)
<pre>
<strong>输入:</strong> edges = [3,3,4,2,3]
<strong>输出:</strong> 3
<strong>解释:</strong> 图中的最长环是：2 -> 4 -> 3 -> 2 。
这个环的长度为 3 ，所以返回 3 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/06/07/graph4drawio-1.png)
<pre>
<strong>输入:</strong> edges = [2,-1,3,1]
<strong>输出:</strong> -1
<strong>解释:</strong> 图中没有任何环。
</pre>

#### 提示:
* `n == edges.length`
* <code>2 <= n <= 10<sup>5</sup></code>
* `-1 <= edges[i] < n`
* `edges[i] != i`

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn longest_cycle(edges: Vec<i32>) -> i32 {
        let mut visited = vec![false; edges.len()];
        let mut nodes = HashMap::new();
        let mut ret = -1;

        for i in 0..edges.len() {
            if visited[i] {
                continue;
            }

            let mut i = i;
            let mut count = 1;
            nodes.clear();

            while edges[i] != -1 {
                if let Some(&x) = nodes.get(&i) {
                    ret = ret.max(count - x);
                    break;
                } else if visited[i] {
                    break;
                }

                visited[i] = true;
                nodes.insert(i, count);
                i = edges[i] as usize;
                count += 1;
            }
        }

        ret
    }
}
```
