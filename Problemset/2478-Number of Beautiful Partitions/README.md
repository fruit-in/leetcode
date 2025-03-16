# 2478. Number of Beautiful Partitions
You are given a string `s` that consists of the digits `'1'` to `'9'` and two integers `k` and `minLength`.

A partition of `s` is called **beautiful** if:
* `s` is partitioned into `k` non-intersecting substrings.
* Each substring has a length of **at least** `minLength`.
* Each substring starts with a **prime** digit and ends with a **non-prime** digit. Prime digits are `'2'`, `'3'`, `'5'`, and `'7'`, and the rest of the digits are non-prime.

Return *the number of **beautiful** partitions of* `s`. Since the answer may be very large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

A **substring** is a contiguous sequence of characters within a string.

#### Example 1:
<pre>
<strong>Input:</strong> s = "23542185131", k = 3, minLength = 2
<strong>Output:</strong> 3
<strong>Explanation:</strong> There exists three ways to create a beautiful partition:
"2354 | 218 | 5131"
"2354 | 21851 | 31"
"2354218 | 51 | 31"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "23542185131", k = 3, minLength = 3
<strong>Output:</strong> 1
<strong>Explanation:</strong> There exists one way to create a beautiful partition: "2354 | 218 | 5131".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "3312958", k = 3, minLength = 1
<strong>Output:</strong> 1
<strong>Explanation:</strong> There exists one way to create a beautiful partition: "331 | 29 | 58".
</pre>

#### Constraints:
* `1 <= k, minLength <= s.length <= 1000`
* `s` consists of the digits `'1'` to `'9'`.

## Solutions (Python)

### 1. Solution
```Python
from functools import cache


class Solution:
    def beautifulPartitions(self, s: str, k: int, minLength: int) -> int:
        @cache
        def subPartitions(i: int, k: int) -> int:
            if k == 0:
                return i == len(s)
            if len(s) - i < k * minLength or s[i] not in "2357":
                return 0

            ret = 0

            for j in range(i + minLength, len(s) - (k - 1) * minLength + 1):
                if s[j - 1] not in "2357":
                    ret = (ret + subPartitions(j, k - 1)) % 1000000007

            return ret

        if s[-1] in "2357":
            return 0

        return subPartitions(0, k)
```
