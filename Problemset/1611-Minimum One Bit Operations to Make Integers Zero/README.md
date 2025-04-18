# 1611. Minimum One Bit Operations to Make Integers Zero
Given an integer `n`, you must transform it into `0` using the following operations any number of times:
* Change the rightmost (<code>0<sup>th</sup></code>) bit in the binary representation of `n`.
* Change the <code>i<sup>th</sup></code> bit in the binary representation of `n` if the <code>(i-1)<sup>th</sup></code> bit is set to `1` and the <code>(i-2)<sup>th</sup></code> through <code>0<sup>th</sup></code> bits are set to `0`.

Return *the minimum number of operations to transform* `n` *into* `0`.

#### Example 1:
<pre>
<strong>Input:</strong> n = 3
<strong>Output:</strong> 2
<strong>Explanation:</strong> The binary representation of 3 is "11".
"11" -> "01" with the 2nd operation since the 0th bit is 1.
"01" -> "00" with the 1st operation.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 6
<strong>Output:</strong> 4
<strong>Explanation:</strong> The binary representation of 6 is "110".
"110" -> "010" with the 2nd operation since the 1st bit is 1 and 0th through 0th bits are 0.
"010" -> "011" with the 1st operation.
"011" -> "001" with the 2nd operation since the 0th bit is 1.
"001" -> "000" with the 1st operation.
</pre>

#### Constraints:
* <code>0 <= n <= 10<sup>9</sup></code>

## Solutions (Python)

### 1. Solution
```Python
from functools import cache


class Solution:
    @cache
    def minimumOneZerosOperations(self, n: int, i: int) -> int:
        if i == 0:
            return 1 - n

        if (n >> i) & 1 == 1:
            return self.minimumOneBitOperations(n & ((1 << i) - 1))
        else:
            return 1 + self.minimumOneZerosOperations(n, i - 1) + self.minimumOneBitOperations(1 << (i - 1))

    @cache
    def minimumOneBitOperations(self, n: int) -> int:
        if n <= 1:
            return n

        for i in range(29, 0, -1):
            if (n >> i) & 1 == 1:
                return 1 + self.minimumOneZerosOperations(n & ((1 << i) - 1), i - 1) + self.minimumOneBitOperations(1 << (i - 1))
```
