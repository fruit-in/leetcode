# 1553. Minimum Number of Days to Eat N Oranges
There are `n` oranges in the kitchen and you decided to eat some of these oranges every day as follows:
* Eat one orange.
* If the number of remaining oranges `n` is divisible by `2` then you can eat `n / 2` oranges.
* If the number of remaining oranges `n` is divisible by `3` then you can eat `2 * (n / 3)` oranges.

You can only choose one of the actions per day.

Given the integer `n`, return *the minimum number of days to eat* `n` *oranges*.

#### Example 1:
<pre>
<strong>Input:</strong> n = 10
<strong>Output:</strong> 4
<strong>Explanation:</strong> You have 10 oranges.
Day 1: Eat 1 orange,  10 - 1 = 9.
Day 2: Eat 6 oranges, 9 - 2*(9/3) = 9 - 6 = 3. (Since 9 is divisible by 3)
Day 3: Eat 2 oranges, 3 - 2*(3/3) = 3 - 2 = 1.
Day 4: Eat the last orange  1 - 1  = 0.
You need at least 4 days to eat the 10 oranges.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 6
<strong>Output:</strong> 3
<strong>Explanation:</strong> You have 6 oranges.
Day 1: Eat 3 oranges, 6 - 6/2 = 6 - 3 = 3. (Since 6 is divisible by 2).
Day 2: Eat 2 oranges, 3 - 2*(3/3) = 3 - 2 = 1. (Since 3 is divisible by 3)
Day 3: Eat the last orange  1 - 1  = 0.
You need at least 3 days to eat the 6 oranges.
</pre>

#### Constraints:
* <code>1 <= n <= 2 * 10<sup>9</sup></code>

## Solutions (Python)

### 1. Solution
```Python
from functools import cache


class Solution:
    @cache
    def minDays(self, n: int) -> int:
        if n <= 1:
            return n

        return 1 + min(self.minDays(n // 2) + n % 2, self.minDays(n // 3) + n % 3)
```
