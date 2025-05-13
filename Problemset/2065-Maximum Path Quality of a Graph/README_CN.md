# 2065. 最大化一张图中的路径价值
给你一张 **无向** 图，图中有 `n` 个节点，节点编号从 `0` 到 `n - 1` （**都包括**）。同时给你一个下标从 **0** 开始的整数数组 `values` ，其中 `values[i]` 是第 `i` 个节点的 **价值** 。同时给你一个下标从 **0** 开始的二维整数数组 `edges` ，其中 <code>edges[j] = [u<sub>j</sub>, v<sub>j</sub>, time<sub>j</sub>]</code> 表示节点 <code>u<sub>j</sub></code> 和 <code>v<sub>j</sub></code> 之间有一条需要 <code>time<sub>j</sub></code> 秒才能通过的无向边。最后，给你一个整数 `maxTime` 。

**合法路径** 指的是图中任意一条从节点 `0` 开始，最终回到节点 `0` ，且花费的总时间 **不超过** `maxTime` 秒的一条路径。你可以访问一个节点任意次。一条合法路径的 **价值** 定义为路径中 **不同节点** 的价值 **之和** （每个节点的价值 **至多** 算入价值总和中一次）。

请你返回一条合法路径的 **最大** 价值。

**注意：**每个节点 **至多** 有 **四条** 边与之相连。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/10/19/ex1drawio.png)
<pre>
<strong>输入:</strong> values = [0,32,10,43], edges = [[0,1,10],[1,2,15],[0,3,10]], maxTime = 49
<strong>输出:</strong> 75
<strong>解释:</strong>
一条可能的路径为：0 -> 1 -> 0 -> 3 -> 0 。总花费时间为 10 + 10 + 10 + 10 = 40 <= 49 。
访问过的节点为 0 ，1 和 3 ，最大路径价值为 0 + 32 + 43 = 75 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/10/19/ex2drawio.png)
<pre>
<strong>输入:</strong> values = [5,10,15,20], edges = [[0,1,10],[1,2,10],[0,3,10]], maxTime = 30
<strong>输出:</strong> 25
<strong>解释:</strong>
一条可能的路径为：0 -> 3 -> 0 。总花费时间为 10 + 10 = 20 <= 30 。
访问过的节点为 0 和 3 ，最大路径价值为 5 + 20 = 25 。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2021/10/19/ex31drawio.png)
<pre>
<strong>输入:</strong> values = [1,2,3,4], edges = [[0,1,10],[1,2,11],[2,3,12],[1,3,13]], maxTime = 50
<strong>输出:</strong> 7
<strong>解释:</strong>
一条可能的路径为：0 -> 1 -> 3 -> 1 -> 0 。总花费时间为 10 + 13 + 13 + 10 = 46 <= 50 。
访问过的节点为 0 ，1 和 3 ，最大路径价值为 1 + 2 + 4 = 7 。
</pre>

#### 示例 4:
![](https://assets.leetcode.com/uploads/2021/10/21/ex4drawio.png)
<pre>
<strong>输入:</strong> values = [0,1,2], edges = [[1,2,10]], maxTime = 10
<strong>输出:</strong> 0
<strong>解释:</strong>
唯一一条路径为 0 。总花费时间为 0 。
唯一访问过的节点为 0 ，最大路径价值为 0 。
</pre>

#### 提示:
* `n == values.length`
* `1 <= n <= 1000`
* <code>0 <= values[i] <= 10<sup>8</sup></code>
* `0 <= edges.length <= 2000`
* `edges[j].length == 3 `
* <code>0 <= u<sub>j</sub> < v<sub>j</sub> <= n - 1</code>
* <code>10 <= time<sub>j</sub>, maxTime <= 100</code>
* <code>[u<sub>j</sub>, v<sub>j</sub>]</code> 所有节点对 **互不相同** 。
* 每个节点 **至多有四条** 边。
* 图可能不连通。

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def maximalPathQuality(self, values: List[int], edges: List[List[int]], maxTime: int) -> int:
        def dfs(u: int, time: int) -> None:
            nonlocal quality, ret
            if u == 0:
                ret = max(ret, quality)
            if usetimes[u] == 0:
                quality += values[u]
            usetimes[u] += 1

            for v, t in neighbors[u].items():
                if time + t <= maxTime:
                    dfs(v, time + t)

            usetimes[u] -= 1
            if usetimes[u] == 0:
                quality -= values[u]

        n = len(values)
        neighbors = [{} for _ in range(n)]
        usetimes = [0] * n
        quality = 0
        ret = values[0]

        for u, v, time in edges:
            if time <= maxTime:
                neighbors[u][v] = time
                neighbors[v][u] = time

        dfs(0, 0)

        return ret
```
