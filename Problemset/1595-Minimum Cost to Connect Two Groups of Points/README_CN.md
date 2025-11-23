# 1595. 连通两组点的最小成本
给你两组点，其中第一组中有 <code>size<sub>1</sub></code> 个点，第二组中有 <code>size2</code> 个点，且 <code>size<sub>1</sub> >= size<sub>2</sub></code> 。

任意两点间的连接成本 `cost` 由大小为 <code>size<sub>1</sub> x size<sub>2</sub></code> 矩阵给出，其中 `cost[i][j]` 是第一组中的点 `i` 和第二组中的点 `j` 的连接成本。**如果两个组中的每个点都与另一组中的一个或多个点连接，则称这两组点是连通的。**换言之，第一组中的每个点必须至少与第二组中的一个点连接，且第二组中的每个点必须至少与第一组中的一个点连接。

返回连通两组点所需的最小成本。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/09/03/ex1.jpg)
<pre>
<strong>输入:</strong> cost = [[15, 96], [36, 2]]
<strong>输出:</strong> 17
<strong>解释:</strong> 连通两组点的最佳方法是：
1--A
2--B
总成本为 17 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2020/09/03/ex2.jpg)
<pre>
<strong>输入:</strong> cost = [[1, 3, 5], [4, 1, 1], [1, 5, 3]]
<strong>输出:</strong> 4
<strong>解释:</strong> 连通两组点的最佳方法是：
1--A
2--B
2--C
3--A
最小成本为 4 。
请注意，虽然有多个点连接到第一组中的点 2 和第二组中的点 A ，但由于题目并不限制连接点的数目，所以只需要关心最低总成本。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> cost = [[2, 5, 1], [3, 4, 7], [8, 1, 2], [6, 2, 4], [3, 8, 8]]
<strong>输出:</strong> 10
</pre>

#### 提示:
* <code>size<sub>1</sub> == cost.length</code>
* <code>size<sub>2</sub> == cost[i].length</code>
* <code>1 <= size<sub>1</sub>, size<sub>2</sub> <= 12</code>
* <code>size<sub>1</sub> >= size<sub>2</sub></code>
* `0 <= cost[i][j] <= 100`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def connectTwoGroups(self, cost: List[List[int]]) -> int:
        @cache
        def dfs(mask: int, i: int, j: int) -> int:
            if i == size1:
                return 0 if mask == (1 << size2) - 1 else inf
            if j == size2:
                return inf

            ret = dfs(mask, i, j + 1)
            mask |= 1 << j
            ret = min(ret, cost[i][j] +
                      min(dfs(mask, i + 1, 0), dfs(mask, i, j + 1)))

            return ret

        size1 = len(cost)
        size2 = len(cost[0])

        return dfs(0, 0, 0)
```
