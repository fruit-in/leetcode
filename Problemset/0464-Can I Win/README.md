# 464. Can I Win
In the "100 game" two players take turns adding, to a running total, any integer from `1` to `10`. The player who first causes the running total to **reach or exceed** 100 wins.

What if we change the game so that players **cannot** re-use integers?

For example, two players might take turns drawing from a common pool of numbers from 1 to 15 without replacement until they reach a total >= 100.

Given two integers `maxChoosableInteger` and `desiredTotal`, return `true` if the first player to move can force a win, otherwise, return `false`. Assume both players play **optimally**.

#### Example 1:
<pre>
<strong>Input:</strong> maxChoosableInteger = 10, desiredTotal = 11
<strong>Output:</strong> false
<strong>Explanation:</strong>
No matter which integer the first player choose, the first player will lose.
The first player can choose an integer from 1 up to 10.
If the first player choose 1, the second player can only choose integers from 2 up to 10.
The second player will win by choosing 10 and get a total = 11, which is >= desiredTotal.
Same with other integers chosen by the first player, the second player will always win.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> maxChoosableInteger = 10, desiredTotal = 0
<strong>Output:</strong> true
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> maxChoosableInteger = 10, desiredTotal = 1
<strong>Output:</strong> true
</pre>

#### Constraints:
* `1 <= maxChoosableInteger <= 20`
* `0 <= desiredTotal <= 300`

## Solutions (Python)

### 1. Solution
```Python
from functools import cache


class Solution:
    def canIWin(self, maxChoosableInteger: int, desiredTotal: int) -> bool:
        @cache
        def canIWinWithUsed(usedmask: int) -> bool:
            total = sum(i + 1 for i in range(maxChoosableInteger)
                        if (usedmask >> i) & 1 == 1)

            for i in range(maxChoosableInteger):
                if (usedmask >> i) & 1 == 0:
                    if total + i + 1 >= desiredTotal or not canIWinWithUsed(usedmask | (1 << i)):
                        return True

            return False

        return sum(range(1, maxChoosableInteger + 1)) >= desiredTotal and canIWinWithUsed(0)
```
