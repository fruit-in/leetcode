# 1579. 保证图可完全遍历
Alice 和 Bob 共有一个无向图，其中包含 n 个节点和 3  种类型的边：
* 类型 1：只能由 Alice 遍历。
* 类型 2：只能由 Bob 遍历。
* 类型 3：Alice 和 Bob 都可以遍历。

给你一个数组 `edges` ，其中 <code>edges[i] = [type<sub>i</sub>, u<sub>i</sub>, v<sub>i</sub>]</code> 表示节点 <code>u<sub>i</sub></code> 和 <code>v<sub>i</sub></code> 之间存在类型为 <code>type<sub>i</sub></code> 的双向边。请你在保证图仍能够被 Alice和 Bob 完全遍历的前提下，找出可以删除的最大边数。如果从任何节点开始，Alice 和 Bob 都可以到达所有其他节点，则认为图是可以完全遍历的。

返回可以删除的最大边数，如果 Alice 和 Bob 无法完全遍历图，则返回 -1 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/08/19/ex1.png)
<pre>
<strong>输入:</strong> n = 4, edges = [[3,1,2],[3,2,3],[1,1,3],[1,2,4],[1,1,2],[2,3,4]]
<strong>输出:</strong> 2
<strong>解释:</strong> 如果删除 [1,1,2] 和 [1,1,3] 这两条边，Alice 和 Bob 仍然可以完全遍历这个图。再删除任何其他的边都无法保证图可以完全遍历。所以可以删除的最大边数是 2 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2020/08/19/ex2.png)
<pre>
<strong>输入:</strong> n = 4, edges = [[3,1,2],[3,2,3],[1,1,4],[2,1,4]]
<strong>输出:</strong> 0
<strong>解释:</strong> 注意，删除任何一条边都会使 Alice 和 Bob 无法完全遍历这个图。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2020/08/19/ex3.png)
<pre>
<strong>输入:</strong> n = 4, edges = [[3,2,3],[1,1,2],[2,3,4]]
<strong>输出:</strong> -1
<strong>解释:</strong> 在当前图中，Alice 无法从其他节点到达节点 4 。类似地，Bob 也不能达到节点 1 。因此，图无法完全遍历。
</pre>

#### 提示:
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>1 <= edges.length <= min(10<sup>5</sup>, 3 * n * (n - 1) / 2)</code>
* `edges[i].length == 3`
* <code>1 <= type<sub>i</sub> <= 3</code>
* <code>1 <= u<sub>i</sub> < v<sub>i</sub> <= n</code>
* 所有元组 <code>(type<sub>i</sub>, u<sub>i</sub>, v<sub>i</sub>)</code> 互不相同

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def maxNumEdgesToRemove(self, n: int, edges: List[List[int]]) -> int:
        parenta = list(range(n + 1))
        parentb = list(range(n + 1))
        counta = 0
        countb = 0
        ret = len(edges)

        edges.sort(key=lambda edge: -edge[0])

        for t, u, v in edges:
            while t != 2 and parenta[u] != parenta[parenta[u]]:
                parenta[u] = parenta[parenta[u]]
            while t != 2 and parenta[v] != parenta[parenta[v]]:
                parenta[v] = parenta[parenta[v]]
            while t != 1 and parentb[u] != parentb[parentb[u]]:
                parentb[u] = parentb[parentb[u]]
            while t != 1 and parentb[v] != parentb[parentb[v]]:
                parentb[v] = parentb[parentb[v]]

            if t == 3 and parenta[u] != parenta[v]:
                parenta[parenta[u]] = parenta[v]
                parentb[parentb[u]] = parentb[v]
                counta += 1
                countb += 1
                ret -= 1
            elif t == 2 and parentb[u] != parentb[v]:
                parentb[parentb[u]] = parentb[v]
                countb += 1
                ret -= 1
            elif t == 1 and parenta[u] != parenta[v]:
                parenta[parenta[u]] = parenta[v]
                counta += 1
                ret -= 1

        return ret if counta == countb == n - 1 else -1
```
