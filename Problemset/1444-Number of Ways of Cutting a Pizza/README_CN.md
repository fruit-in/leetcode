# 1444. 切披萨的方案数
给你一个 `rows x cols` 大小的矩形披萨和一个整数 `k` ，矩形包含两种字符： `'A'` （表示苹果）和 `'.'` （表示空白格子）。你需要切披萨 `k-1` 次，得到 `k` 块披萨并送给别人。

切披萨的每一刀，先要选择是向垂直还是水平方向切，再在矩形的边界上选一个切的位置，将披萨一分为二。如果垂直地切披萨，那么需要把左边的部分送给一个人，如果水平地切，那么需要把上面的部分送给一个人。在切完最后一刀后，需要把剩下来的一块送给最后一个人。

请你返回确保每一块披萨包含 **至少** 一个苹果的切披萨方案数。由于答案可能是个很大的数字，请你返回它对 10^9 + 7 取余的结果。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/04/23/ways_to_cut_apple_1.png)
<pre>
<strong>输入:</strong> pizza = ["A..","AAA","..."], k = 3
<strong>输出:</strong> 3
<strong>解释:</strong> 上图展示了三种切披萨的方案。注意每一块披萨都至少包含一个苹果。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> pizza = ["A..","AA.","..."], k = 3
<strong>输出:</strong> 1
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> pizza = ["A..","A..","..."], k = 1
<strong>输出:</strong> 1
</pre>

#### 提示:
* `1 <= rows, cols <= 50`
* `rows == pizza.length`
* `cols == pizza[i].length`
* `1 <= k <= 10`
* `pizza` 只包含字符 `'A'` 和 `'.'` 。

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def ways(self, pizza: List[str], k: int) -> int:
        @cache
        def subPizzaWays(r: int, c: int, k: int) -> int:
            if applecount[r][c] < k:
                return 0
            if k == 1:
                return 1

            ret = 0

            for i in range(r + 1, rows):
                if applecount[r][c] > applecount[i][c]:
                    ret = (ret + subPizzaWays(i, c, k - 1)) % 1000000007
            for i in range(c + 1, cols):
                if applecount[r][c] > applecount[r][i]:
                    ret = (ret + subPizzaWays(r, i, k - 1)) % 1000000007

            return ret

        rows = len(pizza)
        cols = len(pizza[0])
        applecount = [[0] * (cols) for _ in range(rows)]

        for r in range(rows - 1, -1, -1):
            for c in range(cols - 1, -1, -1):
                if pizza[r][c] == 'A':
                    applecount[r][c] = 1
                if r + 1 < rows:
                    applecount[r][c] += applecount[r + 1][c]
                if c + 1 < cols:
                    applecount[r][c] += applecount[r][c + 1]
                if r + 1 < rows and c + 1 < cols:
                    applecount[r][c] -= applecount[r + 1][c + 1]

        return subPizzaWays(0, 0, k)
```
