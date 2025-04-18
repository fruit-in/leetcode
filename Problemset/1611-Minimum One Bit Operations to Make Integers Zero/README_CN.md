# 1611. 使整数变为 0 的最少操作次数
给你一个整数 `n`，你需要重复执行多次下述操作将其转换为 `0` ：
* 翻转 `n` 的二进制表示中最右侧位（第 `0` 位）。
* 如果第 `(i-1)` 位为 `1` 且从第 `(i-2)` 位到第 `0` 位都为 `0`，则翻转 `n` 的二进制表示中的第 `i` 位。

返回将 `n` 转换为 `0` 的最小操作次数。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 3
<strong>输出:</strong> 2
<strong>解释:</strong> 3 的二进制表示为 "11"
"11" -> "01" ，执行的是第 2 种操作，因为第 0 位为 1 。
"01" -> "00" ，执行的是第 1 种操作。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 6
<strong>输出:</strong> 4
<strong>解释:</strong> 6 的二进制表示为 "110".
"110" -> "010" ，执行的是第 2 种操作，因为第 1 位为 1 ，第 0 到 0 位为 0 。
"010" -> "011" ，执行的是第 1 种操作。
"011" -> "001" ，执行的是第 2 种操作，因为第 0 位为 1 。
"001" -> "000" ，执行的是第 1 种操作。
</pre>

#### 提示:
* <code>0 <= n <= 10<sup>9</sup></code>

## 题解 (Python)

### 1. 题解
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
