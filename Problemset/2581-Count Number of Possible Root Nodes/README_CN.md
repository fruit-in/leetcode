# 2581. 统计可能的树根数目
Alice 有一棵 `n` 个节点的树，节点编号为 `0` 到 `n - 1` 。树用一个长度为 `n - 1` 的二维整数数组 `edges` 表示，其中 <code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> ，表示树中节点 <code>a<sub>i</sub></code> 和 <code>b<sub>i</sub></code> 之间有一条边。

Alice 想要 Bob 找到这棵树的根。她允许 Bob 对这棵树进行若干次 **猜测** 。每一次猜测，Bob 做如下事情：
* 选择两个 **不相等** 的整数 `u` 和 `v` ，且树中必须存在边 `[u, v]` 。
* Bob 猜测树中 `u` 是 `v` 的 **父节点** 。

Bob 的猜测用二维整数数组 `guesses` 表示，其中 <code>guesses[j] = [u<sub>j</sub>, v<sub>j</sub>]</code> 表示 Bob 猜 <code>u<sub>j</sub></code> 是 <code>v<sub>j</sub></code> 的父节点。

Alice 非常懒，她不想逐个回答 Bob 的猜测，只告诉 Bob 这些猜测里面 **至少** 有 `k` 个猜测的结果为 `true` 。

给你二维整数数组 `edges` ，Bob 的所有猜测和整数 `k` ，请你返回可能成为树根的 **节点数目** 。如果没有这样的树，则返回 `0`。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/12/19/ex-1.png)
<pre>
<strong>输入:</strong> edges = [[0,1],[1,2],[1,3],[4,2]], guesses = [[1,3],[0,1],[1,0],[2,4]], k = 3
<strong>输出:</strong> 3
<strong>解释:</strong>
根为节点 0 ，正确的猜测为 [1,3], [0,1], [2,4]
根为节点 1 ，正确的猜测为 [1,3], [1,0], [2,4]
根为节点 2 ，正确的猜测为 [1,3], [1,0], [2,4]
根为节点 3 ，正确的猜测为 [1,0], [2,4]
根为节点 4 ，正确的猜测为 [1,3], [1,0]
节点 0 ，1 或 2 为根时，可以得到 3 个正确的猜测。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/12/19/ex-2.png)
<pre>
<strong>输入:</strong> edges = [[0,1],[1,2],[2,3],[3,4]], guesses = [[1,0],[3,4],[2,1],[3,2]], k = 1
<strong>输出:</strong> 5
<strong>解释:</strong>
根为节点 0 ，正确的猜测为 [3,4]
根为节点 1 ，正确的猜测为 [1,0], [3,4]
根为节点 2 ，正确的猜测为 [1,0], [2,1], [3,4]
根为节点 3 ，正确的猜测为 [1,0], [2,1], [3,2], [3,4]
根为节点 4 ，正确的猜测为 [1,0], [2,1], [3,2]
任何节点为根，都至少有 1 个正确的猜测。
</pre>

#### 提示:
* `edges.length == n - 1`
* <code>2 <= n <= 10<sup>5</sup></code>
* <code>1 <= guesses.length <= 10<sup>5</sup></code>
* <code>0 <= a<sub>i</sub>, b<sub>i</sub>, u<sub>j</sub>, v<sub>j</sub> <= n - 1</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>
* <code>u<sub>j</sub> != v<sub>j</sub></code>
* `edges` 表示一棵有效的树。
* `guesses[j]` 是树中的一条边。
* `guesses` 是唯一的。
* `0 <= k <= guesses.length`

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn root_count(edges: Vec<Vec<i32>>, guesses: Vec<Vec<i32>>, k: i32) -> i32 {
        let guesses = guesses
            .iter()
            .map(|g| (g[0] as usize, g[1] as usize))
            .collect::<HashSet<_>>();
        let n = edges.len() + 1;
        let mut neighbors = vec![vec![]; n];
        let mut stack = vec![(n, 0)];
        let mut as_root = vec![0; n];

        for edge in &edges {
            neighbors[edge[0] as usize].push(edge[1] as usize);
            neighbors[edge[1] as usize].push(edge[0] as usize);
        }

        while let Some((p, a)) = stack.pop() {
            if guesses.contains(&(p, a)) {
                as_root[0] += 1;
            }

            for &b in &neighbors[a] {
                if b != p {
                    stack.push((a, b));
                }
            }
        }

        stack = vec![(n, 0)];

        while let Some((p, a)) = stack.pop() {
            for &b in &neighbors[a] {
                if b != p {
                    stack.push((a, b));
                    as_root[b] = as_root[a];
                    if guesses.contains(&(a, b)) {
                        as_root[b] -= 1;
                    }
                    if guesses.contains(&(b, a)) {
                        as_root[b] += 1;
                    }
                }
            }
        }

        as_root.iter().filter(|&&t| t >= k).count() as i32
    }
}
```
