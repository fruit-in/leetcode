# 483. Smallest Good Base
Given an integer `n` represented as a string, return *the smallest **good base** of* `n`.

We call `k >= 2` a **good base** of `n`, if all digits of `n` base `k` are `1`'s.

#### Example 1:
<pre>
<strong>Input:</strong> n = "13"
<strong>Output:</strong> "3"
<strong>Explanation:</strong> 13 base 3 is 111.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = "4681"
<strong>Output:</strong> "8"
<strong>Explanation:</strong> 4681 base 8 is 11111.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = "1000000000000000000"
<strong>Output:</strong> "999999999999999999"
<strong>Explanation:</strong> 1000000000000000000 base 999999999999999999 is 11.
</pre>

#### Constraints:
* `n` is an integer in the range <code>[3, 10<sup>18</sup>]</code>.
* `n` does not contain any leading zeros.

## Solutions (Python)

### 1. Solution
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
