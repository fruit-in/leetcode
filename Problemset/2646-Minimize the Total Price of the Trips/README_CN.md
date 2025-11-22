# 2646. 最小化旅行的价格总和
现有一棵无向、无根的树，树中有 `n` 个节点，按从 `0` 到 `n - 1` 编号。给你一个整数 `n` 和一个长度为 `n - 1` 的二维整数数组 `edges` ，其中 <code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> 表示树中节点 <code>a<sub>i</sub></code> 和 <code>b<sub>i</sub></code> 之间存在一条边。

每个节点都关联一个价格。给你一个整数数组 `price` ，其中 `price[i]` 是第 `i` 个节点的价格。

给定路径的 **价格总和** 是该路径上所有节点的价格之和。

另给你一个二维整数数组 `trips` ，其中 <code>trips[i] = [start<sub>i</sub>, end<sub>i</sub>]</code> 表示您从节点 <code>start<sub>i</sub></code> 开始第 `i` 次旅行，并通过任何你喜欢的路径前往节点 <code>end<sub>i</sub></code> 。

在执行第一次旅行之前，你可以选择一些 **非相邻节点** 并将价格减半。

返回执行所有旅行的最小价格总和。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2023/03/16/diagram2.png)
<pre>
<strong>输入:</strong> n = 4, edges = [[0,1],[1,2],[1,3]], price = [2,2,10,6], trips = [[0,3],[2,1],[2,3]]
<strong>输出:</strong> 23
<strong>解释:</strong>
上图表示将节点 2 视为根之后的树结构。第一个图表示初始树，第二个图表示选择节点 0 、2 和 3 并使其价格减半后的树。
第 1 次旅行，选择路径 [0,1,3] 。路径的价格总和为 1 + 2 + 3 = 6 。
第 2 次旅行，选择路径 [2,1] 。路径的价格总和为 2 + 5 = 7 。
第 3 次旅行，选择路径 [2,1,3] 。路径的价格总和为 5 + 2 + 3 = 10 。
所有旅行的价格总和为 6 + 7 + 10 = 23 。可以证明，23 是可以实现的最小答案。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2023/03/16/diagram3.png)
<pre>
<strong>输入:</strong> n = 2, edges = [[0,1]], price = [2,2], trips = [[0,0]]
<strong>输出:</strong> 1
<strong>解释:</strong>
上图表示将节点 0 视为根之后的树结构。第一个图表示初始树，第二个图表示选择节点 0 并使其价格减半后的树。
第 1 次旅行，选择路径 [0] 。路径的价格总和为 1 。
所有旅行的价格总和为 1 。可以证明，1 是可以实现的最小答案。
</pre>

#### 提示:
* `1 <= n <= 50`
* `edges.length == n - 1`
* <code>0 <= a<sub>i</sub>, b<sub>i</sub> <= n - 1</code>
* `edges` 表示一棵有效的树
* `price.length == n`
* `price[i]` 是一个偶数
* `1 <= price[i] <= 1000`
* `1 <= trips.length <= 100`
* <code>0 <= start<sub>i</sub>, end<sub>i</sub> <= n - 1</code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def minimumTotalPrice(self, n: int, edges: List[List[int]], price: List[int], trips: List[List[int]]) -> int:
        def searchAndCount(start: int, end: int, prev: int = -1) -> bool:
            if start == end:
                count[start] += 1
                return True

            for neighbor in neighbors[start]:
                if neighbor != prev and searchAndCount(neighbor, end, start):
                    count[start] += 1
                    return True

            return False

        @cache
        def calculatePrice(root: int, canhalve: bool, prev: int = -1) -> int:
            price1 = inf
            price2 = price[root] * count[root]

            if canhalve and count[root] > 0:
                price1 = price2 // 2

                for neighbor in neighbors[root]:
                    if neighbor != prev:
                        price1 += calculatePrice(neighbor, False, root)

            for neighbor in neighbors[root]:
                if neighbor != prev:
                    price2 += min(calculatePrice(neighbor, False, root),
                                  calculatePrice(neighbor, True, root))

            return min(price1, price2)

        neighbors = [[] for _ in range(n)]
        count = [0] * n

        for a, b in edges:
            neighbors[a].append(b)
            neighbors[b].append(a)

        for start, end in trips:
            searchAndCount(start, end)

        return calculatePrice(0, True)
```
