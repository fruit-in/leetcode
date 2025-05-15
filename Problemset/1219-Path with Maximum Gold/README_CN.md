# 1219. 黄金矿工
你要开发一座金矿，地质勘测学家已经探明了这座金矿中的资源分布，并用大小为 `m * n` 的网格 `grid` 进行了标注。每个单元格中的整数就表示这一单元格中的黄金数量；如果该单元格是空的，那么就是 `0`。

为了使收益最大化，矿工需要按以下规则来开采黄金：
* 每当矿工进入一个单元，就会收集该单元格中的所有黄金。
* 矿工每次可以从当前位置向上下左右四个方向走。
* 每个单元格只能被开采（进入）一次。
* 不得开采（进入）黄金数目为 `0` 的单元格。
* 矿工可以从网格中 **任意一个** 有黄金的单元格出发或者是停止。

#### 示例 1:
<pre>
<strong>输入:</strong> grid = [[0,6,0],[5,8,7],[0,9,0]]
<strong>输出:</strong> 24
<strong>解释:</strong>
[[0,6,0],
 [5,8,7],
 [0,9,0]]
一种收集最多黄金的路线是：9 -> 8 -> 7。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> grid = [[1,0,7],[2,0,6],[3,4,5],[0,3,0],[9,0,20]]
<strong>输出:</strong> 28
<strong>解释:</strong>
[[1,0,7],
 [2,0,6],
 [3,4,5],
 [0,3,0],
 [9,0,20]]
一种收集最多黄金的路线是：1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7。
</pre>

#### 提示:
* `1 <= grid.length, grid[i].length <= 15`
* `0 <= grid[i][j] <= 100`
* 最多 **25** 个单元格中有黄金。

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def getMaximumGold(self, grid: List[List[int]]) -> int:
        m, n = len(grid), len(grid[0])
        gold = 0
        ret = 0

        def dfs(i: int, j: int) -> None:
            nonlocal gold, ret
            ret = max(ret, gold)

            if i > 0 and grid[i - 1][j] > 0:
                gold += grid[i - 1][j]
                grid[i - 1][j] = -grid[i - 1][j]
                dfs(i - 1, j)
                gold += grid[i - 1][j]
                grid[i - 1][j] = -grid[i - 1][j]
            if i + 1 < m and grid[i + 1][j] > 0:
                gold += grid[i + 1][j]
                grid[i + 1][j] = -grid[i + 1][j]
                dfs(i + 1, j)
                gold += grid[i + 1][j]
                grid[i + 1][j] = -grid[i + 1][j]
            if j > 0 and grid[i][j - 1] > 0:
                gold += grid[i][j - 1]
                grid[i][j - 1] = -grid[i][j - 1]
                dfs(i, j - 1)
                gold += grid[i][j - 1]
                grid[i][j - 1] = -grid[i][j - 1]
            if j + 1 < n and grid[i][j + 1] > 0:
                gold += grid[i][j + 1]
                grid[i][j + 1] = -grid[i][j + 1]
                dfs(i, j + 1)
                gold += grid[i][j + 1]
                grid[i][j + 1] = -grid[i][j + 1]

        for i in range(m):
            for j in range(n):
                if grid[i][j] > 0:
                    gold = grid[i][j]
                    grid[i][j] = -grid[i][j]
                    dfs(i, j)
                    grid[i][j] = -grid[i][j]

        return ret
```
