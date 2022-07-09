# 52. N-Queens II
The **n-queens** puzzle is the problem of placing `n` queens on an `n x n` chessboard such that no two queens attack each other.

Given an integer `n`, return *the number of distinct solutions to the **n-queens puzzle***.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/11/13/queens.jpg)
<pre>
<strong>Input:</strong> n = 4
<strong>Output:</strong> 2
<strong>Explanation:</strong> There are two distinct solutions to the 4-queens puzzle as shown.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 1
<strong>Output:</strong> 1
</pre>

#### Constraints:
* `1 <= n <= 9`

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def totalNQueens(self, n: int) -> int:
        ret = 0

        for ys in itertools.permutations(range(n)):
            board = [[False] * n for _ in range(n)]
            for x, y in zip(range(n), ys):
                if any(board[x - i][y - i] for i in range(1, min(x, y) + 1)) or \
                        any(board[x - i][y + i] for i in range(1, min(x, n - y - 1) + 1)):
                    break
                board[x][y] = True
            else:
                ret += 1

        return ret
```
