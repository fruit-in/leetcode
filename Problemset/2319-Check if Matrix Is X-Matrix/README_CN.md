# 2319. 判断矩阵是否是一个 X 矩阵
如果一个正方形矩阵满足下述 **全部** 条件，则称之为一个 **X 矩阵** ：
* 矩阵对角线上的所有元素都 **不是 0**
* 矩阵中所有其他元素都是 **0**

给你一个大小为 `n x n` 的二维整数数组 `grid` ，表示一个正方形矩阵。如果 `grid` 是一个 **X 矩阵** ，返回 `true` ；否则，返回 `false` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/05/03/ex1.jpg)
<pre>
<strong>输入:</strong> grid = [[2,0,0,1],[0,3,1,0],[0,5,2,0],[4,0,0,2]]
<strong>输出:</strong> true
<strong>解释:</strong> 矩阵如上图所示。
X 矩阵应该满足：绿色元素（对角线上）都不是 0 ，红色元素都是 0 。
因此，grid 是一个 X 矩阵。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/05/03/ex2.jpg)
<pre>
<strong>输入:</strong> grid = [[5,7,0],[0,3,1],[0,5,0]]
<strong>输出:</strong> false
<strong>解释:</strong> 矩阵如上图所示。
X 矩阵应该满足：绿色元素（对角线上）都不是 0 ，红色元素都是 0 。
因此，grid 不是一个 X 矩阵。
</pre>

#### 提示:
* `n == grid.length == grid[i].length`
* `3 <= n <= 100`
* <code>0 <= grid[i][j] <= 10<sup>5</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def checkXMatrix(self, grid: List[List[int]]) -> bool:
        for x in range(len(grid)):
            for y in range(len(grid)):
                if (x == y or x == len(grid) - 1 - y) ^ (grid[x][y] != 0):
                    return False

        return True
```
