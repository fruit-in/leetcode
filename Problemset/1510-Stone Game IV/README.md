# 1510. Stone Game IV
Alice and Bob take turns playing a game, with Alice starting first.

Initially, there are `n` stones in a pile. On each player's turn, that player makes a *move* consisting of removing **any** non-zero **square number** of stones in the pile.

Also, if a player cannot make a move, he/she loses the game.

Given a positive integer `n`, return `true` if and only if Alice wins the game otherwise return `false`, assuming both players play optimally.

#### Example 1:
<pre>
<strong>Input:</strong> n = 1
<strong>Output:</strong> true
<strong>Explanation:</strong> Alice can remove 1 stone winning the game because Bob doesn't have any moves.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 2
<strong>Output:</strong> false
<strong>Explanation:</strong> Alice can only remove 1 stone, after that Bob removes the last one winning the game (2 -> 1 -> 0).
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 4
<strong>Output:</strong> true
<strong>Explanation:</strong> n is already a perfect square, Alice can win with one move, removing 4 stones (4 -> 0).
</pre>

#### Constraints:
* <code>1 <= n <= 10<sup>5</sup></code>

## Solutions (Python)

### 1. Solution
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
