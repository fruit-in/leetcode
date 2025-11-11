# 600. Non-negative Integers without Consecutive Ones
Given a positive integer `n`, return the number of the integers in the range `[0, n]` whose binary representations **do not** contain consecutive ones.

#### Example 1:
<pre>
<strong>Input:</strong> n = 5
<strong>Output:</strong> 5
<strong>Explanation:</strong>
Here are the non-negative integers <= 5 with their corresponding binary representations:
0 : 0
1 : 1
2 : 10
3 : 11
4 : 100
5 : 101
Among them, only integer 3 disobeys the rule (two consecutive ones) and the other 5 satisfy the rule.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 1
<strong>Output:</strong> 2
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 2
<strong>Output:</strong> 3
</pre>

#### Constraints:
* <code>1 <= n <= 10<sup>9</sup></code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    @cache
    def findIntegers(self, n: int) -> int:
        if n < 4:
            return 3 if n == 3 else n + 1

        m = n.bit_length() - 2
        ret = self.findIntegers((1 << m) - 1) + \
            self.findIntegers((1 << (m - 1)) - 1)
        if n >> m == 0b10:
            ret += self.findIntegers(n & ((1 << m) - 1))
        else:
            ret += self.findIntegers((1 << m) - 1)

        return ret
```
