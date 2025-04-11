# 1510. 石子游戏 IV
Alice 和 Bob 两个人轮流玩一个游戏，Alice 先手。

一开始，有 `n` 个石子堆在一起。每个人轮流操作，正在操作的玩家可以从石子堆里拿走 **任意** 非零 **平方数** 个石子。

如果石子堆里没有石子了，则无法操作的玩家输掉游戏。

给你正整数 `n` ，且已知两个人都采取最优策略。如果 Alice 会赢得比赛，那么返回 `True` ，否则返回 `False` 。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 1
<strong>输出:</strong> true
<strong>解释:</strong> Alice 拿走 1 个石子并赢得胜利，因为 Bob 无法进行任何操作。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 2
<strong>输出:</strong> false
<strong>解释:</strong> Alice 只能拿走 1 个石子，然后 Bob 拿走最后一个石子并赢得胜利（2 -> 1 -> 0）。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 4
<strong>输出:</strong> true
<strong>解释:</strong> n 已经是一个平方数，Alice 可以一次全拿掉 4 个石子并赢得胜利（4 -> 0）。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> n = 7
<strong>输出:</strong> false
<strong>解释:</strong> 当 Bob 采取最优策略时，Alice 无法赢得比赛。
如果 Alice 一开始拿走 4 个石子， Bob 会拿走 1 个石子，然后 Alice 只能拿走 1 个石子，Bob 拿走最后一个石子并赢得胜利（7 -> 3 -> 2 -> 1 -> 0）。
如果 Alice 一开始拿走 1 个石子， Bob 会拿走 4 个石子，然后 Alice 只能拿走 1 个石子，Bob 拿走最后一个石子并赢得胜利（7 -> 6 -> 2 -> 1 -> 0）。
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> n = 17
<strong>输出:</strong> false
<strong>解释:</strong> 如果 Bob 采取最优策略，Alice 无法赢得胜利。
</pre>

#### 提示:
* <code>1 <= n <= 10<sup>5</sup></code>

## 题解 (Python)

### 1. 题解
```Python
from functools import cache


class Solution:
    squares = [1]

    @cache
    def winnerSquareGame(self, n: int) -> bool:
        while n > self.squares[-1]:
            self.squares.append((len(self.squares) + 1) ** 2)

        return any(not self.winnerSquareGame(n - x) for x in self.squares[::-1] if x <= n)
```
