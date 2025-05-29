# 2018. 判断单词是否能放入填字游戏内
给你一个 `m x n` 的矩阵 `board` ，它代表一个填字游戏 **当前** 的状态。填字游戏格子中包含小写英文字母（已填入的单词），表示 空 格的 `' '` 和表示 **障碍** 格子的 `'#'` 。

如果满足以下条件，那么我们可以 **水平** （从左到右 **或者** 从右到左）或 **竖直** （从上到下 **或者** 从下到上）填入一个单词：
* 该单词不占据任何 `'#'` 对应的格子。
* 每个字母对应的格子要么是 `' '` （空格）要么与 `board` 中已有字母 **匹配** 。
* 如果单词是 **水平** 放置的，那么该单词左边和右边 **相邻** 格子不能为 `' '` 或小写英文字母。
* 如果单词是 **竖直** 放置的，那么该单词上边和下边 **相邻** 格子不能为 `' '` 或小写英文字母。

给你一个字符串 `word` ，如果 `word` 可以被放入 `board` 中，请你返回 `true` ，否则请返回 `false` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/09/18/crossword-1.png)
<pre>
<strong>输入:</strong> board = [["#", " ", "#"], [" ", " ", "#"], ["#", "c", " "]], word = "abc"
<strong>输出:</strong> true
<strong>解释:</strong> 单词 "abc" 可以如上图放置（从上往下）。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/09/18/c2.png)
<pre>
<strong>输入:</strong> board = [[" ", "#", "a"], [" ", "#", "c"], [" ", "#", "a"]], word = "ac"
<strong>输出:</strong> false
<strong>解释:</strong> 无法放置单词，因为放置该单词后上方或者下方相邻格会有空格。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2021/09/18/crossword-2.png)
<pre>
<strong>输入:</strong> board = [["#", " ", "#"], [" ", " ", "#"], ["#", " ", "c"]], word = "ca"
<strong>输出:</strong> true
<strong>解释:</strong> 单词 "ca" 可以如上图放置（从右到左）。
</pre>

#### 提示:
* `m == board.length`
* `n == board[i].length`
* <code>1 <= m * n <= 2 * 10<sup>5</sup></code>
* `board[i][j]` 可能为 `' '` ，`'#'` 或者一个小写英文字母。
* `1 <= word.length <= max(m, n)`
* `word` 只包含小写英文字母。

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def placeWordInCrossword(self, board: List[List[str]], word: str) -> bool:
        def rotate() -> None:
            nonlocal board
            board = [list(row)[::-1] for row in zip(*board)]

        def placeWord() -> bool:
            for row in board:
                for s in ''.join(row).split('#'):
                    if len(s) == len(word) and all(x == y or x == ' ' for x, y in zip(s, word)):
                        return True

            return False

        return placeWord() or rotate() or placeWord() or rotate() or placeWord() or rotate() or placeWord()
```
