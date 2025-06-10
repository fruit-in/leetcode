# 2538. 最大价值和与最小价值和的差值
给你一个 `n` 个节点的无向无根图，节点编号为 `0` 到 `n - 1` 。给你一个整数 `n` 和一个长度为 `n - 1` 的二维整数数组 `edges` ，其中 <code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> 表示树中节点 <code>a<sub>i</sub></code> 和 <code>b<sub>i</sub></code> 之间有一条边。

每个节点都有一个价值。给你一个整数数组 `price` ，其中 `price[i]` 是第 `i` 个节点的价值。

一条路径的 **价值和** 是这条路径上所有节点的价值之和。

你可以选择树中任意一个节点作为根节点 `root` 。选择 `root` 为根的 **开销** 是以 `root` 为起点的所有路径中，**价值和** 最大的一条路径与最小的一条路径的差值。

请你返回所有节点作为根节点的选择中，**最大** 的 **开销** 为多少。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/12/01/example14.png)
<pre>
<strong>输入:</strong> n = 6, edges = [[0,1],[1,2],[1,3],[3,4],[3,5]], price = [9,8,7,6,10,5]
<strong>输出:</strong> 24
<strong>解释:</strong> 上图展示了以节点 2 为根的树。左图（红色的节点）是最大价值和路径，右图（蓝色的节点）是最小价值和路径。
- 第一条路径节点为 [2,1,3,4]：价值为 [7,8,6,10] ，价值和为 31 。
- 第二条路径节点为 [2] ，价值为 [7] 。
最大路径和与最小路径和的差值为 24 。24 是所有方案中的最大开销。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/11/24/p1_example2.png)
<pre>
<strong>输入:</strong> n = 3, edges = [[0,1],[1,2]], price = [1,1,1]
<strong>输出:</strong> 2
<strong>解释:</strong> 上图展示了以节点 0 为根的树。左图（红色的节点）是最大价值和路径，右图（蓝色的节点）是最小价值和路径。
- 第一条路径包含节点 [0,1,2]：价值为 [1,1,1] ，价值和为 3 。
- 第二条路径节点为 [0] ，价值为 [1] 。
最大路径和与最小路径和的差值为 2 。2 是所有方案中的最大开销。
</pre>

#### 提示:
* <code>1 <= n <= 10<sup>5</sup></code>
* `edges.length == n - 1`
* <code>0 <= a<sub>i</sub>, b<sub>i</sub> <= n - 1</code>
* `edges` 表示一棵符合题面要求的树。
* `price.length == n`
* <code>1 <= price[i] <= 10<sup>5</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def maxOutput(self, n: int, edges: List[List[int]], price: List[int]) -> int:
        def removeParentFromChildren(node: int, parent: int) -> None:
            i = children[node].index(parent)
            children[node][i], children[node][-1] = children[node][-1], children[node][i]
            children[node].pop()
            for child in children[node]:
                removeParentFromChildren(child, node)

        def setTop2(node: int) -> None:
            a, c0, b, c1 = -1, 0, -1, 0
            for child in children[node]:
                setTop2(child)
                c2 = top2[child][1] + price[child]
                if c2 >= c0:
                    a, c0, b, c1 = child, c2, a, c0
                elif c2 > c1:
                    b, c1 = child, c2
            top2[node] = [a, c0, b, c1]

        def maxCost(node: int, p: int) -> int:
            a, c0, _, c1 = top2[node]
            cost = max(c0, p)
            for child in children[node]:
                if child != a:
                    cost = max(cost, maxCost(child, max(p, c0) + price[node]))
                else:
                    cost = max(cost, maxCost(child, max(p, c1) + price[node]))
            return cost

        children = [[] for _ in range(n)]
        children[0].append(-1)
        top2 = [[] for _ in range(n)]

        for a, b in edges:
            children[a].append(b)
            children[b].append(a)

        removeParentFromChildren(0, -1)
        setTop2(0)

        return maxCost(0, 0)
```
