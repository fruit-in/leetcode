# 2316. 统计无向图中无法互相到达点对数
给你一个整数 `n` ，表示一张 **无向图** 中有 `n` 个节点，编号为 `0` 到 `n - 1` 。同时给你一个二维整数数组 `edges` ，其中 <code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> 表示节点 <code>a<sub>i</sub></code> 和 <code>b<sub>i</sub></code> 之间有一条 **无向** 边。

请你返回 **无法互相到达** 的不同 **点对数目** 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/05/05/tc-3.png)
<pre>
<strong>输入:</strong> n = 3, edges = [[0,1],[0,2],[1,2]]
<strong>输出:</strong> 0
<strong>解释:</strong> 所有点都能互相到达，意味着没有点对无法互相到达，所以我们返回 0 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/05/05/tc-2.png)
<pre>
<strong>输入:</strong> n = 7, edges = [[0,2],[0,5],[2,4],[1,6],[5,4]]
<strong>输出:</strong> 14
<strong>解释:</strong> 总共有 14 个点对互相无法到达：
[[0,1],[0,3],[0,6],[1,2],[1,3],[1,4],[1,5],[2,3],[2,6],[3,4],[3,5],[3,6],[4,6],[5,6]]
所以我们返回 14 。
</pre>

#### 提示:
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>0 <= edges.length <= 2 * 10<sup>5</sup></code>
* `edges[i].length == 2`
* <code>0 <= a<sub>i</sub>, b<sub>i</sub> < n</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>
* 不会有重复边。

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def countPairs(self, n: int, edges: List[List[int]]) -> int:
        parent = list(range(n))
        count = [0] * n

        for a, b in edges:
            while parent[a] != parent[parent[a]]:
                parent[a] = parent[parent[a]]
            while parent[b] != parent[parent[b]]:
                parent[b] = parent[parent[b]]
            if parent[a] < parent[b]:
                parent[parent[b]] = parent[a]
            else:
                parent[parent[a]] = parent[b]

        for i in range(n):
            while parent[i] != parent[parent[i]]:
                parent[i] = parent[parent[i]]
            count[parent[i]] += 1

        return sum(c * (n - c) for c in count) // 2
```
