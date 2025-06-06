# 1591. Strange Printer II
There is a strange printer with the following two special requirements:
* On each turn, the printer will print a solid rectangular pattern of a single color on the grid. This will cover up the existing colors in the rectangle.
* Once the printer has used a color for the above operation, **the same color cannot be used again**.

You are given a `m x n` matrix `targetGrid`, where `targetGrid[row][col]` is the color in the position `(row, col)` of the grid.

Return `true` *if it is possible to print the matrix* `targetGrid`*, otherwise, return* `false`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/12/23/print1.jpg)
<pre>
<strong>Input:</strong> targetGrid = [[1,1,1,1],[1,2,2,1],[1,2,2,1],[1,1,1,1]]
<strong>Output:</strong> true
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/12/23/print2.jpg)
<pre>
<strong>Input:</strong> targetGrid = [[1,1,1,1],[1,1,3,3],[1,1,3,4],[5,5,1,4]]
<strong>Output:</strong> true
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> targetGrid = [[1,2,1],[2,1,2],[1,2,1]]
<strong>Output:</strong> false
<strong>Explanation:</strong> It is impossible to form targetGrid because it is not allowed to print the same color in different turns.
</pre>

#### Constraints:
* `m == targetGrid.length`
* `n == targetGrid[i].length`
* `1 <= m, n <= 60`
* `1 <= targetGrid[row][col] <= 60`

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def isPrintable(self, targetGrid: List[List[int]]) -> bool:
        m, n = len(targetGrid), len(targetGrid[0])
        position = {targetGrid[r][c]: [r, r, c, c]
                    for r in range(m) for c in range(n)}
        colorgrid = {color: set() for color in position}
        belongto = [[set() for _ in range(n)] for _ in range(m)]
        count0 = {color: 0 for color in position}
        stack = []

        for r in range(m):
            for c, color in enumerate(targetGrid[r]):
                position[color][0] = min(position[color][0], r)
                position[color][1] = max(position[color][1], r)
                position[color][2] = min(position[color][2], c)
                position[color][3] = max(position[color][3], c)
                colorgrid[color].add((r, c))

        for r in range(m):
            for c in range(n):
                for color, [top, bottom, left, right] in position.items():
                    if top <= r <= bottom and left <= c <= right:
                        belongto[r][c].add(color)

        for color, [top, bottom, left, right] in position.items():
            if (bottom - top + 1) * (right - left + 1) == len(colorgrid[color]):
                stack.append(color)

        while stack:
            for r, c in colorgrid[stack.pop()]:
                targetGrid[r][c] = 0

                for color in belongto[r][c]:
                    top, bottom, left, right = position[color]
                    count0[color] += 1
                    if count0[color] + len(colorgrid[color]) == (bottom - top + 1) * (right - left + 1):
                        stack.append(color)

        return all(targetGrid[r][c] == 0 for r in range(m) for c in range(n))
```
