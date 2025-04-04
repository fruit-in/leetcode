# 980. Unique Paths III
You are given an `m x n` integer array `grid` where `grid[i][j]` could be:
* `1` representing the starting square. There is exactly one starting square.
* `2` representing the ending square. There is exactly one ending square.
* `0` representing empty squares we can walk over.
* `-1` representing obstacles that we cannot walk over.

Return *the number of 4-directional walks from the starting square to the ending square, that walk over every non-obstacle square exactly once*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/08/02/lc-unique1.jpg)
<pre>
<strong>Input:</strong> grid = [[1,0,0,0],[0,0,0,0],[0,0,2,-1]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> We have the following two paths:
1. (0,0),(0,1),(0,2),(0,3),(1,3),(1,2),(1,1),(1,0),(2,0),(2,1),(2,2)
2. (0,0),(1,0),(2,0),(2,1),(1,1),(0,1),(0,2),(0,3),(1,3),(1,2),(2,2)
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/08/02/lc-unique2.jpg)
<pre>
<strong>Input:</strong> grid = [[1,0,0,0],[0,0,0,0],[0,0,0,2]]
<strong>Output:</strong> 4
<strong>Explanation:</strong> We have the following four paths:
1. (0,0),(0,1),(0,2),(0,3),(1,3),(1,2),(1,1),(1,0),(2,0),(2,1),(2,2),(2,3)
2. (0,0),(0,1),(1,1),(1,0),(2,0),(2,1),(2,2),(1,2),(0,2),(0,3),(1,3),(2,3)
3. (0,0),(1,0),(2,0),(2,1),(2,2),(1,2),(1,1),(0,1),(0,2),(0,3),(1,3),(2,3)
4. (0,0),(1,0),(2,0),(2,1),(1,1),(0,1),(0,2),(0,3),(1,3),(1,2),(2,2),(2,3)
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2021/08/02/lc-unique3-.jpg)
<pre>
<strong>Input:</strong> grid = [[0,1],[2,0]]
<strong>Output:</strong> 0
<strong>Explanation:</strong> There is no path that walks over every empty square exactly once.
Note that the starting and ending square can be anywhere in the grid.
</pre>

#### Constraints:
* `m == grid.length`
* `n == grid[i].length`
* `1 <= m, n <= 20`
* `1 <= m * n <= 20`
* `-1 <= grid[i][j] <= 2`
* There is exactly one starting cell and one ending cell.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def uniquePathsIII(self, grid: List[List[int]]) -> int:
        m, n = len(grid), len(grid[0])
        nonobstaclecount = 0
        stack = []
        visited = set()
        ret = 0

        for i in range(m):
            for j in range(n):
                if grid[i][j] == 1:
                    stack.append([i, j, 0])
                    visited.add((i, j))
                if grid[i][j] != -1:
                    nonobstaclecount += 1

        while stack != []:
            i, j, direct = stack[-1]

            if grid[i][j] == 2:
                ret += len(visited) == nonobstaclecount
                stack.pop()
                visited.remove((i, j))
            elif direct == 0:
                stack[-1][2] += 1
                if i > 0 and (i - 1, j) not in visited and grid[i - 1][j] != -1:
                    stack.append([i - 1, j, 0])
                    visited.add((i - 1, j))
            elif direct == 1:
                stack[-1][2] += 1
                if i < m - 1 and (i + 1, j) not in visited and grid[i + 1][j] != -1:
                    stack.append([i + 1, j, 0])
                    visited.add((i + 1, j))
            elif direct == 2:
                stack[-1][2] += 1
                if j > 0 and (i, j - 1) not in visited and grid[i][j - 1] != -1:
                    stack.append([i, j - 1, 0])
                    visited.add((i, j - 1))
            elif direct == 3:
                stack[-1][2] += 1
                if j < n - 1 and (i, j + 1) not in visited and grid[i][j + 1] != -1:
                    stack.append([i, j + 1, 0])
                    visited.add((i, j + 1))
            else:
                stack.pop()
                visited.remove((i, j))

        return ret
```
