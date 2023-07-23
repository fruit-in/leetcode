# 51. N 皇后
按照国际象棋的规则，皇后可以攻击与之处在同一行或同一列或同一斜线上的棋子。

**n 皇后问题** 研究的是如何将 `n` 个皇后放置在 `n×n` 的棋盘上，并且使皇后彼此之间不能相互攻击。

给你一个整数 `n` ，返回所有不同的 **n 皇后问题** 的解决方案。

每一种解法包含一个不同的 **n 皇后问题** 的棋子放置方案，该方案中 `'Q'` 和 `'.'` 分别代表了皇后和空位。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/11/13/queens.jpg)
<pre>
<strong>输入:</strong> n = 4
<strong>输出:</strong> [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
<strong>解释:</strong> 如上图所示，4 皇后问题存在两个不同的解法。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 1
<strong>输出:</strong> [["Q"]]
</pre>

#### 提示:
* `1 <= n <= 9`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def solveNQueens(self, n: int) -> List[List[str]]:
        ret = []

        for ys in itertools.permutations(range(n)):
            board = [['.'] * n for _ in range(n)]
            for x, y in zip(range(n), ys):
                if any(board[x - i][y - i] == 'Q' for i in range(1, min(x, y) + 1)) or \
                        any(board[x - i][y + i] == 'Q' for i in range(1, min(x, n - y - 1) + 1)):
                    break
                board[x][y] = 'Q'
            else:
                ret.append([''.join(row) for row in board])

        return ret
```
