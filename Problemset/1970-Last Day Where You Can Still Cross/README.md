# 1970. Last Day Where You Can Still Cross
There is a **1-based** binary matrix where `0` represents land and `1` represents water. You are given integers `row` and `col` representing the number of rows and columns in the matrix, respectively.

Initially on day `0`, the **entire** matrix is **land**. However, each day a new cell becomes flooded with **water**. You are given a **1-based** 2D array `cells`, where <code>cells[i] = [r<sub>i</sub>, c<sub>i</sub>]</code> represents that on the <code>i<sup>th</sup></code> day, the cell on the <code>r<sub>i</sub><sup>th</sup></code> row and <code>c<sub>i</sub><sup>th</sup></code> column (**1-based** coordinates) will be covered with **water** (i.e., changed to `1`).

You want to find the **last** day that it is possible to walk from the **top** to the **bottom** by only walking on land cells. You can start from **any** cell in the top row and end at **any** cell in the bottom row. You can only travel in the **four** cardinal directions (left, right, up, and down).

Return *the **last** day where it is possible to walk from the **top** to the **bottom** by only walking on land cells*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/07/27/1.png)
<pre>
<strong>Input:</strong> row = 2, col = 2, cells = [[1,1],[2,1],[1,2],[2,2]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> The above image depicts how the matrix changes each day starting from day 0.
The last day where it is possible to cross from top to bottom is on day 2.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/07/27/2.png)
<pre>
<strong>Input:</strong> row = 2, col = 2, cells = [[1,1],[1,2],[2,1],[2,2]]
<strong>Output:</strong> 1
<strong>Explanation:</strong> The above image depicts how the matrix changes each day starting from day 0.
The last day where it is possible to cross from top to bottom is on day 1.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2021/07/27/3.png)
<pre>
<strong>Input:</strong> row = 3, col = 3, cells = [[1,2],[2,1],[3,3],[2,2],[1,1],[1,3],[2,3],[3,2],[3,1]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The above image depicts how the matrix changes each day starting from day 0.
The last day where it is possible to cross from top to bottom is on day 3.
</pre>

#### Constraints:
* <code>2 <= row, col <= 2 * 10<sup>4</sup></code>
* <code>4 <= row * col <= 2 * 10<sup>4</sup></code>
* `cells.length == row * col`
* <code>1 <= r<sub>i</sub> <= row</code>
* <code>1 <= c<sub>i</sub> <= col</code>
* All the values of `cells` are **unique**.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def latestDayToCross(self, row: int, col: int, cells: List[List[int]]) -> int:
        def cannotCross(day: int) -> bool:
            grid = [[0] * col for _ in range(row)]
            stack = []
            visited = set()

            for r, c in cells[:day]:
                grid[r - 1][c - 1] = 1

            stack.extend((0, c) for c in range(col) if grid[0][c] == 0)
            visited.update((0, c) for c in range(col) if grid[0][c] == 0)

            while stack != []:
                r, c = stack.pop()

                if r == row - 1:
                    return False

                if r - 1 >= 0 and grid[r - 1][c] == 0 and (r - 1, c) not in visited:
                    stack.append((r - 1, c))
                    visited.add((r - 1, c))
                if r + 1 < row and grid[r + 1][c] == 0 and (r + 1, c) not in visited:
                    stack.append((r + 1, c))
                    visited.add((r + 1, c))
                if c - 1 >= 0 and grid[r][c - 1] == 0 and (r, c - 1) not in visited:
                    stack.append((r, c - 1))
                    visited.add((r, c - 1))
                if c + 1 < col and grid[r][c + 1] == 0 and (r, c + 1) not in visited:
                    stack.append((r, c + 1))
                    visited.add((r, c + 1))

            return True

        return bisect.bisect_left(list(range(len(cells))), True, key=cannotCross) - 1
```
