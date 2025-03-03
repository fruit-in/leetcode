# 37. Sudoku Solver
Write a program to solve a Sudoku puzzle by filling the empty cells.

A sudoku solution must satisfy **all of the following rules**:
1. Each of the digits `1-9` must occur exactly once in each row.
2. Each of the digits `1-9` must occur exactly once in each column.
3. Each of the digits `1-9` must occur exactly once in each of the 9 `3x3` sub-boxes of the grid.

The `'.'` character indicates empty cells.

#### Example 1:
![](https://upload.wikimedia.org/wikipedia/commons/thumb/f/ff/Sudoku-by-L2G-20050714.svg/250px-Sudoku-by-L2G-20050714.svg.png)
<pre>
<strong>Input:</strong> board = [["5","3",".",".","7",".",".",".","."],["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],[".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]]
<strong>Output:</strong> [["5","3","4","6","7","8","9","1","2"],["6","7","2","1","9","5","3","4","8"],["1","9","8","3","4","2","5","6","7"],["8","5","9","7","6","1","4","2","3"],["4","2","6","8","5","3","7","9","1"],["7","1","3","9","2","4","8","5","6"],["9","6","1","5","3","7","2","8","4"],["2","8","7","4","1","9","6","3","5"],["3","4","5","2","8","6","1","7","9"]]
<strong>Explanation:</strong> The input board is shown above and the only valid solution is shown below:
<img src="https://upload.wikimedia.org/wikipedia/commons/thumb/3/31/Sudoku-by-L2G-20050714_solution.svg/250px-Sudoku-by-L2G-20050714_solution.svg.png">
</pre>

#### Constraints:
* `board.length == 9`
* `board[i].length == 9`
* `board[i][j]` is a digit or `'.'`.
* It is **guaranteed** that the input board has only one solution.

## Solutions (Python)

### 1. Solution
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
