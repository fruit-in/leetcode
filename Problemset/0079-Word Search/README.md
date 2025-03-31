# 79. Word Search
Given an `m x n` grid of characters `board` and a string `word`, return `true` *if* `word` *exists in the grid*.

The word can be constructed from letters of sequentially adjacent cells, where adjacent cells are horizontally or vertically neighboring. The same letter cell may not be used more than once.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/11/04/word2.jpg)
<pre>
<strong>Input:</strong> board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
<strong>Output:</strong> true
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/11/04/word-1.jpg)
<pre>
<strong>Input:</strong> board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "SEE"
<strong>Output:</strong> true
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2020/10/15/word3.jpg)
<pre>
<strong>Input:</strong> board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCB"
<strong>Output:</strong> false
</pre>

#### Constraints:
* `m == board.length`
* `n = board[i].length`
* `1 <= m, n <= 6`
* `1 <= word.length <= 15`
* `board` and `word` consists of only lowercase and uppercase English letters.

**Follow up:** Could you use search pruning to make your solution faster with a larger `board`?

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def exist(self, board: List[List[str]], word: str) -> bool:
        m, n = len(board), len(board[0])

        for r in range(m):
            for c in range(n):
                if board[r][c] != word[0]:
                    continue

                visited = {(r, c)}
                stack = [[r, c, 0, 0]]

                while stack != []:
                    i, j, k, l = stack[-1]

                    if k == len(word) - 1:
                        return True

                    if l == 0:
                        stack[-1][3] += 1
                        if i > 0 and (i - 1, j) not in visited and board[i - 1][j] == word[k + 1]:
                            visited.add((i - 1, j))
                            stack.append([i - 1, j, k + 1, 0])
                    elif l == 1:
                        stack[-1][3] += 1
                        if i < m - 1 and (i + 1, j) not in visited and board[i + 1][j] == word[k + 1]:
                            visited.add((i + 1, j))
                            stack.append([i + 1, j, k + 1, 0])
                    elif l == 2:
                        stack[-1][3] += 1
                        if j > 0 and (i, j - 1) not in visited and board[i][j - 1] == word[k + 1]:
                            visited.add((i, j - 1))
                            stack.append([i, j - 1, k + 1, 0])
                    elif l == 3:
                        stack[-1][3] += 1
                        if j < n - 1 and (i, j + 1) not in visited and board[i][j + 1] == word[k + 1]:
                            visited.add((i, j + 1))
                            stack.append([i, j + 1, k + 1, 0])
                    else:
                        stack.pop()
                        visited.remove((i, j))

        return False
```
