# 2457. Minimum Addition to Make Integer Beautiful
You are given two positive integers `n` and `target`.

An integer is considered **beautiful** if the sum of its digits is less than or equal to `target`.

Return the *minimum **non-negative** integer* `x` *such that* `n + x` *is beautiful*. The input will be generated such that it is always possible to make `n` beautiful.

#### Example 1:
<pre>
<strong>Input:</strong> n = 16, target = 6
<strong>Output:</strong> 4
<strong>Explanation:</strong> Initially n is 16 and its digit sum is 1 + 6 = 7. After adding 4, n becomes 20 and digit sum becomes 2 + 0 = 2. It can be shown that we can not make n beautiful with adding non-negative integer less than 4.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 467, target = 6
<strong>Output:</strong> 33
<strong>Explanation:</strong> Initially n is 467 and its digit sum is 4 + 6 + 7 = 17. After adding 33, n becomes 500 and digit sum becomes 5 + 0 + 0 = 5. It can be shown that we can not make n beautiful with adding non-negative integer less than 33.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 1, target = 1
<strong>Output:</strong> 0
<strong>Explanation:</strong> Initially n is 1 and its digit sum is 1, which is already smaller than or equal to target.
</pre>

#### Constraints:
* <code>1 <= n <= 10<sup>12</sup></code>
* `1 <= target <= 150`
* The input will be generated such that it is always possible to make `n` beautiful.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def makeIntegerBeautiful(self, n: int, target: int) -> int:
        a = 1
        m = n

        while sum(int(ch) for ch in str(m)) > target:
            m += (10 - m // a % 10) * a
            a *= 10

        return m - n
```
