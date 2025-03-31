# 488. Zuma Game
You are playing a variation of the game Zuma.

In this variation of Zuma, there is a **single row** of colored balls on a board, where each ball can be colored red `'R'`, yellow `'Y'`, blue `'B'`, green `'G'`, or white `'W'`. You also have several colored balls in your hand.

Your goal is to **clear all** of the balls from the board. On each turn:
* Pick **any** ball from your hand and insert it in between two balls in the row or on either end of the row.
* If there is a group of **three or more consecutive balls** of the **same color**, remove the group of balls from the board.
    * If this removal causes more groups of three or more of the same color to form, then continue removing each group until there are none left.
* If there are no more balls on the board, then you win the game.
* Repeat this process until you either win or do not have any more balls in your hand.

Given a string `board`, representing the row of balls on the board, and a string `hand`, representing the balls in your hand, return *the **minimum** number of balls you have to insert to clear all the balls from the board. If you cannot clear all the balls from the board using the balls in your hand, return* `-1`.

#### Example 1:
<pre>
<strong>Input:</strong> board = "WRRBBW", hand = "RB"
<strong>Output:</strong> -1
<strong>Explanation:</strong> It is impossible to clear all the balls. The best you can do is:
- Insert 'R' so the board becomes WRRRBBW. WRRRBBW -> WBBW.
- Insert 'B' so the board becomes WBBBW. WBBBW -> WW.
There are still balls remaining on the board, and you are out of balls to insert.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> board = "WWRRBBWW", hand = "WRBRW"
<strong>Output:</strong> 2
<strong>Explanation:</strong> To make the board empty:
- Insert 'R' so the board becomes WWRRRBBWW. WWRRRBBWW -> WWBBWW.
- Insert 'B' so the board becomes WWBBBWW. WWBBBWW -> WWWW -> empty.
2 balls from your hand were needed to clear the board.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> board = "G", hand = "GGGGG"
<strong>Output:</strong> 2
<strong>Explanation:</strong> To make the board empty:
- Insert 'G' so the board becomes GG.
- Insert 'G' so the board becomes GGG. GGG -> empty.
2 balls from your hand were needed to clear the board.
</pre>

#### Constraints:
* `1 <= board.length <= 16`
* `1 <= hand.length <= 5`
* `board` and `hand` consist of the characters `'R'`, `'Y'`, `'B'`, `'G'`, and `'W'`.
* The initial row of balls on the board will **not** have any groups of three or more consecutive balls of the same color.

## Solutions (Python)

### 1. Solution
```Python
from functools import cache


class Solution:
    COLORS = "RYBGW"
    INDICES = dict(zip(COLORS, range(5)))

    @cache
    def removeGroups(self, board: str) -> str:
        stack = []

        for i in range(len(board)):
            if stack != [] and board[i] != stack[-1][0]:
                if stack[-1][1] > 2:
                    stack.pop()
            if stack != [] and board[i] == stack[-1][0]:
                stack[-1][1] += 1
            else:
                stack.append([board[i], 1])

        if stack[-1][1] > 2:
            stack.pop()

        return ''.join(ball * count for ball, count in stack)

    @cache
    def backtrack(self, board: str, hand: (int, int, int, int, int)) -> int:
        if board == "":
            return 0

        ret = -1

        for i in range(5):
            if hand[i] == 0:
                continue

            newhand = list(hand)
            newhand[i] -= 1
            newhand = tuple(newhand)

            for j in range(len(board) + 1):
                if j < len(board) and board[j] == self.COLORS[i]:
                    continue
                if j > 0 and j < len(board) and board[j] != board[j - 1] and board[j] != self.COLORS[i] and board[j - 1] != self.COLORS[i]:
                    continue

                newboard = self.removeGroups(
                    board[:j] + self.COLORS[i] + board[j:])
                minballs = self.backtrack(newboard, newhand)
                if minballs != -1 and (ret == -1 or minballs + 1 < ret):
                    ret = minballs + 1

        return ret

    def findMinStep(self, board: str, hand: str) -> int:
        count = [0] * 5

        for ball in hand:
            count[self.INDICES[ball]] += 1

        return self.backtrack(board, tuple(count))
```
