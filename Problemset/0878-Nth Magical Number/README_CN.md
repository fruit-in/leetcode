# 878. 第 N 个神奇数字
一个正整数如果能被 `a` 或 `b` 整除，那么它是神奇的。

给定三个整数 `n` , `a` , `b` ，返回第 `n` 个神奇的数字。因为答案可能很大，所以返回答案 对 <code>10<sup>9</sup> + 7</code> **取模** 后的值。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 1, a = 2, b = 3
<strong>输出:</strong> 2
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 4, a = 2, b = 3
<strong>输出:</strong> 6
</pre>

#### 提示:
* <code>1 <= n <= 10<sup>9</sup></code>
* <code>2 <= a, b <= 4 * 10<sup>4</sup></code>

## 题解 (Python)

### 1. 题解
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
