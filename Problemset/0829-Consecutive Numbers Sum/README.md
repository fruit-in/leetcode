# 829. Consecutive Numbers Sum
Given an integer `n`, return *the number of ways you can write* `n` *as the sum of consecutive positive integers*.

#### Example 1:
<pre>
<strong>Input:</strong> n = 5
<strong>Output:</strong> 2
<strong>Explanation:</strong> 5 = 2 + 3
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 9
<strong>Output:</strong> 3
<strong>Explanation:</strong> 9 = 4 + 5 = 2 + 3 + 4
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 15
<strong>Output:</strong> 4
<strong>Explanation:</strong> 15 = 8 + 7 = 4 + 5 + 6 = 1 + 2 + 3 + 4 + 5
</pre>

#### Constraints:
* <code>1 <= n <= 10<sup>9</sup></code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def consecutiveNumbersSum(self, n: int) -> int:
        ret = 0

        for x in range(1, int((2 * n) ** .5) + 1):
            if (n + (x - 1) * x // 2) % x == 0 and (n + (x - 1) * x // 2) // x > 0:
                ret += 1

        return ret
```
