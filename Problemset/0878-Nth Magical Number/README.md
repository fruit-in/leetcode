# 878. Nth Magical Number
A positive integer is *magical* if it is divisible by either `a` or `b`.

Given the three integers `n`, `a`, and `b`, return the <code>n<sup>th</sup></code> magical number. Since the answer may be very large, **return it modulo** <code>10<sup>9</sup> + 7</code>.

#### Example 1:
<pre>
<strong>Input:</strong> n = 1, a = 2, b = 3
<strong>Output:</strong> 2
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 4, a = 2, b = 3
<strong>Output:</strong> 6
</pre>

#### Constraints:
* <code>1 <= n <= 10<sup>9</sup></code>
* <code>2 <= a, b <= 4 * 10<sup>4</sup></code>

## Solutions (Python)

### 1. Solution
```Python
import math


class Solution:
    def nthMagicalNumber(self, n: int, a: int, b: int) -> int:
        c = math.lcm(a, b)
        d = n // (c // a + c // b - 1)
        n %= (c // a + c // b - 1)
        l = 0
        r = c - 1
        m = (l + r) // 2

        while m // a + m // b != n or (m % a != 0 and m % b != 0):
            if m // a + m // b < n:
                l = m + 1
            else:
                r = m - 1

            m = (l + r) // 2

        return (c * d + m) % 1000000007
```
