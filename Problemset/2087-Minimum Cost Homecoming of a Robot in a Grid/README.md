# 2087. Minimum Cost Homecoming of a Robot in a Grid
There is an `m x n` grid, where `(0, 0)` is the top-left cell and `(m - 1, n - 1)` is the bottom-right cell. You are given an integer array `startPos` where <code>startPos = [start<sub>row</sub>, start<sub>col</sub>]</code> indicates that **initially**, a **robot** is at the cell <code>(start<sub>row</sub>, start<sub>col</sub>)</code>. You are also given an integer array `homePos` where <code>homePos = [home<sub>row</sub>, home<sub>col</sub>]</code> indicates that its **home** is at the cell <code>(home<sub>row</sub>, home<sub>col</sub>)</code>.

The robot needs to go to its home. It can move one cell in four directions: **left**, **right**, **up**, or **down**, and it can not move outside the boundary. Every move incurs some cost. You are further given two 0-indexed integer arrays: `rowCosts` of length `m` and `colCosts` of length `n`.

* If the robot moves **up** or **down** into a cell whose **row** is `r`, then this move costs `rowCosts[r]`.
* If the robot moves **left** or **right** into a cell whose **column** is `c`, then this move costs `colCosts[c]`.

Return *the **minimum total cost** for this robot to return home*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/10/11/eg-1.png)
<pre>
<strong>Input:</strong> startPos = [1, 0], homePos = [2, 3], rowCosts = [5, 4, 3], colCosts = [8, 2, 6, 7]
<strong>Output:</strong> 18
<strong>Explanation:</strong> One optimal path is that:
Starting from (1, 0)
-> It goes down to (2, 0). This move costs rowCosts[2] = 3.
-> It goes right to (2, 1). This move costs colCosts[1] = 2.
-> It goes right to (2, 2). This move costs colCosts[2] = 6.
-> It goes right to (2, 3). This move costs colCosts[3] = 7.
The total cost is 3 + 2 + 6 + 7 = 18
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> startPos = [0, 0], homePos = [0, 0], rowCosts = [5], colCosts = [26]
<strong>Output:</strong> 0
<strong>Explanation:</strong> The robot is already at its home. Since no moves occur, the total cost is 0.
</pre>

#### Constraints:
* `m == rowCosts.length`
* `n == colCosts.length`
* <code>1 <= m, n <= 10<sup>5</sup></code>
* <code>0 <= rowCosts[r], colCosts[c] <= 10<sup>4</sup></code>
* `startPos.length == 2`
* `homePos.length == 2`
* <code>0 <= start<sub>row</sub>, home<sub>row</sub> < m</code>
* <code>0 <= start<sub>col</sub>, home<sub>col</sub> < n</code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def minCost(self, startPos: List[int], homePos: List[int], rowCosts: List[int], colCosts: List[int]) -> int:
        rowDir = 1 if homePos[0] < startPos[0] else - 1
        colDir = 1 if homePos[1] < startPos[1] else - 1
        rowSum = sum(rowCosts[homePos[0]:startPos[0]:rowDir])
        colSum = sum(colCosts[homePos[1]:startPos[1]:colDir])

        return rowSum + colSum
```
