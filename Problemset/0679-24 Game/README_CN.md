# 679. 24 点游戏
给定一个长度为4的整数数组 `cards` 。你有 `4` 张卡片，每张卡片上都包含一个范围在 `[1,9]` 的数字。您应该使用运算符 `['+', '-', '*', '/']` 和括号 `'('` 和 `')'` 将这些卡片上的数字排列成数学表达式，以获得值24。

你须遵守以下规则:
* 除法运算符 `'/'` 表示实数除法，而不是整数除法。
    * 例如， `4 /(1 - 2 / 3)= 4 /(1 / 3)= 12` 。
* 每个运算都在两个数字之间。特别是，不能使用 `“-”` 作为一元运算符。
    * 例如，如果 `cards =[1,1,1,1]` ，则表达式 `“-1 -1 -1 -1”` 是 **不允许** 的。
* 你不能把数字串在一起
    * 例如，如果 `cards =[1,2,1,2]` ，则表达式 `“12 + 12”` 无效。

如果可以得到这样的表达式，其计算结果为 `24` ，则返回 `true` ，否则返回 `false` 。

#### 示例 1:
<pre>
<strong>输入:</strong> cards = [4,1,8,7]
<strong>输出:</strong> true
<strong>解释:</strong> (8-4) * (7-1) = 24
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> cards = [1,2,1,2]
<strong>输出:</strong> false
</pre>

#### 提示:
* `cards.length == 4`
* `1 <= cards[i] <= 9`

## 题解 (Python)

### 1. 题解
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
