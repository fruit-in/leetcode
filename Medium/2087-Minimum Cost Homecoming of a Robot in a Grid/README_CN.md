# 2087. 网格图中机器人回家的最小代价
给你一个 `m x n` 的网格图，其中 `(0, 0)` 是最左上角的格子，`(m - 1, n - 1)` 是最右下角的格子。给你一个整数数组 `startPos` ，<code>startPos = [start<sub>row</sub>, start<sub>col</sub>]</code> 表示 **初始** 有一个 **机器人** 在格子 <code>(start<sub>row</sub>, start<sub>col</sub>)</code> 处。同时给你一个整数数组 `homePos` ，<code>homePos = [home<sub>row</sub>, home<sub>col</sub>]</code> 表示机器人的 **家** 在格子 <code>(home<sub>row</sub>, home<sub>col</sub>)</code> 处。

机器人需要回家。每一步它可以往四个方向移动：上，下，左，右，同时机器人不能移出边界。每一步移动都有一定代价。再给你两个下标从 0 开始的额整数数组：长度为 `m` 的数组 `rowCosts`  和长度为 `n` 的数组 `colCosts` 。

* 如果机器人往 上 或者往 下 移动到第 `r` 行 的格子，那么代价为 `rowCosts[r]` 。
* 如果机器人往 左 或者往 右 移动到第 `c` 列 的格子，那么代价为 `colCosts[c]` 。

请你返回机器人回家需要的 **最小总代价** 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/10/11/eg-1.png)
<pre>
<strong>输入:</strong> startPos = [1, 0], homePos = [2, 3], rowCosts = [5, 4, 3], colCosts = [8, 2, 6, 7]
<strong>输出:</strong> 18
<strong>解释:</strong> 一个最优路径为：
从 (1, 0) 开始
-> 往下走到 (2, 0) 。代价为 rowCosts[2] = 3 。
-> 往右走到 (2, 1) 。代价为 colCosts[1] = 2 。
-> 往右走到 (2, 2) 。代价为 colCosts[2] = 6 。
-> 往右走到 (2, 3) 。代价为 colCosts[3] = 7 。
总代价为 3 + 2 + 6 + 7 = 18
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> startPos = [0, 0], homePos = [0, 0], rowCosts = [5], colCosts = [26]
<strong>输出:</strong> 0
<strong>解释:</strong> 机器人已经在家了，所以不需要移动。总代价为 0 。
</pre>

#### 提示:
* `m == rowCosts.length`
* `n == colCosts.length`
* <code>1 <= m, n <= 10<sup>5</sup></code>
* <code>0 <= rowCosts[r], colCosts[c] <= 10<sup>4</sup></code>
* `startPos.length == 2`
* `homePos.length == 2`
* <code>0 <= start<sub>row</sub>, home<sub>row</sub> < m</code>
* <code>0 <= start<sub>col</sub>, home<sub>col</sub> < n</code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def minCost(self, startPos: List[int], homePos: List[int], rowCosts: List[int], colCosts: List[int]) -> int:
        rowDir = 1 if homePos[0] < startPos[0] else - 1
        colDir = 1 if homePos[1] < startPos[1] else - 1
        rowSum = sum(rowCosts[homePos[0]:startPos[0]:rowDir])
        colSum = sum(colCosts[homePos[1]:startPos[1]:colDir])

        return rowSum + colSum
```
