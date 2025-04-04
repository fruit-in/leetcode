# 980. 不同路径 III
在二维网格 `grid` 上，有 4 种类型的方格：
* `1` 表示起始方格。且只有一个起始方格。
* `2` 表示结束方格，且只有一个结束方格。
* `0` 表示我们可以走过的空方格。
* `-1` 表示我们无法跨越的障碍。

返回在四个方向（上、下、左、右）上行走时，从起始方格到结束方格的不同路径的数目。

**每一个无障碍方格都要通过一次，但是一条路径中不能重复通过同一个方格**。

#### 示例 1:
<pre>
<strong>输入:</strong> grid = [[1,0,0,0],[0,0,0,0],[0,0,2,-1]]
<strong>输出:</strong> 2
<strong>解释:</strong> 我们有以下两条路径：
1. (0,0),(0,1),(0,2),(0,3),(1,3),(1,2),(1,1),(1,0),(2,0),(2,1),(2,2)
2. (0,0),(1,0),(2,0),(2,1),(1,1),(0,1),(0,2),(0,3),(1,3),(1,2),(2,2)
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> grid = [[1,0,0,0],[0,0,0,0],[0,0,0,2]]
<strong>输出:</strong> 4
<strong>解释:</strong> 我们有以下四条路径：
1. (0,0),(0,1),(0,2),(0,3),(1,3),(1,2),(1,1),(1,0),(2,0),(2,1),(2,2),(2,3)
2. (0,0),(0,1),(1,1),(1,0),(2,0),(2,1),(2,2),(1,2),(0,2),(0,3),(1,3),(2,3)
3. (0,0),(1,0),(2,0),(2,1),(2,2),(1,2),(1,1),(0,1),(0,2),(0,3),(1,3),(2,3)
4. (0,0),(1,0),(2,0),(2,1),(1,1),(0,1),(0,2),(0,3),(1,3),(1,2),(2,2),(2,3)
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> grid = [[0,1],[2,0]]
<strong>输出:</strong> 0
<strong>解释:</strong>
没有一条路能完全穿过每一个空的方格一次。
请注意，起始和结束方格可以位于网格中的任意位置。
</pre>

#### 提示:
* `1 <= grid.length * grid[0].length <= 20`

## 题解 (Python)

### 1. 题解
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
