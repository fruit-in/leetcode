# 2477. 到达首都的最少油耗
给你一棵 `n` 个节点的树（一个无向、连通、无环图），每个节点表示一个城市，编号从 `0` 到 `n - 1` ，且恰好有 `n - 1` 条路。`0` 是首都。给你一个二维整数数组 `roads` ，其中 <code>roads[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> ，表示城市 <code>a<sub>i</sub></code> 和 <code>b<sub>i</sub></code> 之间有一条 **双向路** 。

每个城市里有一个代表，他们都要去首都参加一个会议。

每座城市里有一辆车。给你一个整数 `seats` 表示每辆车里面座位的数目。

城市里的代表可以选择乘坐所在城市的车，或者乘坐其他城市的车。相邻城市之间一辆车的油耗是一升汽油。

请你返回到达首都最少需要多少升汽油。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/09/22/a4c380025e3ff0c379525e96a7d63a3.png)
<pre>
<strong>输入:</strong> roads = [[0,1],[0,2],[0,3]], seats = 5
<strong>输出:</strong> 3
<strong>解释:</strong>
- 代表 1 直接到达首都，消耗 1 升汽油。
- 代表 2 直接到达首都，消耗 1 升汽油。
- 代表 3 直接到达首都，消耗 1 升汽油。
最少消耗 3 升汽油。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/11/16/2.png)
<pre>
<strong>输入:</strong> roads = [[3,1],[3,2],[1,0],[0,4],[0,5],[4,6]], seats = 2
<strong>输出:</strong> 7
<strong>解释:</strong>
- 代表 2 到达城市 3 ，消耗 1 升汽油。
- 代表 2 和代表 3 一起到达城市 1 ，消耗 1 升汽油。
- 代表 2 和代表 3 一起到达首都，消耗 1 升汽油。
- 代表 1 直接到达首都，消耗 1 升汽油。
- 代表 5 直接到达首都，消耗 1 升汽油。
- 代表 6 到达城市 4 ，消耗 1 升汽油。
- 代表 4 和代表 6 一起到达首都，消耗 1 升汽油。
最少消耗 7 升汽油。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2022/09/27/efcf7f7be6830b8763639cfd01b690a.png)
<pre>
<strong>输入:</strong> roads = [], seats = 1
<strong>输出:</strong> 0
<strong>解释:</strong> 没有代表需要从别的城市到达首都。
</pre>

#### 提示:
* <code>1 <= n <= 10<sup>5</sup></code>
* `roads.length == n - 1`
* `roads[i].length == 2`
* <code>0 <= a<sub>i</sub>, b<sub>i</sub> < n</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>
* `roads` 表示一棵合法的树。
* <code>1 <= seats <= 10<sup>5</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def minimumFuelCost(self, roads: List[List[int]], seats: int) -> int:
        def dfs(i: int, parent: int = -1) -> (int, int):
            fuel = 0
            representatives = 1

            for j in children[i]:
                if j != parent:
                    f, r = dfs(j, i)
                    fuel += f
                    representatives += r

            if i > 0:
                fuel += (representatives + seats - 1) // seats

            return (fuel, representatives)

        n = len(roads) + 1
        children = [[] for _ in range(n)]

        for a, b in roads:
            children[a].append(b)
            children[b].append(a)

        return dfs(0)[0]
```
