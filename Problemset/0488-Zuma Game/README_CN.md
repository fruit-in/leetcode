# 488. 祖玛游戏
你正在参与祖玛游戏的一个变种。

在这个祖玛游戏变体中，桌面上有 **一排** 彩球，每个球的颜色可能是：红色 `'R'`、黄色 `'Y'`、蓝色 `'B'`、绿色 `'G'` 或白色 `'W'` 。你的手中也有一些彩球。

你的目标是 **清空** 桌面上所有的球。每一回合：
* 从你手上的彩球中选出 **任意一颗** ，然后将其插入桌面上那一排球中：两球之间或这一排球的任一端。
* 接着，如果有出现 **三个或者三个以上** 且 **颜色相同** 的球相连的话，就把它们移除掉。
    * 如果这种移除操作同样导致出现三个或者三个以上且颜色相同的球相连，则可以继续移除这些球，直到不再满足移除条件。
* 如果桌面上所有球都被移除，则认为你赢得本场游戏。
* 重复这个过程，直到你赢了游戏或者手中没有更多的球。

给你一个字符串 `board` ，表示桌面上最开始的那排球。另给你一个字符串 `hand` ，表示手里的彩球。请你按上述操作步骤移除掉桌上所有球，计算并返回所需的 **最少** 球数。如果不能移除桌上所有的球，返回 `-1` 。

#### 示例 1:
<pre>
<strong>输入:</strong> board = "WRRBBW", hand = "RB"
<strong>输出:</strong> -1
<strong>解释:</strong> 无法移除桌面上的所有球。可以得到的最好局面是：
- 插入一个 'R' ，使桌面变为 WRRRBBW 。WRRRBBW -> WBBW
- 插入一个 'B' ，使桌面变为 WBBBW 。WBBBW -> WW
桌面上还剩着球，没有其他球可以插入。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> board = "WWRRBBWW", hand = "WRBRW"
<strong>输出:</strong> 2
<strong>解释:</strong> 要想清空桌面上的球，可以按下述步骤：
- 插入一个 'R' ，使桌面变为 WWRRRBBWW 。WWRRRBBWW -> WWBBWW
- 插入一个 'B' ，使桌面变为 WWBBBWW 。WWBBBWW -> WWWW -> empty
只需从手中出 2 个球就可以清空桌面。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> board = "G", hand = "GGGGG"
<strong>输出:</strong> 2
<strong>解释:</strong> 要想清空桌面上的球，可以按下述步骤：
- 插入一个 'G' ，使桌面变为 GG 。
- 插入一个 'G' ，使桌面变为 GGG 。GGG -> empty
只需从手中出 2 个球就可以清空桌面。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> board = "RBYYBBRRB", hand = "YRBGB"
<strong>输出:</strong> 3
<strong>解释:</strong> 要想清空桌面上的球，可以按下述步骤：
- 插入一个 'Y' ，使桌面变为 RBYYYBBRRB 。RBYYYBBRRB -> RBBBRRB -> RRRB -> B
- 插入一个 'B' ，使桌面变为 BB 。
- 插入一个 'B' ，使桌面变为 BBB 。BBB -> empty
只需从手中出 3 个球就可以清空桌面。
</pre>

#### 提示:
* `1 <= board.length <= 16`
* `1 <= hand.length <= 5`
* `board` 和 `hand` 由字符 `'R'`、`'Y'`、`'B'`、`'G'` 和 `'W'` 组成
* 桌面上一开始的球中，不会有三个及三个以上颜色相同且连着的球

## 题解 (Python)

### 1. 题解
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
