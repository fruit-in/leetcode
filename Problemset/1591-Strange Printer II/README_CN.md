# 1591. 奇怪的打印机 II
给你一个奇怪的打印机，它有如下两个特殊的打印规则：
* 每一次操作时，打印机会用同一种颜色打印一个矩形的形状，每次打印会覆盖矩形对应格子里原本的颜色。
* 一旦矩形根据上面的规则使用了一种颜色，那么 **相同的颜色不能再被使用** 。

给你一个初始没有颜色的 `m x n` 的矩形 `targetGrid` ，其中 `targetGrid[row][col]` 是位置 `(row, col)` 的颜色。

如果你能按照上述规则打印出矩形 `targetGrid` ，请你返回 `true` ，否则返回 `false` 。

#### 示例 1:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/09/19/sample_1_1929.png)
<pre>
<strong>输入:</strong> targetGrid = [[1,1,1,1],[1,2,2,1],[1,2,2,1],[1,1,1,1]]
<strong>输出:</strong> true
</pre>

#### 示例 2:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/09/19/sample_2_1929.png)
<pre>
<strong>输入:</strong> targetGrid = [[1,1,1,1],[1,1,3,3],[1,1,3,4],[5,5,1,4]]
<strong>输出:</strong> true
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> targetGrid = [[1,2,1],[2,1,2],[1,2,1]]
<strong>输出:</strong> false
<strong>解释:</strong> 没有办法得到 targetGrid ，因为每一轮操作使用的颜色互不相同。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> targetGrid = [[1,1,1],[3,1,3]]
<strong>输出:</strong> false
</pre>

#### 提示:
* `m == targetGrid.length`
* `n == targetGrid[i].length`
* `1 <= m, n <= 60`
* `1 <= targetGrid[row][col] <= 60`

## 题解 (Python)

### 1. 题解
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
