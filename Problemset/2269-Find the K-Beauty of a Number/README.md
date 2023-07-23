# 2269. Find the K-Beauty of a Number
The **k-beauty** of an integer `num` is defined as the number of **substrings** of `num` when it is read as a string that meet the following conditions:
* It has a length of `k`.
* It is a divisor of `num`.

Given integers `num` and `k`, return *the k-beauty of* `num`.

Note:
* **Leading zeros** are allowed.
* `0` is not a divisor of any value.

A **substring** is a contiguous sequence of characters in a string.

#### Example 1:
<pre>
<strong>Input:</strong> num = 240, k = 2
<strong>Output:</strong> 2
<strong>Explanation:</strong> The following are the substrings of num of length k:
- "24" from "240": 24 is a divisor of 240.
- "40" from "240": 40 is a divisor of 240.
Therefore, the k-beauty is 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> num = 430043, k = 2
<strong>Output:</strong> 2
<strong>Explanation:</strong> The following are the substrings of num of length k:
- "43" from "430043": 43 is a divisor of 430043.
- "30" from "430043": 30 is not a divisor of 430043.
- "00" from "430043": 0 is not a divisor of 430043.
- "04" from "430043": 4 is not a divisor of 430043.
- "43" from "430043": 43 is a divisor of 430043.
Therefore, the k-beauty is 2.
</pre>

#### Constraints:
* <code>1 <= num <= 10<sup>9</sup></code>
* `1 <= k <= num.length` (taking `num` as a string)

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def divisorSubstrings(self, num: int, k: int) -> int:
        s = str(num)
        ret = 0

        for i in range(len(s) - k + 1):
            x = int(s[i:i + k])

            if x > 0 and num % x == 0:
                ret += 1

        return ret
```
