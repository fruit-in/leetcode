# 1617. 统计子树中城市之间最大距离
给你 `n` 个城市，编号为从 `1` 到 `n` 。同时给你一个大小为 `n-1` 的数组 `edges` ，其中 <code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>]</code> 表示城市 <code>u<sub>i</sub></code> 和 <code>v<sub>i</sub></code> 之间有一条双向边。题目保证任意城市之间只有唯一的一条路径。换句话说，所有城市形成了一棵 **树** 。

一棵 **子树** 是城市的一个子集，且子集中任意城市之间可以通过子集中的其他城市和边到达。两个子树被认为不一样的条件是至少有一个城市在其中一棵子树中存在，但在另一棵子树中不存在。

对于 `d` 从 `1` 到 `n-1` ，请你找到城市间 **最大距离** 恰好为 `d` 的所有子树数目。

请你返回一个大小为 `n-1` 的数组，其中第 `d` 个元素（**下标从 1 开始**）是城市间 **最大距离** 恰好等于 `d` 的子树数目。

**请注意**，两个城市间距离定义为它们之间需要经过的边的数目。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/09/21/p1.png)
<pre>
<strong>输入:</strong> n = 4, edges = [[1,2],[2,3],[2,4]]
<strong>输出:</strong> [3,4,0]
<strong>解释:</strong>
子树 {1,2}, {2,3} 和 {2,4} 最大距离都是 1 。
子树 {1,2,3}, {1,2,4}, {2,3,4} 和 {1,2,3,4} 最大距离都为 2 。
不存在城市间最大距离为 3 的子树。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 2, edges = [[1,2]]
<strong>输出:</strong> [1]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 3, edges = [[1,2],[2,3]]
<strong>输出:</strong> [2,1]
</pre>

#### 提示:
* `2 <= n <= 15`
* `edges.length == n-1`
* `edges[i].length == 2`
* <code>1 <= u<sub>i</sub>, v<sub>i</sub> <= n</code>
* 题目保证 <code>(u<sub>i</sub>, v<sub>i</sub>)</code> 所表示的边互不相同。

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def countSubgraphsForEachDiameter(self, n: int, edges: List[List[int]]) -> List[int]:
        def buildTree(x: int) -> (int, List[List[int]]):
            countnodes = x.bit_count()
            countedges = 0
            root = -1
            children = [[] for _ in range(n)]

            for u, v in edges:
                if (x >> u) & 1 == 1 and (x >> v) & 1 == 1:
                    countedges += 1
                    root = u
                    children[u].append(v)
                    children[v].append(u)

            if countnodes == 1 or countnodes - countedges != 1:
                return (-1, [])

            stack = [root]
            while stack:
                node = stack.pop()
                for child in children[node]:
                    children[child].remove(node)
                    stack.append(child)

            return (root, children)

        def maxDistance(root: int, children: List[List[int]]) -> (int, int):
            maxdist, maxheight = 0, 0
            c0, h0, c1, h1 = 0, -1, 0, -1

            for child in children[root]:
                dist, height = maxDistance(child, children)
                maxdist = max(maxdist, dist)
                maxheight = max(maxheight, height + 1)

                if height >= h0:
                    c0, h0, c1, h1 = child, height, c0, h0
                elif height >= h1:
                    c1, h1 = child, height

            return (max(maxdist, h0 + h1 + 2), maxheight)

        edges = [[u - 1, v - 1] for u, v in edges]
        ans = [0] * (n - 1)

        for x in range(3, 1 << n):
            root, children = buildTree(x)
            if root >= 0:
                ans[maxDistance(root, children)[0] - 1] += 1

        return ans
```
