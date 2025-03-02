# 679. 24 Game
You are given an integer array `cards` of length `4`. You have four cards, each containing a number in the range `[1, 9]`. You should arrange the numbers on these cards in a mathematical expression using the operators `['+', '-', '*', '/']` and the parentheses `'('` and `')'` to get the value 24.

You are restricted with the following rules:
* The division operator `'/'` represents real division, not integer division.
    * For example, `4 / (1 - 2 / 3) = 4 / (1 / 3) = 12`.
* Every operation done is between two numbers. In particular, we cannot use `'-'` as a unary operator.
    * For example, if `cards = [1, 1, 1, 1]`, the expression `"-1 - 1 - 1 - 1"` is **not allowed**.
* You cannot concatenate numbers together
    * For example, if `cards = [1, 2, 1, 2]`, the expression `"12 + 12"` is not valid.

Return `true` if you can get such expression that evaluates to `24`, and `false` otherwise.

#### Example 1:
<pre>
<strong>Input:</strong> cards = [4,1,8,7]
<strong>Output:</strong> true
<strong>Explanation:</strong> (8-4) * (7-1) = 24
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> cards = [1,2,1,2]
<strong>Output:</strong> false
</pre>

#### Constraints:
* `cards.length == 4`
* `1 <= cards[i] <= 9`

## Solutions (Python)

### 1. Solution
```Python
from functools import cache
from itertools import permutations
from math import gcd


class Solution:
    @cache
    def add(self, a: (int, int), b: (int, int)) -> (int, int):
        lcm = abs(a[1] * b[1]) // gcd(a[1], b[1])
        return (a[0] * lcm // a[1] + b[0] * lcm // b[1], lcm)

    @cache
    def sub(self, a: (int, int), b: (int, int)) -> (int, int):
        return self.add(a, (-b[0], b[1]))

    @cache
    def mul(self, a: (int, int), b: (int, int)) -> (int, int):
        return (a[0] * b[0], a[1] * b[1])

    @cache
    def div(self, a: (int, int), b: (int, int)) -> (int, int):
        if b[0] == 0:
            return (0, 1)
        return self.mul(a, (b[1], b[0]))

    @cache
    def judge1(self, a: (int, int)) -> bool:
        return a[0] % a[1] == 0 and a[0] // a[1] == 24

    @cache
    def judge2(self, a: (int, int), b: (int, int)) -> bool:
        return self.judge1(self.add(a, b)) or self.judge1(self.sub(a, b)) or self.judge1(self.mul(a, b)) or self.judge1(self.div(a, b))

    @cache
    def judge3(self, a: (int, int), b: (int, int), c: (int, int)) -> bool:
        return self.judge2(self.add(a, b), c) or self.judge2(self.sub(a, b), c) or self.judge2(self.mul(a, b), c) or self.judge2(self.div(a, b), c) or \
            self.judge2(a, self.add(b, c)) or self.judge2(a, self.sub(b, c)) or self.judge2(
                a, self.mul(b, c)) or self.judge2(a, self.div(b, c))

    @cache
    def judge4(self, a: (int, int), b: (int, int), c: (int, int), d: (int, int)) -> bool:
        return self.judge3(self.add(a, b), c, d) or self.judge3(self.sub(a, b), c, d) or self.judge3(self.mul(a, b), c, d) or self.judge3(self.div(a, b), c, d) or \
            self.judge3(a, self.add(b, c), d) or self.judge3(a, self.sub(b, c), d) or self.judge3(a, self.mul(b, c), d) or self.judge3(a, self.div(b, c), d) or \
            self.judge3(a, b, self.add(c, d)) or self.judge3(a, b, self.sub(
                c, d)) or self.judge3(a, b, self.mul(c, d)) or self.judge3(a, b, self.div(c, d))

    def judgePoint24(self, cards: List[int]) -> bool:
        for (a, b, c, d) in permutations(cards):
            if self.judge4((a, 1), (b, 1), (c, 1), (d, 1)):
                return True
        return False
```
