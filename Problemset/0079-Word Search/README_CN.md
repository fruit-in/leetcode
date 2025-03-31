# 79. 单词搜索
给定一个 `m x n` 二维字符网格 `board` 和一个字符串单词 `word` 。如果 `word` 存在于网格中，返回 `true` ；否则，返回 `false` 。

单词必须按照字母顺序，通过相邻的单元格内的字母构成，其中“相邻”单元格是那些水平相邻或垂直相邻的单元格。同一个单元格内的字母不允许被重复使用。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/11/04/word2.jpg)
<pre>
<strong>输入:</strong> board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
<strong>输出:</strong> true
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2020/11/04/word-1.jpg)
<pre>
<strong>输入:</strong> board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "SEE"
<strong>输出:</strong> true
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2020/10/15/word3.jpg)
<pre>
<strong>输入:</strong> board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCB"
<strong>输出:</strong> false
</pre>

#### 提示:
* `m == board.length`
* `n = board[i].length`
* `1 <= m, n <= 6`
* `1 <= word.length <= 15`
* `board` 和 `word` 仅由大小写英文字母组成

**进阶：**你可以使用搜索剪枝的技术来优化解决方案，使其在 `board` 更大的情况下可以更快解决问题？

## 题解 (Python)

### 1. 题解
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
