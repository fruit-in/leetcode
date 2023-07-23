# 483. 最小好进制
以字符串的形式给出 `n` , 以字符串的形式返回 *`n` 的最小 **好进制***  。

如果 `n` 的  `k(k>=2)` 进制数的所有数位全为1，则称 `k(k>=2)` 是 `n` 的一个 **好进制** 。

#### 示例 1:
<pre>
<strong>输入:</strong> n = "13"
<strong>输出:</strong> "3"
<strong>解释:</strong> 13 的 3 进制是 111。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = "4681"
<strong>输出:</strong> "8"
<strong>解释:</strong> 4681 的 8 进制是 11111。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = "1000000000000000000"
<strong>输出:</strong> "999999999999999999"
<strong>解释:</strong> 1000000000000000000 的 999999999999999999 进制是 11。
</pre>

#### 提示:
* `n` 的取值范围是 <code>[3, 10<sup>18</sup>]</code>
* `n` 没有前导 0

## 题解 (Python)

### 1. 题解
```Python
import math


class Solution:
    def smallestGoodBase(self, n: str) -> str:
        n = int(n)

        for x in range(math.ceil(math.log2(n)), 1, -1):
            l, r = 2, n - 1

            while l <= r:
                k = (l + r) // 2

                if k ** x - 1 == n * (k - 1):
                    return str(k)
                elif k ** x - 1 < n * (k - 1):
                    l = k + 1
                else:
                    r = k - 1
```
