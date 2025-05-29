# 2018. Check if Word Can Be Placed In Crossword
You are given an `m x n` matrix `board`, representing the **current** state of a crossword puzzle. The crossword contains lowercase English letters (from solved words), `' '` to represent any **empty** cells, and `'#'` to represent any **blocked** cells.

A word can be placed **horizontally** (left to right **or** right to left) or **vertically** (top to bottom **or** bottom to top) in the board if:
* It does not occupy a cell containing the character `'#'`.
* The cell each letter is placed in must either be `' '` (empty) or **match** the letter already on the `board`.
* There must not be any empty cells `' '` or other lowercase letters **directly left or right** of the word if the word was placed **horizontally**.
* There must not be any empty cells `' '` or other lowercase letters **directly above or below** the word if the word was placed **vertically**.

Given a string `word`, return `true` *if* `word` *can be placed in* `board`*, or* `false` ***otherwise***.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/10/04/crossword-ex1-1.png)
<pre>
<strong>Input:</strong> board = [["#", " ", "#"], [" ", " ", "#"], ["#", "c", " "]], word = "abc"
<strong>Output:</strong> true
<strong>Explanation:</strong> The word "abc" can be placed as shown above (top to bottom).
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/10/04/crossword-ex2-1.png)
<pre>
<strong>Input:</strong> board = [[" ", "#", "a"], [" ", "#", "c"], [" ", "#", "a"]], word = "ac"
<strong>Output:</strong> false
<strong>Explanation:</strong> It is impossible to place the word because there will always be a space/letter above or below it.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2021/10/04/crossword-ex3-1.png)
<pre>
<strong>Input:</strong> board = [["#", " ", "#"], [" ", " ", "#"], ["#", " ", "c"]], word = "ca"
<strong>Output:</strong> true
<strong>Explanation:</strong> The word "ca" can be placed as shown above (right to left).
</pre>

#### Constraints:
* `m == board.length`
* `n == board[i].length`
* <code>1 <= m * n <= 2 * 10<sup>5</sup></code>
* `board[i][j]` will be `' '`, `'#'`, or a lowercase English letter.
* `1 <= word.length <= max(m, n)`
* `word` will contain only lowercase English letters.

## Solutions (Python)

### 1. Solution
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
