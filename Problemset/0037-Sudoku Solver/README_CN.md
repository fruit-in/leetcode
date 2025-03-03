# 37. 解数独
编写一个程序，通过填充空格来解决数独问题。

数独的解法需 **遵循如下规则**：
1. 数字 `1-9` 在每一行只能出现一次。
2. 数字 `1-9` 在每一列只能出现一次。
3. 数字 `1-9` 在每一个以粗实线分隔的 `3x3` 宫内只能出现一次。（请参考示例图）

数独部分空格内已填入了数字，空白格用 `'.'` 表示。

#### 示例 1:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2021/04/12/250px-sudoku-by-l2g-20050714svg.png)
<pre>
<strong>输入:</strong> board = [["5","3",".",".","7",".",".",".","."],["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],[".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]]
<strong>输出:</strong> [["5","3","4","6","7","8","9","1","2"],["6","7","2","1","9","5","3","4","8"],["1","9","8","3","4","2","5","6","7"],["8","5","9","7","6","1","4","2","3"],["4","2","6","8","5","3","7","9","1"],["7","1","3","9","2","4","8","5","6"],["9","6","1","5","3","7","2","8","4"],["2","8","7","4","1","9","6","3","5"],["3","4","5","2","8","6","1","7","9"]]
<strong>解释:</strong> 输入的数独如上图所示，唯一有效的解决方案如下所示：
<img src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2021/04/12/250px-sudoku-by-l2g-20050714_solutionsvg.png">
</pre>

#### 提示:
* `board.length == 9`
* `board[i].length == 9`
* `board[i][j]` 是一位数字或者 `'.'`
* 题目数据 **保证** 输入数独仅有一个解

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def solveSudoku(self, board: List[List[str]]) -> None:
        """
        Do not return anything, modify board in-place instead.
        """
        emptycells = []
        rows = [0] * 9
        cols = [0] * 9
        boxes = [0] * 9
        i = 0

        for r in range(9):
            for c in range(9):
                if board[r][c] == '.':
                    board[r][c] = '0'
                    emptycells.append((r, c))
                else:
                    x = ord(board[r][c]) - 48
                    rows[r] |= 1 << x
                    cols[c] |= 1 << x
                    boxes[r // 3 * 3 + c // 3] |= 1 << x

        while i < len(emptycells):
            r, c = emptycells[i]
            mask = rows[r] | cols[c] | boxes[r // 3 * 3 + c // 3]
            x = ord(board[r][c]) - 47
            while x <= 9 and mask & (1 << x) != 0:
                x += 1
            if x <= 9:
                board[r][c] = chr(x + 48)
                i += 1
            else:
                board[r][c] = '0'
                i -= 1
                r, c = emptycells[i]
                x = ord(board[r][c]) - 48
            rows[r] ^= 1 << x
            cols[c] ^= 1 << x
            boxes[r // 3 * 3 + c // 3] ^= 1 << x
```
