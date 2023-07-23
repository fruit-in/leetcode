# 2319. Check if Matrix Is X-Matrix
A square matrix is said to be an **X-Matrix** if **both** of the following conditions hold:
* All the elements in the diagonals of the matrix are **non-zero**.
* All other elements are 0.

Given a 2D integer array `grid` of size `n x n` representing a square matrix, return `true` *if* `grid` *is an X-Matrix*. Otherwise, return `false`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/05/03/ex1.jpg)
<pre>
<strong>Input:</strong> grid = [[2,0,0,1],[0,3,1,0],[0,5,2,0],[4,0,0,2]]
<strong>Output:</strong> true
<strong>Explanation:</strong> Refer to the diagram above.
An X-Matrix should have the green elements (diagonals) be non-zero and the red elements be 0.
Thus, grid is an X-Matrix.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/05/03/ex2.jpg)
<pre>
<strong>Input:</strong> grid = [[5,7,0],[0,3,1],[0,5,0]]
<strong>Output:</strong> false
<strong>Explanation:</strong> Refer to the diagram above.
An X-Matrix should have the green elements (diagonals) be non-zero and the red elements be 0.
Thus, grid is not an X-Matrix.
</pre>

#### Constraints:
* `n == grid.length == grid[i].length`
* `3 <= n <= 100`
* <code>0 <= grid[i][j] <= 10<sup>5</sup></code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def checkXMatrix(self, grid: List[List[int]]) -> bool:
        for x in range(len(grid)):
            for y in range(len(grid)):
                if (x == y or x == len(grid) - 1 - y) ^ (grid[x][y] != 0):
                    return False

        return True
```
