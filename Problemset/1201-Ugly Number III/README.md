# 1201. Ugly Number III
An **ugly number** is a positive integer that is divisible by `a`, `b`, or `c`.

Given four integers `n`, `a`, `b`, and `c`, return the <code>n<sup>th</sup></code> **ugly number**.

#### Example 1:
<pre>
<strong>Input:</strong> n = 3, a = 2, b = 3, c = 5
<strong>Output:</strong> 4
<strong>Explanation:</strong> The ugly numbers are 2, 3, 4, 5, 6, 8, 9, 10... The 3rd is 4.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 4, a = 2, b = 3, c = 4
<strong>Output:</strong> 6
<strong>Explanation:</strong> The ugly numbers are 2, 3, 4, 6, 8, 9, 10, 12... The 4th is 6.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 5, a = 2, b = 11, c = 13
<strong>Output:</strong> 10
<strong>Explanation:</strong> The ugly numbers are 2, 4, 6, 8, 10, 11, 12, 13... The 5th is 10.
</pre>

#### Constraints:
* <code>1 <= n, a, b, c <= 10<sup>9</sup></code>
* <code>1 <= a * b * c <= 10<sup>18</sup></code>
* It is guaranteed that the result will be in range <code>[1, 2 * 10<sup>9</sup>]</code>.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def nthUglyNumber(self, n: int, a: int, b: int, c: int) -> int:
        def leUglyCount(x: int) -> int:
            return x // a + x // b + x // c - x // d - x // e - x // f + x // g

        d, e, f, g = math.lcm(a, b), math.lcm(
            b, c),  math.lcm(a, c), math.lcm(a, b, c)

        return bisect.bisect_left(range(2000000000), n, key=leUglyCount)
```
